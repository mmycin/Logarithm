use crate::ai::AiPanel;
use crate::components::{BottomBar, FileBar, FilterPanel, TitleBar};
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LoganAction, LogFile, Theme};
use crate::viewer::FileViewer;
use leptos::prelude::*;

/// All filter state — lifted to App so FilterPanel and FileViewer share it.
#[derive(Clone, Copy)]
pub struct FilterState {
    // Level
    pub selected_status: ReadSignal<String>,
    pub set_selected_status: WriteSignal<String>,
    pub custom_status: ReadSignal<String>,
    pub set_custom_status: WriteSignal<String>,
    pub inherit_level: ReadSignal<bool>,       // continuation lines inherit parent level
    pub set_inherit_level: WriteSignal<bool>,

    // Search
    pub search_query: ReadSignal<String>,
    pub set_search_query: WriteSignal<String>,
    pub match_case: ReadSignal<bool>,
    pub set_match_case: WriteSignal<bool>,
    pub fuzzy_find: ReadSignal<bool>,
    pub set_fuzzy_find: WriteSignal<bool>,
    pub regex_mode: ReadSignal<bool>,
    pub set_regex_mode: WriteSignal<bool>,
    pub invert_match: ReadSignal<bool>,
    pub set_invert_match: WriteSignal<bool>,
    pub search_in_datetime: ReadSignal<bool>,
    pub set_search_in_datetime: WriteSignal<bool>,

    // Date/time
    pub from_datetime: ReadSignal<String>,
    pub set_from_datetime: WriteSignal<String>,
    pub to_datetime: ReadSignal<String>,
    pub set_to_datetime: WriteSignal<String>,

    // Line range
    pub line_from: ReadSignal<String>,
    pub set_line_from: WriteSignal<String>,
    pub line_to: ReadSignal<String>,
    pub set_line_to: WriteSignal<String>,

