/// Group assignment for log entries.
/// 
/// Assigns group line numbers to log entries for display.
/// A "group" is a levelled line + all following no-level lines.

use crate::shared::types::LogEntry;
use crate::viewer::types::DisplayEntry;

/// Assign group line numbers to entries.
/// 
/// Each entry with a status starts a new group. All continuation lines
/// (no status) belong to the most recent group and share its line number.
pub fn assign_groups(entries: Vec<LogEntry>) -> Vec<DisplayEntry> {
    let mut result = Vec::with_capacity(entries.len());
    let mut group_line = 0usize;

    for entry in entries {
        if !entry.status.is_empty() {
            // This is a group leader
            group_line = entry.line;
            result.push(DisplayEntry::new(entry, group_line, false));
        } else {
            // This is a continuation line
            result.push(DisplayEntry::new(entry, group_line, true));
        }
    }

    result
}
