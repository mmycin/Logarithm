/// File mention autocomplete handler.
/// 
/// Handles @ mention autocomplete for referencing open files.

use crate::shared::types::LogFile;
use leptos::prelude::*;

/// Handle input changes for @ mention autocomplete
pub fn handle_mention_input(
    val: &str,
    open_files: &[LogFile],
    set_mention_suggestions: WriteSignal<Vec<String>>,
    set_mention_active: WriteSignal<bool>,
    set_mention_selected: WriteSignal<usize>,
) {
    if let Some(at_pos) = val.rfind('@') {
        let after_at = &val[at_pos + 1..];
        let before_at = if at_pos > 0 { &val[at_pos - 1..at_pos] } else { " " };
        
        if before_at.chars().all(|c| c.is_whitespace()) && !after_at.contains(char::is_whitespace) {
            let query = after_at.to_lowercase();
            let suggestions: Vec<String> = open_files.iter()
                .map(|f| {
                    f.name.rsplit('/').next()
                        .or_else(|| f.name.rsplit('\\').next())
                        .unwrap_or(&f.name)
                        .to_string()
                })
                .filter(|name| query.is_empty() || name.to_lowercase().contains(&query))
                .collect();
            
            if !suggestions.is_empty() {
                set_mention_suggestions.set(suggestions);
                set_mention_active.set(true);
                set_mention_selected.set(0);
            } else {
                set_mention_active.set(false);
            }
        } else {
            set_mention_active.set(false);
        }
    } else {
        set_mention_active.set(false);
    }
}

/// Handle keyboard navigation in mention autocomplete
pub fn handle_mention_keydown(
    key: &str,
    suggestions: &[String],
    selected: usize,
    set_mention_selected: WriteSignal<usize>,
) -> bool {
    match key {
        "ArrowDown" => {
            if selected + 1 < suggestions.len() {
                set_mention_selected.set(selected + 1);
            }
            true
        }
        "ArrowUp" => {
            if selected > 0 {
                set_mention_selected.set(selected - 1);
            }
            true
        }
        _ => false,
    }
}

/// Select a file from mention autocomplete
pub fn select_mention(
    file_name: String,
    input: ReadSignal<String>,
    set_input: WriteSignal<String>,
    set_mentioned_files: WriteSignal<Vec<String>>,
    set_mention_active: WriteSignal<bool>,
) {
    set_mentioned_files.update(|files| {
        if !files.contains(&file_name) {
            files.push(file_name);
        }
    });
    
    let current = input.get();
    if let Some(at_pos) = current.rfind('@') {
        let new_val = current[..at_pos].trim_end().to_string();
        set_input.set(new_val);
    }
    
    set_mention_active.set(false);
}
