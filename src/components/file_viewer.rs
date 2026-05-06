use crate::app::{FilterState, LogEntry, LogFile, Theme, DARK, LIGHT};
use crate::components::severity::{level_colors, severity_rank};
use leptos::prelude::*;

// ── Filter logic ──────────────────────────────────────────────────────────────

struct FilterParams {
    status: String,
    custom_status: String,
    inherit_level: bool,
    query: String,
    match_case: bool,
    fuzzy: bool,
    #[allow(dead_code)]
    regex_mode: bool,
    invert_match: bool,
    search_in_datetime: bool,
    from_datetime: String,
    to_datetime: String,
    line_from: usize,
    line_to: usize,
    hide_no_level: bool,
    min_severity: String,
}

/// Propagate level to continuation lines (lines with no status).
fn propagate_levels(entries: &[LogEntry]) -> Vec<LogEntry> {
    let mut out = Vec::with_capacity(entries.len());
    let mut cur = String::new();
    for e in entries {
        if !e.status.is_empty() {
            cur = e.status.clone();
            out.push(e.clone());
        } else {
            let mut inh = e.clone();
            inh.status = cur.clone();
            out.push(inh);
        }
    }
    out
}

fn apply_filters(entries: &[LogEntry], p: &FilterParams) -> Vec<LogEntry> {
    let owned;
    let entries: &[LogEntry] = if p.inherit_level {
        owned = propagate_levels(entries);
        &owned
    } else {
        entries
    };

    let from_cmp = p.from_datetime.replace('T', " ");
    let to_cmp   = p.to_datetime.replace('T', " ");
    let sf       = p.status.to_ascii_lowercase();
    let cf       = p.custom_status.trim().to_ascii_uppercase();
    let ql       = p.query.to_ascii_lowercase();
    let min_rank = severity_rank(&p.min_severity);

    entries.iter().filter(|e| {
        // Level
        if sf != "all" {
            let es = e.status.to_ascii_uppercase();
            let ok = if sf == "custom" {
                cf.is_empty() || cf.split(',').map(|s| s.trim()).any(|s| es == s)
            } else if sf == "warn" {
                es == "WARN" || es == "WARNING"
            } else {
                es == sf.to_ascii_uppercase()
            };
            if !ok { return false; }
        }
        // Min severity
        if p.min_severity != "all" && !e.status.is_empty() {
            if severity_rank(&e.status) < min_rank { return false; }
        }
        // Hide no-level
        if p.hide_no_level && e.status.is_empty() { return false; }
        // Search
        if !p.query.is_empty() {
            let hay = if p.search_in_datetime {
                format!("{} {} {}", e.datetime, e.status, e.message)
            } else {
                format!("{} {}", e.status, e.message)
            };
            let matched = if p.fuzzy {
                let (h, q) = if p.match_case { (hay.clone(), p.query.clone()) }
                             else { (hay.to_ascii_lowercase(), ql.clone()) };
                let mut hi = h.chars();
                q.chars().all(|c| hi.any(|hc| hc == c))
            } else if p.match_case {
                hay.contains(&p.query)
            } else {
                hay.to_ascii_lowercase().contains(&ql)
            };
            let matched = if p.invert_match { !matched } else { matched };
            if !matched { return false; }
        }
        // DateTime
        if !from_cmp.is_empty() && !e.datetime.is_empty() && e.datetime < from_cmp { return false; }
        if !to_cmp.is_empty() && !e.datetime.is_empty() {
            let ep = &e.datetime[..e.datetime.len().min(to_cmp.len())];
            if ep > to_cmp.as_str() { return false; }
        }
        // Line range
        if p.line_from > 0 && e.line < p.line_from { return false; }
        if p.line_to   > 0 && e.line > p.line_to   { return false; }
        true
    }).cloned().collect()
}

// ── Log-group numbering ───────────────────────────────────────────────────────
// A "group" is a levelled line + all following no-level lines.
// All lines in a group share the group's starting line number for display.
// The original `entry.line` is preserved for filtering; `group_line` is for display.
#[derive(Clone)]
struct DisplayEntry {
    entry: LogEntry,
    group_line: usize,   // line number of the group leader
    is_continuation: bool,
}

