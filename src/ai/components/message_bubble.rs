/// Single message bubble component.
/// 
/// Renders a chat message with copy and retry buttons.

use crate::ai::types::ChatMessage;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::Theme;
use leptos::prelude::*;

#[component]
pub fn MessageBubble(
    theme: ReadSignal<Theme>,
    message: ChatMessage,
    index: usize,
    messages: ReadSignal<Vec<ChatMessage>>,
    set_messages: WriteSignal<Vec<ChatMessage>>,
    set_input: WriteSignal<String>,
    send_message: Callback<()>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let is_user = message.role == "user";
    let is_error = message.role == "error";
    let content = message.content.clone();
    let content_copy = content.clone();

    view! {
        <div style=if is_user { "display:flex;flex-direction:column;align-items:flex-end;gap:4px;" }
                   else       { "display:flex;flex-direction:column;align-items:flex-start;gap:4px;" }>
            <div style=move || {
                let t = tok();
                if is_error {
                    "max-width:92%;padding:9px 12px;border-radius:10px;\
                     font-size:12px;line-height:1.55;\
                     background:#f8717118;color:#fca5a5;border:1px solid #f8717130;".to_string()
                } else if is_user {
                    format!("max-width:80%;padding:9px 12px;border-radius:10px 10px 3px 10px;\
                             font-size:12px;line-height:1.55;\
                             background:linear-gradient(135deg,{},#a78bfa);color:white;",
                        t.accent)
                } else {
                    format!("max-width:92%;padding:9px 12px;border-radius:10px 10px 10px 3px;\
                             font-size:12px;line-height:1.55;\
                             background:{};border:1px solid {};color:{};",
                        t.bg_elevated, t.border, t.text_primary)
                }
            } inner_html=move || {
                use crate::markdown::md_to_html;
                let is_dark = theme.get() == Theme::Dark;
                if is_user || is_error {
                    format!("<div style='white-space:pre-wrap'>{}</div>", 
                        content.replace('&', "&amp;")
                               .replace('<', "&lt;")
                               .replace('>', "&gt;"))
                } else {
                    md_to_html(&content, is_dark)
                }
            }>
            </div>
            
            <div style="display:flex;gap:4px;align-items:center;">
                <button
                    style=move || format!(
                        "padding:3px 7px;border-radius:4px;border:1px solid {};\
                         background:{};color:{};font-size:10px;cursor:pointer;\
                         display:flex;align-items:center;gap:4px;opacity:0.7;\
                         transition:opacity 0.15s;",
                        tok().border, tok().bg_input, tok().text_muted
                    )
                    on:click=move |_| {
                        let js_code = format!(
                            "navigator.clipboard.writeText(`{}`)",
                            content_copy.replace('`', "\\`").replace('\\', "\\\\")
                        );
                        let _ = js_sys::eval(&js_code);
                    }
                    title="Copy message"
                >
                    <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"/>
                        <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"/>
                    </svg>
                    "Copy"
                </button>
                
                {if !is_user && !is_error {
                    view! {
                        <button
                            style=move || format!(
                                "padding:3px 7px;border-radius:4px;border:1px solid {};\
                                 background:{};color:{};font-size:10px;cursor:pointer;\
                                 display:flex;align-items:center;gap:4px;opacity:0.7;\
                                 transition:opacity 0.15s;",
                                tok().border, tok().bg_input, tok().text_muted
                            )
                            on:click=move |_| {
                                let msgs = messages.get();
                                if index > 0 {
                                    if let Some(prev_msg) = msgs.get(index - 1) {
                                        if prev_msg.role == "user" {
                                            set_messages.update(|m| m.truncate(index - 1));
                                            set_input.set(prev_msg.content.clone());
                                            send_message.run(());
                                        }
                                    }
                                }
                            }
                            title="Retry this request"
                        >
                            <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor">
                                <path fill-rule="evenodd" d="M8 3a5 5 0 1 0 4.546 2.914.5.5 0 0 1 .908-.417A6 6 0 1 1 8 2v1z"/>
                                <path d="M8 4.466V.534a.25.25 0 0 1 .41-.192l2.36 1.966c.12.1.12.284 0 .384L8.41 4.658A.25.25 0 0 1 8 4.466z"/>
                            </svg>
                            "Retry"
                        </button>
                    }.into_any()
                } else {
                    view! { <span/> }.into_any()
                }}
            </div>
        </div>
    }
}
