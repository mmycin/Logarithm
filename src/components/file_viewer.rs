use crate::app::Theme;
use leptos::prelude::*;

#[component]
pub fn FileViewer(
    theme: ReadSignal<Theme>,
    open_files: ReadSignal<Vec<String>>,
    active_file: ReadSignal<Option<usize>>,
) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;

    let log_lines = vec![
        ("2024-05-06 10:23:45", "INFO", "Application started successfully"),
        ("2024-05-06 10:23:46", "DEBUG", "Initializing database connection"),
        ("2024-05-06 10:23:47", "SUCCESS", "Connected to PostgreSQL database"),
        ("2024-05-06 10:23:48", "INFO", "Loading configuration from config.toml"),
        ("2024-05-06 10:23:49", "WARNING", "Deprecated configuration key used: 'log_level'"),
        ("2024-05-06 10:23:50", "INFO", "Starting HTTP server on port 8080"),
        ("2024-05-06 10:23:51", "SUCCESS", "Server listening on http://localhost:8080"),
        ("2024-05-06 10:24:00", "INFO", "Received request: GET /api/v1/users"),
        ("2024-05-06 10:24:01", "DEBUG", "Querying database for users"),
        ("2024-05-06 10:24:02", "SUCCESS", "Fetched 15 users from database"),
        ("2024-05-06 10:24:03", "INFO", "Response sent: 200 OK"),
        ("2024-05-06 10:24:15", "ERROR", "Failed to process request: connection timeout"),
        ("2024-05-06 10:24:16", "INFO", "Retrying request (1/3)"),
        ("2024-05-06 10:24:20", "SUCCESS", "Request completed successfully on retry"),
    ];

    let selected_file = move || {
        active_file
            .get()
            .and_then(|idx| open_files.get().get(idx).cloned())
    };

    view! {
        <div class=move || {
            if is_dark() {
                "flex-1 flex flex-col bg-[#1e1e2e]"
            } else {
                "flex-1 flex flex-col bg-[#eff1f5]"
            }
        }>
            <div class="flex-1 overflow-auto font-mono">
                <div class=move || {
                    if is_dark() {
                        "px-3 py-2 text-[12px] text-[#a6adc8] border-b border-[#313244]/50 bg-[#11111b]"
                    } else {
                        "px-3 py-2 text-[12px] text-[#6c6f85] border-b border-[#9ca0b0]/30 bg-[#e6e9ef]"
                    }
                }>
                    {move || selected_file().unwrap_or_else(|| "No file selected".to_string())}
                </div>
                {move || {
                    if selected_file().is_none() {
                        view! {
                            <div class=move || {
                                if is_dark() {
                                    "px-3 py-6 text-[13px] text-[#a6adc8]"
                                } else {
                                    "px-3 py-6 text-[13px] text-[#6c6f85]"
                                }
                            }>
                                "Open a file to view its contents."
                            </div>
                        }
                        .into_any()
                    } else {
                        view! {
                <div class=move || {
                    if is_dark() {
                        "flex items-center px-3 py-1 border-b border-[#313244]/50 bg-[#181825] sticky top-0 z-10"
                    } else {
                        "flex items-center px-3 py-1 border-b border-[#9ca0b0]/30 bg-[#e6e9ef] sticky top-0 z-10"
                    }
                }>
                    <span class=move || {
                        if is_dark() {
                            "w-10 text-right text-[11px] text-[#6c7086] mr-3 flex-shrink-0 select-none"
                        } else {
                            "w-10 text-right text-[11px] text-[#8c8fa1] mr-3 flex-shrink-0 select-none"
                        }
                    }>
                        "Line"
                    </span>
                    <span class=move || {
                        if is_dark() {
                            "w-28 text-[11px] text-[#a6adc8] mr-3 flex-shrink-0"
                        } else {
                            "w-28 text-[11px] text-[#6c6f85] mr-3 flex-shrink-0"
                        }
                    }>
                        "Date"
                    </span>
                    <span class=move || {
                        if is_dark() {
                            "w-24 text-[11px] text-[#a6adc8] mr-3 flex-shrink-0"
                        } else {
                            "w-24 text-[11px] text-[#6c6f85] mr-3 flex-shrink-0"
                        }
                    }>
                        "Time"
                    </span>
                    <span class=move || {
                        if is_dark() {
                            "w-20 text-[11px] text-[#a6adc8] mr-3 flex-shrink-0 text-center"
                        } else {
                            "w-20 text-[11px] text-[#6c6f85] mr-3 flex-shrink-0 text-center"
                        }
                    }>
                        "Status"
                    </span>
                    <span class=move || {
                        if is_dark() {
                            "text-[11px] text-[#a6adc8]"
                        } else {
                            "text-[11px] text-[#6c6f85]"
                        }
                    }>
                        "Message"
                    </span>
                </div>
                {log_lines
                    .iter()
                    .enumerate()
                    .map(|(idx, (time, level, msg))| {
                        let (date_part, time_part) = time
                            .split_once(' ')
                            .or_else(|| time.split_once('T'))
                            .map(|(d, t)| (d, t))
                            .unwrap_or((time, ""));

                        let level_color = match *level {
                            "INFO" => if is_dark() { "#89b4fa" } else { "#1e66f5" },
                            "DEBUG" => if is_dark() { "#a6adc8" } else { "#6c6f85" },
                            "SUCCESS" => if is_dark() { "#a6e3a1" } else { "#40a02b" },
                            "WARNING" => if is_dark() { "#f9e2af" } else { "#df8e1d" },
                            "ERROR" => if is_dark() { "#f38ba8" } else { "#d20f39" },
                            _ => if is_dark() { "#cdd6f4" } else { "#4c4f69" },
                        };

                        view! {
                            <div class=move || {
                                if is_dark() {
                                    "flex items-center px-3 py-1 border-b border-[#313244]/30 hover:bg-[#313244]/15 transition-all duration-50"
                                } else {
                                    "flex items-center px-3 py-1 border-b border-[#9ca0b0]/15 hover:bg-[#ccd0da]/15 transition-all duration-50"
                                }
                            }>
                                <span class=move || {
                                    if is_dark() {
                                        "w-10 text-right text-[11px] text-[#6c7086] mr-3 flex-shrink-0 select-none"
                                    } else {
                                        "w-10 text-right text-[11px] text-[#8c8fa1] mr-3 flex-shrink-0 select-none"
                                    }
                                }>
                                    {format!("{}", idx + 1)}
                                </span>
                                <span class=move || {
                                    if is_dark() {
                                        "w-28 text-[11px] text-[#a6adc8] mr-3 flex-shrink-0"
                                    } else {
                                        "w-28 text-[11px] text-[#6c6f85] mr-3 flex-shrink-0"
                                    }
                                }>
                                    {date_part.to_string()}
                                </span>
                                <span class=move || {
                                    if is_dark() {
                                        "w-24 text-[11px] text-[#a6adc8] mr-3 flex-shrink-0"
                                    } else {
                                        "w-24 text-[11px] text-[#6c6f85] mr-3 flex-shrink-0"
                                    }
                                }>
                                    {time_part.to_string()}
                                </span>
                                <span
                                    class="w-20 px-2 py-0.5 rounded text-[11px] font-semibold mr-3 flex-shrink-0 text-center"
                                    style:background-color=format!("{}/12", level_color)
                                    style:color=level_color
                                >
                                    {level.to_string()}
                                </span>
                                <span class=move || {
                                    if is_dark() {
                                        "text-[13px] text-[#cdd6f4]"
                                    } else {
                                        "text-[13px] text-[#4c4f69]"
                                    }
                                }>
                                    {msg.to_string()}
                                </span>
                            </div>
                        }
                    })
                    .collect_view()}
                        }
                        .into_any()
                    }
                }}
            </div>
        </div>
    }
}