fn assign_groups(entries: Vec<LogEntry>) -> Vec<DisplayEntry> {
    let mut out = Vec::with_capacity(entries.len());
    let mut group_line = 0usize;
    for e in entries {
        if !e.status.is_empty() {
            group_line = e.line;
            out.push(DisplayEntry { group_line, is_continuation: false, entry: e });
        } else {
            out.push(DisplayEntry { group_line, is_continuation: true, entry: e });
        }
    }
    out
}

// ── Welcome page ─────────────────────────────────────────────────────────────

#[component]
fn WelcomePage(theme: ReadSignal<Theme>) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    view! {
        <div style=move || format!(
            "flex:1;overflow-y:auto;padding:40px 48px;background:{};",
            tok().bg_base
        )>
            <div style="display:flex;align-items:center;gap:16px;margin-bottom:36px">
                <div style="width:52px;height:52px;border-radius:14px;\
                    background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                    display:flex;align-items:center;justify-content:center;\
                    box-shadow:0 8px 24px rgba(124,157,255,0.25);flex-shrink:0">
                    <span style="color:white;font-weight:800;font-size:22px">"L"</span>
                </div>
                <div>
                    <h1 style=move || format!(
                        "font-size:22px;font-weight:700;color:{};margin:0 0 3px;",
                        tok().text_primary
                    )>"Welcome to Logarithm"</h1>
                    <p style=move || format!(
                        "font-size:13px;color:{};margin:0;", tok().text_muted
                    )>"A modern, fast log file viewer"</p>
                </div>
            </div>
            <div style=move || format!(
                "background:{};border:1px solid {};border-radius:10px;\
                 padding:20px 24px;margin-bottom:20px;",
                tok().bg_surface, tok().border
            )>
                <h2 style=move || format!(
                    "font-size:11px;font-weight:700;color:{};margin:0 0 14px;\
                     letter-spacing:0.06em;text-transform:uppercase;",
                    tok().text_muted
                )>"Quick Start"</h2>
                <div style="display:flex;flex-direction:column;gap:10px">
                    {[
                        ("📂","Open a log file","Ctrl+O or File → Open — only .log files"),
                        ("🔍","Filter by level","Use the left panel to pick a level"),
                        ("🔎","Search messages","Fuzzy, regex, case-sensitive, invert — all supported"),
                        ("📅","Filter by date","From / To date range in the left panel"),
                        ("✨","Logar AI","Click AI Chat in the bottom bar (coming soon)"),
                    ].iter().map(|(icon, title, desc)| {
                        let icon = *icon; let title = *title; let desc = *desc;
                        view! {
                            <div style="display:flex;align-items:flex-start;gap:12px">
                                <span style="font-size:16px;flex-shrink:0;margin-top:1px">{icon}</span>
                                <div>
                                    <span style=move || format!(
                                        "font-size:13px;font-weight:600;color:{};",
                                        tok().text_primary
                                    )>{title}</span>
                                    <span style=move || format!(
                                        "font-size:12px;color:{};margin-left:8px;",
                                        tok().text_secondary
                                    )>{desc}</span>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
            <div style=move || format!(
                "background:{};border:1px solid {};border-radius:10px;\
                 padding:20px 24px;margin-bottom:20px;",
                tok().bg_surface, tok().border
            )>
                <h2 style=move || format!(
                    "font-size:11px;font-weight:700;color:{};margin:0 0 14px;\
                     letter-spacing:0.06em;text-transform:uppercase;",
                    tok().text_muted
                )>"Keyboard Shortcuts"</h2>
                <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">
                    {[
                        ("Ctrl+O","Open file"),("Ctrl+W","Close tab"),
                        ("Ctrl+F","Focus search"),("Ctrl+T","Toggle theme"),
                        ("Ctrl+B","Toggle filters"),("Ctrl+/","Show shortcuts"),
                    ].iter().map(|(key, desc)| {
                        let key = *key; let desc = *desc;
                        view! {
                            <div style="display:flex;align-items:center;justify-content:space-between;gap:8px">
                                <span style=move || format!("font-size:12px;color:{}", tok().text_secondary)>{desc}</span>
                                <span style=move || format!(
                                    "font-size:11px;color:{};background:{};\
                                     border:1px solid {};border-radius:4px;\
                                     padding:1px 7px;font-family:'Fira Code',monospace;\
                                     white-space:nowrap;flex-shrink:0;",
                                    tok().text_secondary, tok().bg_elevated, tok().border
                                )>{key}</span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
            <p style=move || format!(
                "font-size:11px;color:{};text-align:center;margin:0;", tok().text_muted
            )>
                "Built with Tauri + Leptos + Rust · "
                <span
                    style=move || format!("color:{};text-decoration:underline;cursor:pointer;", tok().accent)
                    on:click=move |_| {
                        use wasm_bindgen::prelude::*;
                        use wasm_bindgen_futures::spawn_local;
                        use js_sys::Reflect;
                        #[wasm_bindgen]
                        extern "C" {
                            #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
                            async fn invoke(cmd: &str, args: JsValue) -> JsValue;
                        }
                        spawn_local(async {
                            let args = js_sys::Object::new();
                            let _ = Reflect::set(&args, &JsValue::from_str("url"), &JsValue::from_str("https://github.com/mmycin/Logarithm"));
                            let _ = invoke("open_url", JsValue::from(args)).await;
                        });
                    }
                >
                    "github.com/mmycin/Logarithm"
                </span>
            </p>
        </div>
    }
}

// ── Main viewer ───────────────────────────────────────────────────────────────

#[component]
pub fn FileViewer(
    theme: ReadSignal<Theme>,
    open_files: ReadSignal<Vec<LogFile>>,
    active_file: ReadSignal<Option<usize>>,
    filter_state: FilterState,
) -> impl IntoView {
    let tok  = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let dark = move || theme.get() == Theme::Dark;

    let selected_file = move || -> Option<LogFile> {
        active_file.get().and_then(|idx| open_files.get().get(idx).cloned())
    };

    let display_entries = move || -> Vec<DisplayEntry> {
        let Some(file) = selected_file() else { return vec![]; };
        let lf = filter_state.line_from.get().parse::<usize>().unwrap_or(0);
        let lt = filter_state.line_to.get().parse::<usize>().unwrap_or(0);
        let filtered = apply_filters(&file.entries, &FilterParams {
            status:             filter_state.selected_status.get(),
            custom_status:      filter_state.custom_status.get(),
            inherit_level:      filter_state.inherit_level.get(),
            query:              filter_state.search_query.get(),
            match_case:         filter_state.match_case.get(),
            fuzzy:              filter_state.fuzzy_find.get(),
            regex_mode:         filter_state.regex_mode.get(),
            invert_match:       filter_state.invert_match.get(),
            search_in_datetime: filter_state.search_in_datetime.get(),
            from_datetime:      filter_state.from_datetime.get(),
            to_datetime:        filter_state.to_datetime.get(),
            line_from:          lf,
            line_to:            lt,
            hide_no_level:      filter_state.hide_no_level.get(),
            min_severity:       filter_state.min_severity.get(),
        });
        assign_groups(filtered)
    };

    view! {
        <div style=move || format!(
            "flex:1;display:flex;flex-direction:column;background:{};overflow:hidden;",
            tok().bg_base
        )>
            {move || {
                if selected_file().is_none() {
                    view! { <WelcomePage theme /> }.into_any()
                } else {
                    view! {
                        <div style="flex:1;display:flex;flex-direction:column;overflow:hidden">
                            // Column header
                            <div style=move || format!(
                                "display:flex;align-items:center;padding:0 12px;height:26px;\
                                 border-bottom:1px solid {};background:{};flex-shrink:0;",
                                tok().border_subtle, tok().bg_surface
                            )>
                                <span style=move || format!(
                                    "width:44px;text-align:right;font-size:10px;font-weight:600;\
                                     color:{};margin-right:12px;flex-shrink:0;letter-spacing:0.06em;",
                                    tok().text_muted
                                )>"LINE"</span>
                                <span style=move || format!(
                                    "width:160px;font-size:10px;font-weight:600;color:{};\
                                     margin-right:12px;flex-shrink:0;letter-spacing:0.06em;",
                                    tok().text_muted
                                )>"TIMESTAMP"</span>
                                <span style=move || format!(
                                    "width:72px;font-size:10px;font-weight:600;color:{};\
                                     margin-right:12px;flex-shrink:0;letter-spacing:0.06em;",
                                    tok().text_muted
                                )>"LEVEL"</span>
                                <span style=move || format!(
                                    "font-size:10px;font-weight:600;color:{};letter-spacing:0.06em;",
                                    tok().text_muted
                                )>"MESSAGE"</span>
                                <span style=move || format!(
                                    "margin-left:auto;font-size:10px;color:{};font-variant-numeric:tabular-nums;",
                                    tok().text_muted
                                )>
                                    {move || {
                                        let count = display_entries().len();
                                        let total = selected_file().map(|f| f.entries.len()).unwrap_or(0);
                                        if count == total { format!("{} entries", count) }
                                        else { format!("{} / {} entries", count, total) }
                                    }}
                                </span>
                            </div>

                            // Log rows
                            <div style="flex:1;overflow-y:auto;overflow-x:auto;font-family:'Fira Code',monospace;">
                                {move || {
                                    let entries = display_entries();
                                    if entries.is_empty() {
                                        return view! {
                                            <div style="display:flex;align-items:center;justify-content:center;height:80px">
                                                <span style=move || format!("font-size:13px;color:{}", tok().text_muted)>
                                                    "No entries match the current filters."
                                                </span>
                                            </div>
                                        }.into_any();
                                    }

                                    entries.into_iter().map(|de| {
                                        let level = de.entry.status.clone();
                                        let (lc, lb, lborder, accent) = level_colors(&level, dark());
                                        let show_badge = !level.is_empty() && !de.is_continuation;
                                        let row_bl = if accent != "transparent" && !de.is_continuation {
                                            format!("border-left:2px solid {}55;", accent)
                                        } else if de.is_continuation {
                                            // Continuation lines get a subtle indent line
                                            format!("border-left:2px solid {}22;", accent)
                                        } else {
                                            "border-left:2px solid transparent;".to_string()
                                        };

                                        // Continuation lines are slightly dimmed
                                        let msg_opacity = if de.is_continuation { "opacity:0.65;" } else { "" };

                                        view! {
                                            <div style=move || format!(
                                                "display:flex;align-items:baseline;padding:{}12px;\
                                                 border-bottom:1px solid {};{}cursor:default;",
                                                if de.is_continuation { "1px " } else { "2px " },
                                                tok().border_subtle,
                                                row_bl
                                            )>
                                                // Group line number (same for all lines in a group)
                                                <span style=move || format!(
                                                    "width:44px;text-align:right;font-size:11px;\
                                                     color:{};margin-right:12px;flex-shrink:0;\
                                                     user-select:none;font-variant-numeric:tabular-nums;",
                                                    if de.is_continuation { tok().border } else { tok().text_muted }
                                                )>
                                                    {if de.is_continuation { String::new() } else { de.group_line.to_string() }}
                                                </span>
                                                // Timestamp
                                                <span style=move || format!(
                                                    "width:160px;font-size:11px;color:{};\
                                                     margin-right:12px;flex-shrink:0;\
                                                     font-variant-numeric:tabular-nums;{}",
                                                    tok().text_secondary,
                                                    if de.is_continuation { "opacity:0.4;" } else { "" }
                                                )>{de.entry.datetime.clone()}</span>
                                                // Level badge (only on group leader)
                                                <span style="width:72px;margin-right:12px;flex-shrink:0;display:flex;align-items:center">
                                                    {if show_badge {
                                                        view! {
                                                            <span style=format!(
                                                                "display:inline-block;padding:1px 6px;\
                                                                 border-radius:4px;font-size:10px;\
                                                                 font-weight:700;letter-spacing:0.05em;\
                                                                 white-space:nowrap;line-height:1.4;\
                                                                 color:{lc};background:{lb};border:1px solid {lborder};"
                                                            )>{level.clone()}</span>
                                                        }.into_any()
                                                    } else {
                                                        view! { <span/> }.into_any()
                                                    }}
                                                </span>
                                                // Message
                                                <span style=move || format!(
                                                    "font-size:12px;color:{};line-height:1.6;\
                                                     word-break:break-all;user-select:text;{}",
                                                    tok().text_primary,
                                                    msg_opacity
                                                )>{de.entry.message}</span>
                                            </div>
                                        }
                                    }).collect_view().into_any()
                                }}
                            </div>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}
