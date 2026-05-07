/// Shared modules used across the application.
/// 
/// Contains types, utilities, constants, and storage helpers
/// that are used by multiple components.

pub mod constants;
pub mod storage;
pub mod types;
pub mod utils;

// Re-export commonly used items
pub use constants::{ColorTokens, DARK, LIGHT};
pub use storage::{get_item, remove_item, set_item};
pub use types::{LogEntry, LogFile, LoganAction, Theme};
pub use utils::invoke_tauri;
