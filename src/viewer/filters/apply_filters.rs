/// Apply all filters to log entries.
/// 
/// Filters log entries based on level, search, datetime, line range, and severity.

use crate::components::severity::severity_rank;
use crate::shared::types::LogEntry;
use crate::viewer::filters::propagate_levels;
use crate::viewer::types::FilterParams;

/// Apply all filters to the given log entries
pub fn apply_filters(entries: &[LogEntry], params: &FilterParams) -> Vec<LogEntry> {
    // First, optionally propagate levels to continuation lines
    let owned;
    let entries: &[LogEntry] = if params.inherit_level {
        owned = propagate_levels(entries);
        &owned
    } else {
        entries
    };

    // Prepare filter values
    let from_cmp = params.from_datetime.replace('T', " ");
    let to_cmp = params.to_datetime.replace('T', " ");
    let status_filter = params.status.to_ascii_lowercase();
    let custom_filter = params.custom_status.trim().to_ascii_uppercase();
    let query_lower = params.query.to_ascii_lowercase();
    let min_rank = severity_rank(&params.min_severity) as i32;

    // Apply all filters
    entries
        .iter()
        .filter(|entry| {
            // Level filter
            if !filter_by_level(entry, &status_filter, &custom_filter) {
                return false;
            }

            // Minimum severity filter
            if !filter_by_severity(entry, &params.min_severity, min_rank) {
                return false;
            }

            // Hide no-level filter
            if params.hide_no_level && entry.status.is_empty() {
                return false;
            }

            // Search filter
            if !filter_by_search(entry, params, &query_lower) {
                return false;
            }

            // DateTime filter
            if !filter_by_datetime(entry, &from_cmp, &to_cmp) {
                return false;
            }

            // Line range filter
            if !filter_by_line_range(entry, params.line_from, params.line_to) {
                return false;
            }

            true
        })
        .cloned()
        .collect()
}

/// Filter by status level
fn filter_by_level(entry: &LogEntry, status_filter: &str, custom_filter: &str) -> bool {
    if status_filter == "all" {
        return true;
    }

    let entry_status = entry.status.to_ascii_uppercase();

    if status_filter == "custom" {
        if custom_filter.is_empty() {
            return true;
        }
        return custom_filter
            .split(',')
            .map(|s| s.trim())
            .any(|s| entry_status == s);
    }

    if status_filter == "warn" {
        return entry_status == "WARN" || entry_status == "WARNING";
    }

    entry_status == status_filter.to_ascii_uppercase()
}

/// Filter by minimum severity
fn filter_by_severity(entry: &LogEntry, min_severity: &str, min_rank: i32) -> bool {
    if min_severity == "all" || entry.status.is_empty() {
        return true;
    }

    severity_rank(&entry.status) as i32 >= min_rank
}

/// Filter by search query
fn filter_by_search(entry: &LogEntry, params: &FilterParams, query_lower: &str) -> bool {
    if params.query.is_empty() {
        return true;
    }

    let haystack = if params.search_in_datetime {
        format!("{} {} {}", entry.datetime, entry.status, entry.message)
    } else {
        format!("{} {}", entry.status, entry.message)
    };

    let matched = if params.fuzzy {
        fuzzy_match(&haystack, &params.query, params.match_case, query_lower)
    } else if params.match_case {
        haystack.contains(&params.query)
    } else {
        haystack.to_ascii_lowercase().contains(query_lower)
    };

    if params.invert_match {
        !matched
    } else {
        matched
    }
}

/// Fuzzy match implementation
fn fuzzy_match(haystack: &str, query: &str, match_case: bool, query_lower: &str) -> bool {
    let (hay, q) = if match_case {
        (haystack.to_string(), query.to_string())
    } else {
        (haystack.to_ascii_lowercase(), query_lower.to_string())
    };

    let mut hay_iter = hay.chars();
    q.chars().all(|c| hay_iter.any(|hc| hc == c))
}

/// Filter by datetime range
fn filter_by_datetime(entry: &LogEntry, from_cmp: &str, to_cmp: &str) -> bool {
    if !from_cmp.is_empty() && !entry.datetime.is_empty() && entry.datetime.as_str() < from_cmp {
        return false;
    }

    if !to_cmp.is_empty() && !entry.datetime.is_empty() {
        let entry_prefix = &entry.datetime[..entry.datetime.len().min(to_cmp.len())];
        if entry_prefix > to_cmp {
            return false;
        }
    }

    true
}

/// Filter by line range
fn filter_by_line_range(entry: &LogEntry, line_from: usize, line_to: usize) -> bool {
    if line_from > 0 && entry.line < line_from {
        return false;
    }

    if line_to > 0 && entry.line > line_to {
        return false;
    }

    true
}
