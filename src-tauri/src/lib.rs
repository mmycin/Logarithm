use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub line: usize,
    pub datetime: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterParams {
    pub status: String,
    pub custom_status: String,
    pub query: String,
    pub match_case: bool,
    pub fuzzy: bool,
    pub from_datetime: String,
    pub to_datetime: String,
}

#[derive(Debug, Deserialize)]
pub struct AiChatRequest {
    pub provider: String,   // "gemini" | "openai"
    pub api_key: String,
    pub model: String,
    pub messages: Vec<AiMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiMessage {
    pub role: String,    // "user" | "assistant" | "system"
    pub content: String,
}

mod commands {
    use super::{AiChatRequest, AiMessage, FilterParams, LogEntry};

    #[tauri::command]
    pub fn parse_log(text: String) -> Vec<LogEntry> {
        const STATUSES: [&str; 8] = [
            "TRACE", "DEBUG", "INFO", "WARN", "WARNING", "ERROR", "SUCCESS", "FATAL",
        ];
        text.lines()
            .enumerate()
            .filter_map(|(line_idx, line)| {
                let line = line.trim();
                if line.is_empty() { return None; }
                let tokens: Vec<&str> = line.split_whitespace().collect();
                let mut datetime = String::new();
                let mut start_idx = 0usize;
                if let Some(first) = tokens.first().copied() {
                    if let Some((d, t)) = first.split_once('T') {
                        if !d.is_empty() && !t.is_empty() {
                            datetime = format!("{} {}", d, t);
                            start_idx = 1;
                        }
                    } else if tokens.len() >= 2 {
                        let d = tokens[0]; let t = tokens[1];
                        if d.contains('-') && t.contains(':') {
                            datetime = format!("{} {}", d, t);
                            start_idx = 2;
                        }
                    }
                }
                let mut status_idx: Option<usize> = None;
                let mut status = String::new();
                for (i, tok) in tokens.iter().enumerate().skip(start_idx) {
                    let cleaned = tok.trim_matches(|c: char| !c.is_alphanumeric()).to_ascii_uppercase();
                    if STATUSES.iter().any(|s| *s == cleaned) {
                        status_idx = Some(i); status = cleaned; break;
                    }
                }
                let message = match status_idx {
                    Some(si) => tokens.iter().enumerate()
                        .filter_map(|(i, t)| if i == si || i < start_idx { None } else { Some(*t) })
                        .collect::<Vec<_>>().join(" "),
                    None => tokens.iter().skip(start_idx).copied().collect::<Vec<_>>().join(" "),
                };
                Some(LogEntry { line: line_idx + 1, datetime, status, message })
            })
            .collect()
    }

    #[tauri::command]
    pub fn filter_entries(entries: Vec<LogEntry>, params: FilterParams) -> Vec<LogEntry> {
        let from_cmp = params.from_datetime.replace('T', " ");
        let to_cmp   = params.to_datetime.replace('T', " ");
        let sf = params.status.to_ascii_lowercase();
        let cf = params.custom_status.trim().to_ascii_uppercase();
        entries.into_iter().filter(|entry| {
            if sf != "all" {
                let es = entry.status.to_ascii_uppercase();
                let ok = if sf == "custom" {
                    cf.is_empty() || cf.split(',').map(|s| s.trim()).any(|s| es == s)
                } else if sf == "warn" { es == "WARN" || es == "WARNING" }
                else { es == sf.to_ascii_uppercase() };
                if !ok { return false; }
            }
            if !params.query.is_empty() {
                let hay = format!("{} {} {}", entry.datetime, entry.status, entry.message);
                let matched = if params.fuzzy {
                    let (h, q) = if params.match_case { (hay.clone(), params.query.clone()) }
                                 else { (hay.to_ascii_lowercase(), params.query.to_ascii_lowercase()) };
                    let mut hi = h.chars();
                    q.chars().all(|c| hi.any(|hc| hc == c))
                } else if params.match_case { hay.contains(&params.query) }
                else { hay.to_ascii_lowercase().contains(&params.query.to_ascii_lowercase()) };
                if !matched { return false; }
            }
            if !from_cmp.is_empty() && !entry.datetime.is_empty() && entry.datetime < from_cmp { return false; }
            if !to_cmp.is_empty() && !entry.datetime.is_empty() {
                let ep = &entry.datetime[..entry.datetime.len().min(to_cmp.len())];
                if ep > to_cmp.as_str() { return false; }
            }
            true
        }).collect()
    }

