/// A chat message in the conversation history.
/// 
/// Represents a single message from user, assistant, or error.

#[derive(Clone, PartialEq)]
pub struct ChatMessage {
    /// Message role: "user", "assistant", or "error"
    pub role: String,
    
    /// Message content (may contain markdown)
    pub content: String,
}

impl ChatMessage {
    /// Create a new user message
    pub fn user(content: String) -> Self {
        Self {
            role: "user".into(),
            content,
        }
    }

    /// Create a new assistant message
    pub fn assistant(content: String) -> Self {
        Self {
            role: "assistant".into(),
            content,
        }
    }

    /// Create a new error message
    pub fn error(content: String) -> Self {
        Self {
            role: "error".into(),
            content,
        }
    }

    /// Check if this is a user message
    pub fn is_user(&self) -> bool {
        self.role == "user"
    }

    /// Check if this is an assistant message
    pub fn is_assistant(&self) -> bool {
        self.role == "assistant"
    }

    /// Check if this is an error message
    pub fn is_error(&self) -> bool {
        self.role == "error"
    }
}
