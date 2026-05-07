/// Browser localStorage utilities.
/// 
/// Provides safe wrappers around browser localStorage API
/// for persisting application state.

mod local_storage;

pub use local_storage::{get_item, set_item, remove_item};
