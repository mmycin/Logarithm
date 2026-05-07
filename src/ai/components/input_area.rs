/// Input area component with context chips, mentions, and autocomplete.
/// 
/// Handles user input, @ mentions, context chips, and send button.

use crate::ai::handlers::mention::{handle_mention_input, handle_mention_keydown, select_mention};
use crate::ai::types::Provider;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LogFile, Theme};
use leptos::prelude::*;

#[component]
pub fn InputArea(
    theme: ReadSignal<Theme>,
    input: ReadSignal<String>,
    set_input: WriteSignal<String>,
    loading: ReadSignal<bool>,
    set_loading: WriteSignal<bool>,
    provider: ReadSignal<Provider>,
    model: ReadSignal<String>,
    context_chips: ReadSignal<Vec<(String, usize, String)>>,
    set_context_chips: WriteSignal<Vec<(String, usize, String)>>,
    mentioned_files: ReadSignal<Vec<String>>,
    set_mentioned_files: WriteSignal<Vec<String>>,
    mention_suggestions: ReadSignal<Vec<String>>,
    set_mention_suggestions: WriteSignal<Vec<String>>,
    mention_active: ReadSignal<bool>,
    set_mention_active: WriteSignal<bool>,
    mention_selected: ReadSignal<usize>,
    set_mention_selected: WriteSignal<usize>,
    open_files: ReadSignal<Vec<LogFile>>,
    send_message: Callback<()>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    view! {
        <div style=move || format!(
            "padding:10px 12px;border-top:1px solid {};flex-shrink:0;position:relative;", tok().border
        )>
            // Context chips
            <Show when=move || !context_chips.get().is_empty()>
                <div style="display:flex;flex-wrap:wrap;gap:6px;margin-bottom:8px;">
                    {move || context_chips.get().into_iter().enumerate().map(|(idx, (file, line, _text))| {
                        view! {
                            <div style=move || format!(
                                "display:flex;align-items:center;gap:6px;padding:4px 10px;\
                                 background:{};border:1px solid {};border-radius:6px;\
                                 font-size:11px;color:{};font-family:'Fira Code',monospace;\
                                 font-weight:600;",
                                tok().bg_elevated, tok().accent_border, tok().accent
                            )>
                                <img src="/public/LoganIcon.png" width="10" height="10" style="border-radius:2px;opacity:0.6" alt="Logan" />
                                <span>{format!("{}:{}", file, line)}</span>
                                <button
                                    style=move || format!(
                                        "margin-left:4px;width:16px;height:16px;border-radius:3px;\
                                         border:none;background:{};color:{};cursor:pointer;\
                                         display:flex;align-items:center;justify-content:center;\
                                         font-size:10px;line-height:1;",
                                        tok().bg_input, tok().text_muted
                                    )
                                    on:click=move |_| {
                                        set_context_chips.update(|chips| {
                                            chips.remove(idx);
                                        });
                                    }
                                    title="Remove context"
                                >"×"</button>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </Show>
            
            // Mentioned files chips
            <Show when=move || !mentioned_files.get().is_empty()>
                <div style="display:flex;flex-wrap:wrap;gap:6px;margin-bottom:8px;">
                    {move || mentioned_files.get().into_iter().enumerate().map(|(idx, file_name)| {
                        view! {
                            <div style=move || format!(
                                "display:flex;align-items:center;gap:6px;padding:4px 10px;\
                                 background:{};border:1px solid {};border-radius:6px;\
                                 font-size:11px;color:{};font-family:'Fira Code',monospace;\
                                 font-weight:600;",
                                tok().bg_elevated, tok().border, tok().text_secondary
                            )>
                                <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5">
                                    <path d="M4 0h5.293A1 1 0 0 1 10 .293L13.707 4a1 1 0 0 1 .293.707V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2zm5.5 1.5v2a1 1 0 0 0 1 1h2L9.5 1.5z"/>
                                </svg>
                                <span>{"@"}{file_name.clone()}</span>
                                <button
                                    style=move || format!(
                                        "margin-left:4px;width:16px;height:16px;border-radius:3px;\
                                         border:none;background:{};color:{};cursor:pointer;\
                                         display:flex;align-items:center;justify-content:center;\
                                         font-size:10px;line-height:1;",
                                        tok().bg_input, tok().text_muted
                                    )
                                    on:click=move |_| {
                                        set_mentioned_files.update(|files| {
                                            files.remove(idx);
                                        });
                                    }
                                    title="Remove file"
                                >"×"</button>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </Show>
            
            <div style=move || format!(
                "display:flex;align-items:flex-end;gap:8px;padding:8px 10px;\
                 background:{};border:1px solid {};border-radius:10px;position:relative;",
                tok().bg_input, tok().border
            )>
                // Autocomplete dropdown
                <Show when=move || mention_active.get()>
                    <div style=move || format!(
                        "position:absolute;bottom:calc(100% + 4px);left:0;right:0;\
                         background:{};border:1px solid {};border-radius:8px;\
                         box-shadow:0 4px 12px rgba(0,0,0,0.3);max-height:200px;\
                         overflow-y:auto;z-index:100;",
                        tok().bg_elevated, tok().border
                    )>
                        {move || mention_suggestions.get().into_iter().enumerate().map(|(idx, file_name)| {
                            let is_selected = mention_selected.get() == idx;
                            let fname = file_name.clone();
                            view! {
                                <div
                                    style=move || format!(
                                        "padding:8px 12px;cursor:pointer;font-size:12px;\
                                         color:{};background:{};transition:background 0.1s;\
                                         display:flex;align-items:center;gap:8px;",
                                        tok().text_primary,
                                        if is_selected { tok().accent_bg } else { "transparent" }
                                    )
                                    on:click=move |_| {
                                        select_mention(fname.clone(), input, set_input, set_mentioned_files, set_mention_active);
                                    }
                                >
                                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                        <path d="M4 0h5.293A1 1 0 0 1 10 .293L13.707 4a1 1 0 0 1 .293.707V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2zm5.5 1.5v2a1 1 0 0 0 1 1h2L9.5 1.5z"/>
                                    </svg>
                                    {file_name}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </Show>
                
                <textarea
                    style=move || format!(
                        "flex:1;border:none;background:transparent;font-size:12.5px;\
                         color:{};outline:none;font-family:'Inter',sans-serif;\
                         resize:none;min-height:20px;max-height:120px;line-height:1.5;cursor:text;",
                        tok().text_primary
                    )
                    placeholder="Ask about your logs (@ to mention files)"
                    rows="1"
                    prop:value=move || input.get()
                    on:input=move |ev| {
                        let val = event_target_value(&ev);
                        set_input.set(val.clone());
                        handle_mention_input(&val, &open_files.get(), set_mention_suggestions, set_mention_active, set_mention_selected);
                    }
                    on:keydown=move |ev| {
                        if mention_active.get() {
                            let suggestions = mention_suggestions.get();
                            let selected = mention_selected.get();
                            
                            if handle_mention_keydown(&ev.key(), &suggestions, selected, set_mention_selected) {
                                ev.prevent_default();
                            }
                            
                            if (ev.key() == "Enter" || ev.key() == "Tab") && !suggestions.is_empty() && selected < suggestions.len() {
                                ev.prevent_default();
                                let file_name = suggestions[selected].clone();
                                select_mention(file_name, input, set_input, set_mentioned_files, set_mention_active);
                            } else if ev.key() == "Escape" {
                                set_mention_active.set(false);
                            }
                        }
                        
                        if ev.key() == "Enter" && !ev.shift_key() && !mention_active.get() {
                            ev.prevent_default();
                            send_message.run(());
                        }
                    }
                />
                
                <button
                    style=move || {
                        let is_loading = loading.get();
                        if is_loading {
                            "width:28px;height:28px;border-radius:7px;border:none;\
                             background:#f87171;\
                             display:flex;align-items:center;justify-content:center;\
                             cursor:pointer;flex-shrink:0;transition:all 0.15s;".to_string()
                        } else {
                            let ready = !input.get().trim().is_empty();
                            if ready {
                                "width:28px;height:28px;border-radius:7px;border:none;\
                                 background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                                 display:flex;align-items:center;justify-content:center;\
                                 cursor:pointer;flex-shrink:0;transition:opacity 0.1s;".to_string()
                            } else {
                                "width:28px;height:28px;border-radius:7px;border:none;\
                                 background:rgba(124,157,255,0.15);\
                                 display:flex;align-items:center;justify-content:center;\
                                 cursor:not-allowed;flex-shrink:0;opacity:0.4;".to_string()
                            }
                        }
                    }
                    on:click=move |_| {
                        if loading.get() {
                            set_loading.set(false);
                        } else {
                            send_message.run(());
                        }
                    }
                    title=move || if loading.get() { "Stop generation" } else { "Send (Enter)" }
                >
                    {move || {
                        if loading.get() {
                            view! {
                                <svg width="10" height="10" viewBox="0 0 16 16" fill="white">
                                    <rect width="16" height="16" rx="2"/>
                                </svg>
                            }.into_any()
                        } else {
                            view! {
                                <svg width="12" height="12" viewBox="0 0 16 16" fill="white">
                                    <path d="M15.964.686a.5.5 0 0 0-.65-.65L.767 5.855H.766l-.452.18a.5.5 0 0 0-.082.887l.41.26.001.002 4.995 3.178 3.178 4.995.002.002.26.41a.5.5 0 0 0 .886-.083l6-15Zm-1.833 1.89L6.637 10.07l-.215-.338a.5.5 0 0 0-.154-.154l-.338-.215 7.494-7.494 1.178-.471-.47 1.178Z"/>
                                </svg>
                            }.into_any()
                        }
                    }}
                </button>
            </div>
            
            <div style=move || format!(
                "font-size:10px;color:{};text-align:center;margin-top:5px;", tok().text_muted
            )>
                {move || {
                    let m = model.get();
                    let mdl = if m.is_empty() { provider.get().default_model().to_string() } else { m };
                    format!("{} · {} · Enter to send  Shift+Enter for newline", provider.get().label(), mdl)
                }}
            </div>
        </div>
    }
}
