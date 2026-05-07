/// OpenAI integration.

use crate::types::AiChatRequest;

pub async fn openai_chat(req: AiChatRequest) -> Result<String, String> {
    let model = if req.model.is_empty() {
        "gpt-4o-mini".to_string()
    } else {
        req.model.clone()
    };

    let messages: Vec<serde_json::Value> = req
        .messages
        .iter()
        .map(|m| serde_json::json!({ "role": m.role, "content": m.content }))
        .collect();

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
    let text = resp
        .text()
        .await
        .map_err(|e| format!("Read error: {e}"))?;

    if !status.is_success() {
        return Err(format!("OpenAI API error {status}: {text}"));
    }

    parse_openai_response(&text)
}

fn parse_openai_response(text: &str) -> Result<String, String> {
    let json: serde_json::Value =
        serde_json::from_str(text).map_err(|e| format!("JSON parse error: {e}"))?;

    json["choices"][0]["message"]["content"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| format!("Unexpected OpenAI response: {text}"))
}
