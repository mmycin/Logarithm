/// Type definitions for the AI panel.
/// 
/// Contains provider configuration, chat messages, and other
/// AI-related data structures.

mod chat_message;
mod provider;

pub use chat_message::ChatMessage;
pub use provider::Provider;