    /// Open a URL in the system default browser.
    #[tauri::command]
    pub async fn open_url(url: String, _app: tauri::AppHandle) -> Result<(), String> {
        tauri_plugin_opener::open_url(url, None::<&str>)
            .map_err(|e| e.to_string())
    }

    /// Read a file by path and return its contents.
    #[tauri::command]
    pub async fn read_file_by_path(path: String) -> Result<String, String> {
        std::fs::read_to_string(&path)
            .map_err(|e| format!("Failed to read file {}: {}", path, e))
    }

    /// Send a chat message to Gemini or OpenAI and return the assistant reply.
    #[tauri::command]
    pub async fn ai_chat(req: AiChatRequest) -> Result<String, String> {
        // Hard timeout: 30 s — prevents infinite hang
        let fut = async {
            match req.provider.as_str() {
                "gemini" => gemini_chat(req).await,
                "openai" => openai_chat(req).await,
                other    => Err(format!("Unknown provider: {other}")),
            }
        };
        tokio::time::timeout(std::time::Duration::from_secs(30), fut)
            .await
            .unwrap_or_else(|_| Err("Request timed out after 30 seconds.".into()))
    }

    // ── Gemini ────────────────────────────────────────────────────────────

    async fn gemini_chat(req: AiChatRequest) -> Result<String, String> {
        let model = if req.model.is_empty() { "gemini-1.5-flash".to_string() } else { req.model.clone() };
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            model, req.api_key
        );

        // Gemini requires strictly alternating user/model turns.
        // Extract the system prompt (first message with role "system") and prepend
        // its text to the first real user message so we never send two user turns in a row.
        let mut system_text = String::new();
        let mut chat_msgs: Vec<&AiMessage> = Vec::new();
        for msg in &req.messages {
            if msg.role == "system" {
                system_text = msg.content.clone();
            } else {
                chat_msgs.push(msg);
            }
        }

        let mut contents: Vec<serde_json::Value> = Vec::new();
        for (i, msg) in chat_msgs.iter().enumerate() {
            let role = if msg.role == "assistant" { "model" } else { "user" };
            // Prepend system text to the very first user message
            let text = if i == 0 && !system_text.is_empty() && role == "user" {
                format!("{}\n\n{}", system_text, msg.content)
            } else {
                msg.content.clone()
            };
            contents.push(serde_json::json!({
                "role": role,
                "parts": [{ "text": text }]
            }));
        }

        // Gemini needs at least one message
        if contents.is_empty() {
            return Err("No messages to send.".into());
        }

        let body = serde_json::json!({ "contents": contents });

        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .map_err(|e| format!("Client build error: {e}"))?;

        let resp = client
            .post(&url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Network error: {e}"))?;

        let status = resp.status();
        let text = resp.text().await.map_err(|e| format!("Read error: {e}"))?;

        if !status.is_success() {
            return Err(format!("Gemini API error {status}: {text}"));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("JSON parse error: {e}"))?;

        json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Unexpected Gemini response: {text}"))
    }

    // ── OpenAI ────────────────────────────────────────────────────────────

    async fn openai_chat(req: AiChatRequest) -> Result<String, String> {
        let model = if req.model.is_empty() { "gpt-4o-mini".to_string() } else { req.model.clone() };

        let messages: Vec<serde_json::Value> = req.messages.iter().map(|m| {
            serde_json::json!({ "role": m.role, "content": m.content })
        }).collect();

        let body = serde_json::json!({
            "model": model,
            "messages": messages,
            "max_tokens": 2048
        });

        let client = reqwest::Client::builder()
            .use_rustls_tls()
            .build()
            .map_err(|e| format!("Client build error: {e}"))?;

        let resp = client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", req.api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("Network error: {e}"))?;

        let status = resp.status();
        let text = resp.text().await.map_err(|e| format!("Read error: {e}"))?;

        if !status.is_success() {
            return Err(format!("OpenAI API error {status}: {text}"));
        }

        let json: serde_json::Value = serde_json::from_str(&text)
            .map_err(|e| format!("JSON parse error: {e}"))?;

        json["choices"][0]["message"]["content"]
            .as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| format!("Unexpected OpenAI response: {text}"))
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::parse_log,
            commands::filter_entries,
            commands::open_url,
            commands::ai_chat,
            commands::read_file_by_path,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
