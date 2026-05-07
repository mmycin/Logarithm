/// Gemini AI integration.

use crate::types::{AiChatRequest, AiMessage};

pub async fn gemini_chat(req: AiChatRequest) -> Result<String, String> {
    let model = if req.model.is_empty() {
        "gemini-1.5-flash".to_string()
    } else {
        req.model.clone()
    };

    let url = format!(
        "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
        model, req.api_key
    );

    let contents = build_gemini_contents(&req.messages)?;
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
    let text = resp
        .text()
        .await
        .map_err(|e| format!("Read error: {e}"))?;

    if !status.is_success() {
        return Err(format!("Gemini API error {status}: {text}"));
    }

    parse_gemini_response(&text)
}

fn build_gemini_contents(messages: &[AiMessage]) -> Result<Vec<serde_json::Value>, String> {
    let mut system_text = String::new();
    let mut chat_msgs: Vec<&AiMessage> = Vec::new();

    for msg in messages {
        if msg.role == "system" {
            system_text = msg.content.clone();
        } else {
            chat_msgs.push(msg);
        }
    }

    if chat_msgs.is_empty() {
        return Err("No messages to send.".into());
    }

    let mut contents: Vec<serde_json::Value> = Vec::new();
    for (i, msg) in chat_msgs.iter().enumerate() {
        let role = if msg.role == "assistant" {
            "model"
        } else {
            "user"
        };

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

    Ok(contents)
}

fn parse_gemini_response(text: &str) -> Result<String, String> {
    let json: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("JSON parse error: {e}"))?;

    json["candidates"][0]["content"]["parts"][0]["text"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Unexpected Gemini response: {text}"))
}
