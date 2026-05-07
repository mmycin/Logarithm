/// A log file with its parsed entries.
/// 
/// Represents an opened log file containing the file path
/// and all parsed log entries.

use super::LogEntry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LogFile {
    /// Full file path or name
    pub name: String,
    
    /// All parsed log entries from this file
    pub entries: Vec<LogEntry>,
}

impl LogFile {
    /// Create a new LogFile
    pub fn new(name: String, entries: Vec<LogEntry>) -> Self {
        Self { name, entries }
    }

    /// Get the short filename (last path component)
    pub fn short_name(&self) -> &str {
        self.name
            .rsplit('/')
            .next()
            .or_else(|| self.name.rsplit('\\').next())
            .unwrap_or(&self.name)
    }

    /// Get the total number of entries
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}
