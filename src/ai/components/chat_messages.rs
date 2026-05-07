/// Chat messages list component.
/// 
/// Displays all chat messages and loading indicator.

use crate::ai::components::message_bubble::MessageBubble;
use crate::ai::types::ChatMessage;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::Theme;
use leptos::prelude::*;

#[component]
pub fn ChatMessages(
    theme: ReadSignal<Theme>,
    messages: ReadSignal<Vec<ChatMessage>>,
    set_messages: WriteSignal<Vec<ChatMessage>>,
    set_input: WriteSignal<String>,
    loading: ReadSignal<bool>,
    send_message: Callback<()>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    view! {
        <div style="flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:10px;">
            {move || messages.get().into_iter().enumerate().map(|(idx, msg)| {
                let send_msg_clone = send_message.clone();
                view! {
                    <MessageBubble
                        theme=theme
                        message=msg
                        index=idx
                        messages=messages
                        set_messages=set_messages
                        set_input=set_input
                        send_message=send_msg_clone
                    />
                }
            }).collect_view()}

            <Show when=move || loading.get()>
                <div style="display:flex;justify-content:flex-start;">
                    <div style=move || format!(
                        "padding:9px 14px;border-radius:10px 10px 10px 3px;\
                         background:{};border:1px solid {};display:flex;align-items:center;gap:6px;",
                        tok().bg_elevated, tok().border
                    )>
                        <span style=move || format!("font-size:11px;color:{}", tok().text_muted)>
                            "Logan is thinking"
                        </span>
                        <span style="display:flex;gap:3px;align-items:center">
                            <span style="width:4px;height:4px;border-radius:50%;background:#7c9dff;animation:pulse 1.2s ease-in-out infinite;"/>
                            <span style="width:4px;height:4px;border-radius:50%;background:#7c9dff;animation:pulse 1.2s ease-in-out 0.25s infinite;"/>
                            <span style="width:4px;height:4px;border-radius:50%;background:#7c9dff;animation:pulse 1.2s ease-in-out 0.5s infinite;"/>
                        </span>
                    </div>
                </div>
            </Show>
        </div>
    }
}
