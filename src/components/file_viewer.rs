use crate::app::Theme;
use leptos::prelude::*;

#[component]
pub fn FileViewer(theme: ReadSignal<Theme>) -> impl IntoView {
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

    view! {
        <div class=move || {
            if is_dark() {
                "flex-1 flex flex-col bg-[#1e1e2e]"
            } else {
                "flex-1 flex flex-col bg-[#eff1f5]"
            }
        }>
            <div class="flex-1 overflow-auto font-mono">
                {log_lines
                    .iter()
                    .enumerate()
                    .map(|(idx, (time, level, msg))| {
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
                                        "w-36 text-[11px] text-[#a6adc8] mr-3 flex-shrink-0"
                                    } else {
                                        "w-36 text-[11px] text-[#6c6f85] mr-3 flex-shrink-0"
                                    }
                                }>
                                    {time.to_string()}
                                </span>
                                <span
                                    class="px-2 py-0.5 rounded text-[11px] font-semibold mr-3 flex-shrink-0"
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
            </div>
        </div>
    }
}
