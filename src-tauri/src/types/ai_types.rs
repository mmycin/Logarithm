/// AI chat request and message types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct AiChatRequest {
    pub provider: String,
    pub api_key: String,
    pub model: String,
    pub messages: Vec<AiMessage>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiMessage {
    pub role: String,
    pub content: String,
}
