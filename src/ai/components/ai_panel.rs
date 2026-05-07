/// AI Panel - Main orchestrator component.
/// 
/// Composes all AI panel sub-components and manages state.
/// This is the new modular version that replaces the 912-line monolithic file.

use super::{ChatMessages, InputArea, PanelHeader, SetupPanel};
use crate::ai::handlers::{create_logan_action_handler, create_send_handler};
use crate::ai::state::PanelState;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LoganAction, LogFile, Theme};
use leptos::prelude::*;

#[component]
pub fn AiPanel(
    theme: ReadSignal<Theme>,
    ai_width: ReadSignal<u32>,
    set_ai_width: WriteSignal<u32>,
    set_ai_open: WriteSignal<bool>,
    logan_action: ReadSignal<Option<LoganAction>>,
    set_logan_action: WriteSignal<Option<LoganAction>>,
    open_files: ReadSignal<Vec<LogFile>>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let state = PanelState::new();

    // Create send message handler
    let send_message = create_send_handler(
        state.input,
        state.set_input,
        state.context_chips,
        state.set_context_chips,
        state.mentioned_files,
        state.set_mentioned_files,
        state.loading,
        state.set_loading,
        state.api_key,
        state.provider,
        state.model,
        state.messages,
        state.set_messages,
        open_files,
    );

    // Create Logan action handler
    let send_msg_for_action = send_message.clone();
    create_logan_action_handler(
        logan_action,
        set_logan_action,
        state.set_context_chips,
        state.set_input,
        state.setup_done,
        state.set_setup_done,
        move || send_msg_for_action.run(()),
    );

    view! {
        <div
            style=move || format!(
                "width:{}px;flex-shrink:0;display:flex;flex-direction:column;\
                 background:{};position:relative;min-width:280px;max-width:640px;",
                ai_width.get(), tok().bg_surface
            )
            on:mousemove=move |ev| {
                if state.dragging.get() {
                    let delta = state.drag_start_x.get() - ev.client_x();
                    let new_w = (state.drag_start_w.get() as i32 + delta).max(280).min(640) as u32;
                    set_ai_width.set(new_w);
                }
            }
            on:mouseup=move |_| state.set_dragging.set(false)
            on:mouseleave=move |_| state.set_dragging.set(false)
        >
            // Resize handle
            <div
                style="position:absolute;left:0;top:0;bottom:0;width:4px;cursor:col-resize;z-index:10;"
                on:mousedown=move |ev| {
                    ev.prevent_default();
                    state.set_dragging.set(true);
                    state.set_drag_start_x.set(ev.client_x());
                    state.set_drag_start_w.set(ai_width.get());
                }
            />
            <div style=move || format!("position:absolute;left:0;top:0;bottom:0;width:1px;background:{};", tok().border)/>

            <PanelHeader
                theme=theme
                provider=state.provider
                setup_done=state.setup_done
                set_setup_done=state.set_setup_done
                set_ai_open=set_ai_open
            />

            <Show when=move || !state.setup_done.get()>
                <SetupPanel
                    theme=theme
                    provider=state.provider
                    set_provider=state.set_provider
                    api_key=state.api_key
                    set_api_key=state.set_api_key
                    model=state.model
                    set_model=state.set_model
                    set_setup_done=state.set_setup_done
                    set_messages=state.set_messages
                />
            </Show>

            <Show when=move || state.setup_done.get()>
                <ChatMessages
                    theme=theme
                    messages=state.messages
                    set_messages=state.set_messages
                    set_input=state.set_input
                    loading=state.loading
                    send_message=send_message
                />
            </Show>

            <Show when=move || state.setup_done.get()>
                <InputArea
                    theme=theme
                    input=state.input
                    set_input=state.set_input
                    loading=state.loading
                    set_loading=state.set_loading
                    provider=state.provider
                    model=state.model
                    context_chips=state.context_chips
                    set_context_chips=state.set_context_chips
                    mentioned_files=state.mentioned_files
                    set_mentioned_files=state.set_mentioned_files
                    mention_suggestions=state.mention_suggestions
                    set_mention_suggestions=state.set_mention_suggestions
                    mention_active=state.mention_active
                    set_mention_active=state.set_mention_active
                    mention_selected=state.mention_selected
                    set_mention_selected=state.set_mention_selected
                    open_files=open_files
                    send_message=send_message
                />
            </Show>
        </div>
    }
}
