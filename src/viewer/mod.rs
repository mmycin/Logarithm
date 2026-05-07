/// File viewer module.
/// 
/// Contains the log file viewer with filtering, rendering, and context menu.

pub mod components;
pub mod filters;
pub mod rendering;
pub mod types;

// Re-export the main component
pub use components::FileViewer;
