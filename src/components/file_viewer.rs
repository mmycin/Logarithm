use crate::app::{FilterState, LoganAction, LogEntry, LogFile, Theme, DARK, LIGHT};
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
                    <img src="/public/StoreLogo.png" width="32" height="32" style="border-radius:6px;opacity:0.9" alt="Logarithm" />
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
    set_logan_action: WriteSignal<Option<LoganAction>>,
    set_ai_open: WriteSignal<bool>,
) -> impl IntoView {
    let tok  = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let dark = move || theme.get() == Theme::Dark;

    // Context menu state
    let (ctx_visible, set_ctx_visible) = signal(false);
    let (ctx_x, set_ctx_x)             = signal(0i32);
    let (ctx_y, set_ctx_y)             = signal(0i32);
    let (ctx_entry, set_ctx_entry)     = signal(None::<(String, usize, String)>); // (filename, line, message)
    
    // Focused line state
    let (focused_line, set_focused_line) = signal(None::<usize>);
    
    // Multi-select state
    let (selected_lines, set_selected_lines) = signal(Vec::<usize>::new());

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
            "flex:1;display:flex;flex-direction:column;background:{};overflow:hidden;position:relative;",
            tok().bg_base
        )
        // Close context menu on any click in the viewer
        on:click=move |_| set_ctx_visible.set(false)
        on:contextmenu=move |ev| ev.prevent_default()
        >
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
                                        let is_focused = move || focused_line.get() == Some(de.group_line);
                                        let is_selected = move || selected_lines.get().contains(&de.group_line);
                                        
                                        // Enhanced styling with box and colors
                                        let row_bg = if !de.is_continuation && accent != "transparent" {
                                            format!("background:linear-gradient(90deg, {}08 0%, transparent 100%);", accent)
                                        } else {
                                            String::new()
                                        };
                                        
                                        let row_bl = if accent != "transparent" && !de.is_continuation {
                                            format!("border-left:3px solid {};", accent)
                                        } else if de.is_continuation {
                                            format!("border-left:3px solid {}22;", accent)
                                        } else {
                                            "border-left:3px solid transparent;".to_string()
                                        };
                                        
                                        let msg_opacity = if de.is_continuation { "opacity:0.65;" } else { "" };

                                        // For context menu
                                        let file_name = selected_file()
                                            .map(|f| {
                                                // Shorten: take last segment before extension
                                                let n = &f.name;
                                                n.rsplit('/').next()
                                                 .or_else(|| n.rsplit('\\').next())
                                                 .unwrap_or(n)
                                                 .to_string()
                                            })
                                            .unwrap_or_default();
                                        let entry_line = de.group_line;
                                        let entry_msg  = de.entry.message.clone();

                                        view! {
                                            <div
                                                style=move || {
                                                    let base = format!(
                                                        "display:flex;align-items:baseline;padding:{}12px;\
                                                         border-bottom:1px solid {};{}{}cursor:pointer;\
                                                         transition:all 0.15s ease;",
                                                        if de.is_continuation { "1px " } else { "2px " },
                                                        tok().border_subtle,
                                                        row_bl,
                                                        row_bg
                                                    );
                                                    if is_selected() {
                                                        format!("{}background:{};box-shadow:inset 0 0 0 2px {};",
                                                            base, tok().accent_bg, tok().accent_border)
                                                    } else if is_focused() {
                                                        format!("{}background:{};box-shadow:inset 0 0 0 1px {};",
                                                            base, tok().bg_elevated, tok().border)
                                                    } else {
                                                        base
                                                    }
                                                }
                                                class="log-row"
                                                on:click=move |ev| {
                                                    if ev.ctrl_key() {
                                                        // Ctrl+Click: toggle selection
                                                        set_selected_lines.update(|lines| {
                                                            if let Some(pos) = lines.iter().position(|&l| l == entry_line) {
                                                                lines.remove(pos);
                                                            } else {
                                                                lines.push(entry_line);
                                                            }
                                                        });
                                                    } else {
                                                        // Normal click: focus and clear selection
                                                        set_focused_line.set(Some(entry_line));
                                                        set_selected_lines.set(Vec::new());
                                                    }
                                                }
                                                on:contextmenu=move |ev| {
                                                    ev.prevent_default();
                                                    ev.stop_propagation();
                                                    set_ctx_x.set(ev.client_x());
                                                    set_ctx_y.set(ev.client_y());
                                                    set_ctx_entry.set(Some((
                                                        file_name.clone(),
                                                        entry_line,
                                                        entry_msg.clone(),
                                                    )));
                                                    set_ctx_visible.set(true);
                                                }
                                            >
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

            // ── Context menu ──────────────────────────────────────────────
            <Show when=move || ctx_visible.get()>
                <div
                    style=move || {
                        // Clamp to viewport: menu is ~200px wide, ~100px tall
                        let win_w = web_sys::window()
                            .and_then(|w| w.inner_width().ok())
                            .and_then(|v| v.as_f64())
                            .unwrap_or(1200.0) as i32;
                        let win_h = web_sys::window()
                            .and_then(|w| w.inner_height().ok())
                            .and_then(|v| v.as_f64())
                            .unwrap_or(800.0) as i32;
                        let x = (ctx_x.get()).min(win_w - 210);
                        let y = (ctx_y.get()).min(win_h - 120);
                        format!(
                            "position:fixed;left:{}px;top:{}px;z-index:500;\
                             background:{};border:1px solid {};border-radius:8px;\
                             box-shadow:0 8px 32px rgba(0,0,0,0.5);padding:4px 0;\
                             min-width:200px;overflow:hidden;",
                            x, y, tok().bg_elevated, tok().border
                        )
                    }
                    on:click=|ev| ev.stop_propagation()
                >
                    // Context chip header
                    {move || {
                        let sel_count = selected_lines.get().len();
                        if sel_count > 0 {
                            view! {
                                <div style=move || format!(
                                    "padding:6px 12px 5px;font-size:10px;font-weight:700;\
                                     color:{};letter-spacing:0.07em;text-transform:uppercase;\
                                     border-bottom:1px solid {};margin-bottom:2px;",
                                    tok().text_muted, tok().border
                                )>
                                    {format!("{} lines selected", sel_count)}
                                </div>
                            }.into_any()
                        } else {
                            ctx_entry.get().map(|(fname, lnum, _)| view! {
                                <div style=move || format!(
                                    "padding:6px 12px 5px;font-size:10px;font-weight:700;\
                                     color:{};letter-spacing:0.07em;text-transform:uppercase;\
                                     border-bottom:1px solid {};margin-bottom:2px;",
                                    tok().text_muted, tok().border
                                )>
                                    {format!("{}:{}", fname, lnum)}
                                </div>
                            }).into_any()
                        }
                    }}

                    // Copy line(s)
                    <button
                        style=move || format!(
                            "display:flex;align-items:center;gap:9px;width:100%;text-align:left;\
                             padding:7px 12px;font-size:12.5px;color:{};background:transparent;\
                             border:none;cursor:pointer;transition:background 0.08s;",
                            tok().text_primary
                        )
                        on:click=move |_| {
                            let sel = selected_lines.get();
                            let text_to_copy = if !sel.is_empty() {
                                // Copy all selected lines
                                let entries = display_entries();
                                let mut lines_text = Vec::new();
                                for line_num in sel {
                                    if let Some(de) = entries.iter().find(|e| e.group_line == line_num) {
                                        lines_text.push(format!("[Line {}] {}", line_num, de.entry.message));
                                    }
                                }
                                lines_text.join("\n")
                            } else if let Some((_, lnum, msg)) = ctx_entry.get() {
                                // Copy single line
                                format!("[Line {}] {}", lnum, msg)
                            } else {
                                String::new()
                            };
                            
                            if !text_to_copy.is_empty() {
                                let js_code = format!(
                                    "navigator.clipboard.writeText(`{}`)",
                                    text_to_copy.replace('`', "\\`").replace('\\', "\\\\")
                                );
                                let _ = js_sys::eval(&js_code);
                            }
                            set_ctx_visible.set(false);
                        }
                    >
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.55;flex-shrink:0">
                            <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"/>
                            <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"/>
                        </svg>
                        {move || {
                            let sel_count = selected_lines.get().len();
                            if sel_count > 0 {
                                format!("Copy {} lines", sel_count)
                            } else {
                                "Copy line".to_string()
                            }
                        }}
                    </button>

                    // Add to Logan context (single or multiple)
                    <button
                        style=move || format!(
                            "display:flex;align-items:center;gap:9px;width:100%;text-align:left;\
                             padding:7px 12px;font-size:12.5px;color:{};background:transparent;\
                             border:none;cursor:pointer;transition:background 0.08s;",
                            tok().text_primary
                        )
                        on:click=move |_| {
                            let sel = selected_lines.get();
                            if !sel.is_empty() {
                                // Add all selected lines to context at once
                                let entries = display_entries();
                                let file_name = selected_file()
                                    .map(|f| {
                                        let n = &f.name;
                                        n.rsplit('/').next()
                                         .or_else(|| n.rsplit('\\').next())
                                         .unwrap_or(n)
                                         .to_string()
                                    })
                                    .unwrap_or_default();
                                
                                let mut items = Vec::new();
                                for line_num in sel {
                                    if let Some(de) = entries.iter().find(|e| e.group_line == line_num) {
                                        items.push((file_name.clone(), line_num, de.entry.message.clone()));
                                    }
                                }
                                
                                if !items.is_empty() {
                                    set_logan_action.set(Some(LoganAction::AddMultipleContext { items }));
                                }
                                
                                set_selected_lines.set(Vec::new());
                                set_ai_open.set(true);
                            } else if let Some((fname, lnum, msg)) = ctx_entry.get() {
                                set_logan_action.set(Some(LoganAction::AddContext {
                                    file: fname, line: lnum, text: msg,
                                }));
                                set_ai_open.set(true);
                            }
                            set_ctx_visible.set(false);
                        }
                    >
                        <img src="/public/LoganIcon.png" width="13" height="13" style="border-radius:2px;opacity:0.6" alt="Logan" />
                        "Add to Logan context"
                    </button>

                    // Explain with AI
                    <button
                        style=move || format!(
                            "display:flex;align-items:center;gap:9px;width:100%;text-align:left;\
                             padding:7px 12px;font-size:12.5px;color:{};background:transparent;\
                             border:none;cursor:pointer;transition:background 0.08s;",
                            tok().text_primary
                        )
                        on:click=move |_| {
                            if let Some((fname, lnum, msg)) = ctx_entry.get() {
                                set_logan_action.set(Some(LoganAction::Explain {
                                    file: fname, line: lnum, text: msg,
                                }));
                                set_ai_open.set(true);
                            }
                            set_ctx_visible.set(false);
                        }
                    >
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.55;flex-shrink:0">
                            <path d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"/>
                        </svg>
                        "Explain with AI"
                    </button>
                </div>
            </Show>
        </div>
    }
}
