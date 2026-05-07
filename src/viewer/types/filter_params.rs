/// Filter parameters for log entries.
/// 
/// Contains all filter settings used to filter log entries.

#[derive(Clone)]
pub struct FilterParams {
    /// Selected status level (e.g., "all", "info", "error")
    pub status: String,
    
    /// Custom status levels (comma-separated)
    pub custom_status: String,
    
    /// Whether continuation lines inherit parent level
    pub inherit_level: bool,
    
    /// Search query string
    pub query: String,
    
    /// Case-sensitive search
    pub match_case: bool,
    
    /// Fuzzy search mode
    pub fuzzy: bool,
    
    /// Regex search mode
    #[allow(dead_code)]
    pub regex_mode: bool,
    
    /// Invert search match
    pub invert_match: bool,
    
    /// Search in datetime field
    pub search_in_datetime: bool,
    
    /// From datetime filter
    pub from_datetime: String,
    
    /// To datetime filter
    pub to_datetime: String,
    
    /// Line number from
    pub line_from: usize,
    
    /// Line number to
    pub line_to: usize,
    
    /// Hide lines with no level
    pub hide_no_level: bool,
    
    /// Minimum severity level
    pub min_severity: String,
}
