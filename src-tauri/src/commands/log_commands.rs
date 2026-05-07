/// Log parsing and filtering commands.

use crate::types::{FilterParams, LogEntry};

const STATUSES: [&str; 8] = [
    "TRACE", "DEBUG", "INFO", "WARN", "WARNING", "ERROR", "SUCCESS", "FATAL",
];

#[tauri::command]
pub fn parse_log(text: String) -> Vec<LogEntry> {
    text.lines()
        .enumerate()
        .filter_map(|(line_idx, line)| {
            let line = line.trim();
            if line.is_empty() {
                return None;
            }

            let tokens: Vec<&str> = line.split_whitespace().collect();
            let (datetime, start_idx) = extract_datetime(&tokens);
            let (status, status_idx) = extract_status(&tokens, start_idx);
            let message = extract_message(&tokens, start_idx, status_idx);

            Some(LogEntry {
                line: line_idx + 1,
                datetime,
                status,
                message,
            })
        })
        .collect()
}

fn extract_datetime(tokens: &[&str]) -> (String, usize) {
    if let Some(first) = tokens.first().copied() {
        if let Some((d, t)) = first.split_once('T') {
            if !d.is_empty() && !t.is_empty() {
                return (format!("{} {}", d, t), 1);
            }
        } else if tokens.len() >= 2 {
            let d = tokens[0];
            let t = tokens[1];
            if d.contains('-') && t.contains(':') {
                return (format!("{} {}", d, t), 2);
            }
        }
    }
    (String::new(), 0)
}

fn extract_status(tokens: &[&str], start_idx: usize) -> (String, Option<usize>) {
    for (i, tok) in tokens.iter().enumerate().skip(start_idx) {
        let cleaned = tok
            .trim_matches(|c: char| !c.is_alphanumeric())
            .to_ascii_uppercase();
        if STATUSES.iter().any(|s| *s == cleaned) {
            return (cleaned, Some(i));
        }
    }
    (String::new(), None)
}

fn extract_message(tokens: &[&str], start_idx: usize, status_idx: Option<usize>) -> String {
    match status_idx {
        Some(si) => tokens
            .iter()
            .enumerate()
            .filter_map(|(i, t)| {
                if i == si || i < start_idx {
                    None
                } else {
                    Some(*t)
                }
            })
            .collect::<Vec<_>>()
            .join(" "),
        None => tokens
            .iter()
            .skip(start_idx)
            .copied()
            .collect::<Vec<_>>()
            .join(" "),
    }
}

#[tauri::command]
pub fn filter_entries(entries: Vec<LogEntry>, params: FilterParams) -> Vec<LogEntry> {
    let from_cmp = params.from_datetime.replace('T', " ");
    let to_cmp = params.to_datetime.replace('T', " ");
    let sf = params.status.to_ascii_lowercase();
    let cf = params.custom_status.trim().to_ascii_uppercase();

    entries
        .into_iter()
        .filter(|entry| {
            filter_by_status(entry, &sf, &cf)
                && filter_by_query(entry, &params)
                && filter_by_datetime(entry, &from_cmp, &to_cmp)
        })
        .collect()
}

fn filter_by_status(entry: &LogEntry, sf: &str, cf: &str) -> bool {
    if sf == "all" {
        return true;
    }

    let es = entry.status.to_ascii_uppercase();
    if sf == "custom" {
        cf.is_empty() || cf.split(',').map(|s| s.trim()).any(|s| es == s)
    } else if sf == "warn" {
        es == "WARN" || es == "WARNING"
    } else {
        es == sf.to_ascii_uppercase()
    }
}

fn filter_by_query(entry: &LogEntry, params: &FilterParams) -> bool {
    if params.query.is_empty() {
        return true;
    }

    let hay = format!("{} {} {}", entry.datetime, entry.status, entry.message);
    if params.fuzzy {
        fuzzy_match(&hay, &params.query, params.match_case)
    } else if params.match_case {
        hay.contains(&params.query)
    } else {
        hay.to_ascii_lowercase()
            .contains(&params.query.to_ascii_lowercase())
    }
}

fn fuzzy_match(haystack: &str, query: &str, match_case: bool) -> bool {
    let (h, q) = if match_case {
        (haystack.to_string(), query.to_string())
    } else {
        (haystack.to_ascii_lowercase(), query.to_ascii_lowercase())
    };
    let mut hay_iter = h.chars();
    q.chars().all(|c| hay_iter.any(|hc| hc == c))
}

fn filter_by_datetime(entry: &LogEntry, from_cmp: &str, to_cmp: &str) -> bool {
    if !from_cmp.is_empty() && !entry.datetime.is_empty() && entry.datetime.as_str() < from_cmp {
        return false;
    }

    if !to_cmp.is_empty() && !entry.datetime.is_empty() {
        let ep = &entry.datetime[..entry.datetime.len().min(to_cmp.len())];
        if ep > to_cmp {
            return false;
        }
    }

    true
}
