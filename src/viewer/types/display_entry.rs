/// Display entry with group information.
/// 
/// Wraps a LogEntry with display metadata for rendering.
/// A "group" is a levelled line + all following no-level lines.
/// All lines in a group share the group's starting line number for display.

use crate::shared::types::LogEntry;

#[derive(Clone)]
pub struct DisplayEntry {
    /// The original log entry
    pub entry: LogEntry,
    
    /// Line number of the group leader (for display)
    pub group_line: usize,
    
    /// Whether this is a continuation line (no status)
    pub is_continuation: bool,
}

impl DisplayEntry {
    /// Create a new display entry
    pub fn new(entry: LogEntry, group_line: usize, is_continuation: bool) -> Self {
        Self {
            entry,
            group_line,
            is_continuation,
        }
    }
    
    /// Check if this is a group leader
    pub fn is_leader(&self) -> bool {
        !self.is_continuation
    }
}
