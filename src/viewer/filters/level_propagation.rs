/// Level propagation for continuation lines.
/// 
/// Propagates status levels to continuation lines (lines with no status).

use crate::shared::types::LogEntry;

/// Propagate level to continuation lines (lines with no status).
/// 
/// Each continuation line inherits the status from the most recent
/// line that has a status.
pub fn propagate_levels(entries: &[LogEntry]) -> Vec<LogEntry> {
    let mut out = Vec::with_capacity(entries.len());
    let mut current_status = String::new();
    
    for entry in entries {
        if !entry.status.is_empty() {
            // This line has a status - use it
            current_status = entry.status.clone();
            out.push(entry.clone());
        } else {
            // Continuation line - inherit current status
            let mut inherited = entry.clone();
            inherited.status = current_status.clone();
            out.push(inherited);
        }
    }
    
    out
}
