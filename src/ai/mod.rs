/// AI assistant panel module.
/// 
/// Contains the Logan AI chat interface with provider management,
/// message history, context chips, and file mentions.

pub mod components;
pub mod handlers;
pub mod state;
pub mod types;

// Re-export the main component
pub use components::AiPanel;
