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

mod commands {
    use super::{FilterParams, LogEntry};

    /// Parse raw log text into structured entries.
    #[tauri::command]
    pub fn parse_log(text: String) -> Vec<LogEntry> {
        const STATUSES: [&str; 8] = [
            "TRACE", "DEBUG", "INFO", "WARN", "WARNING", "ERROR", "SUCCESS", "FATAL",
        ];

        text.lines()
            .enumerate()
            .filter_map(|(line_idx, line)| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }

                let tokens: Vec<&str> = line.split_whitespace().collect();
                let mut datetime = String::new();
                let mut start_idx = 0usize;

                if let Some(first) = tokens.first().copied() {
                    if let Some((d, t)) = first.split_once('T') {
                        if !d.is_empty() && !t.is_empty() {
                            datetime = format!("{} {}", d, t);
                            start_idx = 1;
                        }
                    } else if tokens.len() >= 2 {
                        let d = tokens[0];
                        let t = tokens[1];
                        if d.contains('-') && t.contains(':') {
                            datetime = format!("{} {}", d, t);
                            start_idx = 2;
                        }
                    }
                }

                let mut status_idx: Option<usize> = None;
                let mut status = String::new();
                for (i, tok) in tokens.iter().enumerate().skip(start_idx) {
                    let cleaned = tok
                        .trim_matches(|c: char| !c.is_alphanumeric())
                        .to_ascii_uppercase();
                    if STATUSES.iter().any(|s| *s == cleaned) {
                        status_idx = Some(i);
                        status = cleaned;
                        break;
                    }
                }

                let message = match status_idx {
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
                };

                Some(LogEntry {
                    line: line_idx + 1,
                    datetime,
                    status,
                    message,
                })
            })
            .collect()
    }

    /// Filter already-parsed entries using the given params.
    /// All heavy lifting happens in Rust, not in the frontend.
    #[tauri::command]
    pub fn filter_entries(entries: Vec<LogEntry>, params: FilterParams) -> Vec<LogEntry> {
        let from_cmp = params.from_datetime.replace('T', " ");
        let to_cmp = params.to_datetime.replace('T', " ");

        let status_filter = params.status.to_ascii_lowercase();
        let custom_filter = params.custom_status.trim().to_ascii_uppercase();

        entries
            .into_iter()
            .filter(|entry| {
                // ── Status ──────────────────────────────────────────────
                if status_filter != "all" {
                    let es = entry.status.to_ascii_uppercase();
                    let matches = if status_filter == "custom" {
                        if custom_filter.is_empty() {
                            true
                        } else {
                            custom_filter
                                .split(',')
                                .map(|s| s.trim())
                                .any(|s| es == s)
                        }
                    } else if status_filter == "warn" {
                        es == "WARN" || es == "WARNING"
                    } else {
                        es == status_filter.to_ascii_uppercase()
                    };
                    if !matches {
                        return false;
                    }
                }

                // ── Search ───────────────────────────────────────────────
                if !params.query.is_empty() {
                    let haystack =
                        format!("{} {} {}", entry.datetime, entry.status, entry.message);
                    let matched = if params.fuzzy {
                        let (h, q) = if params.match_case {
                            (haystack.clone(), params.query.clone())
                        } else {
                            (
                                haystack.to_ascii_lowercase(),
                                params.query.to_ascii_lowercase(),
                            )
                        };
                        let mut hi = h.chars();
                        q.chars().all(|c| hi.any(|hc| hc == c))
                    } else if params.match_case {
                        haystack.contains(&params.query)
                    } else {
                        haystack
                            .to_ascii_lowercase()
                            .contains(&params.query.to_ascii_lowercase())
                    };
                    if !matched {
                        return false;
                    }
                }

                // ── DateTime range ───────────────────────────────────────
                if !from_cmp.is_empty() && !entry.datetime.is_empty() {
                    if entry.datetime < from_cmp {
                        return false;
                    }
                }
                if !to_cmp.is_empty() && !entry.datetime.is_empty() {
                    let entry_prefix =
                        &entry.datetime[..entry.datetime.len().min(to_cmp.len())];
                    if entry_prefix > to_cmp.as_str() {
                        return false;
                    }
                }

                true
            })
            .collect()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            commands::parse_log,
            commands::filter_entries
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
