/// Context menu component for log entries.
/// 
/// Provides copy, add to context, and explain actions for log lines.

use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LoganAction, LogFile, Theme};
use crate::viewer::types::DisplayEntry;
use leptos::prelude::*;

#[component]
pub fn ContextMenu(
    theme: ReadSignal<Theme>,
    ctx_x: ReadSignal<i32>,
    ctx_y: ReadSignal<i32>,
    ctx_entry: ReadSignal<Option<(String, usize, String)>>,
    selected_lines: ReadSignal<Vec<usize>>,
    set_selected_lines: WriteSignal<Vec<usize>>,
    set_ctx_visible: WriteSignal<bool>,
    display_entries: impl Fn() -> Vec<DisplayEntry> + 'static + Copy + Send,
    selected_file: impl Fn() -> Option<LogFile> + 'static + Copy + Send,
    set_logan_action: WriteSignal<Option<LoganAction>>,
    set_ai_open: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    view! {
        <div
            style=move || {
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
            // Header
            <MenuHeader theme=theme ctx_entry=ctx_entry selected_lines=selected_lines />
            
            // Copy button
            <CopyButton
                theme=theme
                selected_lines=selected_lines
                ctx_entry=ctx_entry
                display_entries=display_entries
                set_ctx_visible=set_ctx_visible
            />
            
            // Add to context button
            <AddToContextButton
                theme=theme
                selected_lines=selected_lines
                set_selected_lines=set_selected_lines
                ctx_entry=ctx_entry
                display_entries=display_entries
                selected_file=selected_file
                set_logan_action=set_logan_action
                set_ai_open=set_ai_open
                set_ctx_visible=set_ctx_visible
            />
            
            // Explain button
            <ExplainButton
                theme=theme
                ctx_entry=ctx_entry
                set_logan_action=set_logan_action
                set_ai_open=set_ai_open
                set_ctx_visible=set_ctx_visible
            />
        </div>
    }
}

#[component]
fn MenuHeader(
    theme: ReadSignal<Theme>,
    ctx_entry: ReadSignal<Option<(String, usize, String)>>,
    selected_lines: ReadSignal<Vec<usize>>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
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
    }
}

#[component]
fn CopyButton(
    theme: ReadSignal<Theme>,
    selected_lines: ReadSignal<Vec<usize>>,
    ctx_entry: ReadSignal<Option<(String, usize, String)>>,
    display_entries: impl Fn() -> Vec<DisplayEntry> + 'static + Copy + Send,
    set_ctx_visible: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
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
                    let entries = display_entries();
                    let mut lines_text = Vec::new();
                    for line_num in sel {
                        if let Some(de) = entries.iter().find(|e| e.group_line == line_num) {
                            lines_text.push(format!("[Line {}] {}", line_num, de.entry.message));
                        }
                    }
                    lines_text.join("\n")
                } else if let Some((_, lnum, msg)) = ctx_entry.get() {
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
    }
}

#[component]
fn AddToContextButton(
    theme: ReadSignal<Theme>,
    selected_lines: ReadSignal<Vec<usize>>,
    set_selected_lines: WriteSignal<Vec<usize>>,
    ctx_entry: ReadSignal<Option<(String, usize, String)>>,
    display_entries: impl Fn() -> Vec<DisplayEntry> + 'static + Copy + Send,
    selected_file: impl Fn() -> Option<LogFile> + 'static + Copy + Send,
    set_logan_action: WriteSignal<Option<LoganAction>>,
    set_ai_open: WriteSignal<bool>,
    set_ctx_visible: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
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
                    let entries = display_entries();
                    let file_name = selected_file()
                        .map(|f| {
                            f.name
                                .rsplit('/')
                                .next()
                                .or_else(|| f.name.rsplit('\\').next())
                                .unwrap_or(&f.name)
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
                        file: fname,
                        line: lnum,
                        text: msg,
                    }));
                    set_ai_open.set(true);
                }
                set_ctx_visible.set(false);
            }
        >
            <img src="/public/LoganIcon.png" width="13" height="13" style="border-radius:2px;opacity:0.6" alt="Logan" />
            "Add to Logan context"
        </button>
    }
}

#[component]
fn ExplainButton(
    theme: ReadSignal<Theme>,
    ctx_entry: ReadSignal<Option<(String, usize, String)>>,
    set_logan_action: WriteSignal<Option<LoganAction>>,
    set_ai_open: WriteSignal<bool>,
    set_ctx_visible: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
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
                        file: fname,
                        line: lnum,
                        text: msg,
                    }));
                    set_ai_open.set(true);
                }
                set_ctx_visible.set(false);
            }
        >
            <svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.55;flex-shrink:0">
                <path d="M8 15A7 7 0 1 1 8 1a7 7 0 0 1 0 14zm0 1A8 8 0 1 0 8 0a8 8 0 0 0 0 16z"/>
                <path d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"/>
            </svg>
            "Explain with Logan AI"
        </button>
    }
}
