/// AI chat integration.

mod gemini;
mod openai;

use crate::types::AiChatRequest;

#[tauri::command]
pub async fn ai_chat(req: AiChatRequest) -> Result<String, String> {
    let fut = async {
        match req.provider.as_str() {
            "gemini" => gemini::gemini_chat(req).await,
            "openai" => openai::openai_chat(req).await,
            other => Err(format!("Unknown provider: {other}")),
        }
    };

    tokio::time::timeout(std::time::Duration::from_secs(30), fut)
        .await
        .unwrap_or_else(|_| Err("Request timed out after 30 seconds.".into()))
}
