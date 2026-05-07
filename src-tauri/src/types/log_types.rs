/// Log entry and filter types.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub line: usize,
    pub datetime: String,
    pub status: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct FilterParams {
    pub status: String,
    pub custom_status: String,
    pub query: String,
    pub match_case: bool,
    pub fuzzy: bool,
    pub from_datetime: String,
    pub to_datetime: String,
}
