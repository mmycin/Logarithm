/// Logarithm Tauri Backend.
/// 
/// Provides log parsing, filtering, file operations, and AI chat integration.

mod ai;
mod commands;
mod types;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            commands::log_commands::parse_log,
            commands::log_commands::filter_entries,
            commands::file_commands::open_url,
            commands::file_commands::read_file_by_path,
            ai::ai_chat,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