    // Advanced
    pub hide_no_level: ReadSignal<bool>,       // hide lines with no level
    pub set_hide_no_level: WriteSignal<bool>,
    pub min_severity: ReadSignal<String>,      // "all"|"debug"|"info"|"warn"|"error"|"fatal"
    pub set_min_severity: WriteSignal<String>,
}

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme)                             = signal(Theme::Dark);
    let (open_dialog_trigger, set_open_dialog_trigger) = signal(0u32);
    let (open_files, set_open_files)                   = signal(Vec::<LogFile>::new());
    let (active_file, set_active_file)                 = signal(None::<usize>);

    // Modals
    let (show_shortcuts, set_show_shortcuts) = signal(false);
    let (show_about, set_show_about)         = signal(false);

    // AI panel
    let (ai_open, set_ai_open)     = signal(false);
    let (ai_width, set_ai_width)   = signal(340u32);

    // Logan context bridge: FileViewer → AiPanel
    let (logan_action, set_logan_action) = signal(None::<crate::app::LoganAction>);

    // Filter panel
    let (filter_open, set_filter_open) = signal(true);

    // Filter state
    let (selected_status, set_selected_status) = signal("all".to_string());
    let (custom_status, set_custom_status)     = signal(String::new());
    let (inherit_level, set_inherit_level)     = signal(true);
    let (search_query, set_search_query)       = signal(String::new());
    let (match_case, set_match_case)           = signal(false);
    let (fuzzy_find, set_fuzzy_find)           = signal(false);
    let (regex_mode, set_regex_mode)           = signal(false);
    let (invert_match, set_invert_match)       = signal(false);
    let (search_in_datetime, set_search_in_datetime) = signal(false);
    let (from_datetime, set_from_datetime)     = signal(String::new());
    let (to_datetime, set_to_datetime)         = signal(String::new());
    let (line_from, set_line_from)             = signal(String::new());
    let (line_to, set_line_to)                 = signal(String::new());
    let (hide_no_level, set_hide_no_level)     = signal(false);
    let (min_severity, set_min_severity)       = signal("all".to_string());

    let filter_state = FilterState {
        selected_status, set_selected_status,
        custom_status, set_custom_status,
        inherit_level, set_inherit_level,
        search_query, set_search_query,
        match_case, set_match_case,
        fuzzy_find, set_fuzzy_find,
        regex_mode, set_regex_mode,
        invert_match, set_invert_match,
        search_in_datetime, set_search_in_datetime,
        from_datetime, set_from_datetime,
        to_datetime, set_to_datetime,
        line_from, set_line_from,
        line_to, set_line_to,
        hide_no_level, set_hide_no_level,
        min_severity, set_min_severity,
    };

    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    let total_lines = move || {
        active_file.get()
            .and_then(|idx| open_files.get().get(idx).cloned())
            .map(|f| f.entries.len())
            .unwrap_or(0)
    };

    view! {
        <div style=move || format!(
            "display:flex;flex-direction:column;height:100vh;background:{};\
             font-family:'Inter',sans-serif;overflow:hidden;position:relative;",
            tok().bg_base
        )>
            <TitleBar theme set_theme set_open_dialog_trigger
                set_show_shortcuts set_show_about
                filter_open set_filter_open set_ai_open />

            // ── Main body: [FilterPanel?] | [FileArea] | [AiPanel?] ───────
            <div style="flex:1;display:flex;overflow:hidden;min-height:0">

                // Left filter panel
                <Show when=move || filter_open.get()>
                    <FilterPanel theme filter_state open_files active_file />
                </Show>

                // Center: file tabs + viewer
                <div style="flex:1;display:flex;flex-direction:column;overflow:hidden;min-width:0">
                    <FileBar theme open_files set_open_files active_file set_active_file
                        open_dialog_trigger filter_state />
                    <FileViewer theme open_files active_file filter_state
                        set_logan_action set_ai_open />
                </div>

                // Right AI panel — independent, resizable
                <Show when=move || ai_open.get()>
                    <AiPanel theme ai_width set_ai_width set_ai_open
                        logan_action set_logan_action open_files />
                </Show>
            </div>

            <BottomBar theme total_lines ai_open set_ai_open filter_open set_filter_open />

            // ── Keyboard Shortcuts Modal ───────────────────────────────────
            <Show when=move || show_shortcuts.get()>
                <div
                    style="position:fixed;inset:0;z-index:1000;display:flex;align-items:center;\
                           justify-content:center;background:rgba(0,0,0,0.65);backdrop-filter:blur(6px)"
                    on:click=move |_| set_show_shortcuts.set(false)
                >
                    <div
                        style=move || format!(
                            "background:{};border:1px solid {};border-radius:14px;\
                             box-shadow:0 32px 96px rgba(0,0,0,0.8);width:500px;max-height:80vh;\
                             overflow:hidden;display:flex;flex-direction:column;",
                            tok().bg_elevated, tok().border
                        )
                        on:click=|ev| ev.stop_propagation()
                    >
                        <div style=move || format!(
                            "display:flex;align-items:center;justify-content:space-between;\
                             padding:18px 22px;border-bottom:1px solid {};flex-shrink:0;",
                            tok().border
                        )>
                            <span style=move || format!(
                                "font-size:15px;font-weight:700;color:{}", tok().text_primary
                            )>"Keyboard Shortcuts"</span>
                            <button
                                style=move || format!(
                                    "width:28px;height:28px;display:flex;align-items:center;\
                                     justify-content:center;border-radius:6px;border:none;\
                                     background:{};color:{};cursor:default;font-size:14px;",
                                    tok().bg_input, tok().text_secondary
                                )
                                on:click=move |_| set_show_shortcuts.set(false)
                            >"✕"</button>
                        </div>
                        <div style="overflow-y:auto;padding:8px 0">
                            {[
                                ("File", vec![
                                    ("Ctrl+O", "Open log file"),
                                    ("Ctrl+W", "Close active tab"),
                                ]),
                                ("Navigation", vec![
                                    ("Ctrl+F", "Focus search"),
                                    ("Ctrl+T", "Toggle theme"),
                                    ("Ctrl+B", "Toggle filter panel"),
                                    ("Ctrl+L", "Toggle AI chat"),
                                ]),
                                ("Help", vec![
                                    ("Ctrl+/", "Keyboard shortcuts"),
                                ]),
                            ].iter().map(|(section, items)| {
                                let section = *section;
                                let items = items.clone();
                                view! {
                                    <div style="padding:4px 0 8px">
                                        <div style=move || format!(
                                            "padding:8px 22px 4px;font-size:10px;font-weight:700;\
                                             color:{};letter-spacing:0.09em;text-transform:uppercase;",
                                            tok().text_muted
                                        )>{section}</div>
                                        {items.iter().map(|(key, desc)| {
                                            let key = *key; let desc = *desc;
                                            view! {
                                                <div style="display:flex;align-items:center;\
                                                    justify-content:space-between;padding:7px 22px">
                                                    <span style=move || format!(
                                                        "font-size:13px;color:{}", tok().text_primary
                                                    )>{desc}</span>
                                                    <span style=move || format!(
                                                        "font-size:11px;color:{};background:{};\
                                                         border:1px solid {};border-radius:5px;\
                                                         padding:2px 9px;font-family:'Fira Code',monospace;",
                                                        tok().text_secondary, tok().bg_input, tok().border
                                                    )>{key}</span>
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </Show>

            // ── About Modal ───────────────────────────────────────────────
            <Show when=move || show_about.get()>
                <div
                    style="position:fixed;inset:0;z-index:1000;display:flex;align-items:center;\
                           justify-content:center;background:rgba(0,0,0,0.65);backdrop-filter:blur(6px)"
                    on:click=move |_| set_show_about.set(false)
                >
                    <div
                        style=move || format!(
                            "background:{};border:1px solid {};border-radius:14px;\
                             box-shadow:0 32px 96px rgba(0,0,0,0.8);width:380px;overflow:hidden;",
                            tok().bg_elevated, tok().border
                        )
                        on:click=|ev| ev.stop_propagation()
                    >
                        <div style="background:linear-gradient(135deg,#131d35,#1a1535);\
                            padding:32px 28px;display:flex;flex-direction:column;align-items:center;gap:14px">
                            <div style="width:60px;height:60px;border-radius:16px;\
                                background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                                display:flex;align-items:center;justify-content:center;\
                                box-shadow:0 10px 30px rgba(124,157,255,0.4)">
                                <img src="/public/StoreLogo.png" width="40" height="40" style="border-radius:8px" alt="Logarithm" />
                            </div>
                            <div style="text-align:center">
                                <div style="font-size:22px;font-weight:700;color:#e8eaf0;margin-bottom:3px">
                                    "Logarithm"
                                </div>
                                <div style="font-size:12px;color:#4a5168">"v0.1.0 — Early Access"</div>
                            </div>
                        </div>
                        <div style=move || format!("padding:22px 26px;background:{}", tok().bg_elevated)>
                            <p style=move || format!(
                                "font-size:13px;color:{};line-height:1.65;margin:0 0 18px",
                                tok().text_secondary
                            )>
                                "A modern, blazing-fast log file viewer built with Tauri and Leptos. \
                                 Designed for developers who need to quickly inspect, filter, and analyse log files."
                            </p>
                            <div style=move || format!(
                                "border-top:1px solid {};padding-top:16px;display:flex;\
                                 flex-direction:column;gap:8px;",
                                tok().border
                            )>
                                {[
                                    ("Author", "mmycin"),
                                    ("Built with", "Tauri + Leptos + Rust"),
                                    ("License", "MIT"),
                                    ("Repository", "github.com/mmycin/Logarithm"),
                                ].iter().map(|(k, v)| {
                                    let k = *k; let v = *v;
                                    view! {
                                        <div style="display:flex;justify-content:space-between;align-items:center">
                                            <span style=move || format!("font-size:12px;color:{}", tok().text_muted)>{k}</span>
                                            <span style=move || format!("font-size:12px;font-weight:500;color:{}", tok().text_primary)>{v}</span>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                            <button
                                style=move || format!(
                                    "margin-top:18px;width:100%;padding:9px;border-radius:8px;\
                                     border:none;background:{};color:{};font-size:13px;\
                                     font-weight:500;cursor:default;",
                                    tok().bg_input, tok().text_secondary
                                )
                                on:click=move |_| set_show_about.set(false)
                            >"Close"</button>
                        </div>
                    </div>
                </div>
            </Show>
        </div>
    }
}
