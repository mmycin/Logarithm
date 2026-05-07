/// A single log entry parsed from a log file.
/// 
/// Represents one line of a log file with structured fields for
/// line number, timestamp, status level, and message content.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LogEntry {
    /// Line number in the original file (1-indexed)
    pub line: usize,
    
    /// Timestamp string (e.g., "2024-01-15 10:30:45")
    pub datetime: String,
    
    /// Log level/status (e.g., "INFO", "ERROR", "WARN")
    /// Empty string for continuation lines
    pub status: String,
    
    /// The log message content
    pub message: String,
}

impl LogEntry {
    /// Check if this entry has a status level (not a continuation line)
    pub fn has_status(&self) -> bool {
        !self.status.is_empty()
    }

    /// Check if this is a continuation line (no status)
    pub fn is_continuation(&self) -> bool {
        self.status.is_empty()
    }
}
