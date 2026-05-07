/// AI panel header component.
/// 
/// Displays the Logan branding, provider badge, and control buttons.

use crate::ai::types::Provider;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::Theme;
use leptos::prelude::*;

#[component]
pub fn PanelHeader(
    theme: ReadSignal<Theme>,
    provider: ReadSignal<Provider>,
    setup_done: ReadSignal<bool>,
    set_setup_done: WriteSignal<bool>,
    set_ai_open: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    view! {
        <div style=move || format!(
            "display:flex;align-items:center;justify-content:space-between;\
             padding:0 14px;height:42px;border-bottom:1px solid {};flex-shrink:0;",
            tok().border
        )>
            <div style="display:flex;align-items:center;gap:8px">
                <div style="width:24px;height:24px;border-radius:7px;\
                    background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                    display:flex;align-items:center;justify-content:center;flex-shrink:0">
                    <img src="/public/LoganIcon.png" width="16" height="16" 
                         style="border-radius:3px;opacity:0.7" alt="Logan" />
                </div>
                <span style=move || format!("font-size:13px;font-weight:700;color:{}", tok().text_primary)>
                    "Logan"
                </span>
                <span style=move || format!(
                    "font-size:9px;padding:2px 6px;border-radius:8px;font-weight:700;\
                     letter-spacing:0.06em;background:{};color:{};",
                    tok().accent_bg, tok().accent
                )>"AI"</span>
                {move || if setup_done.get() {
                    view! {
                        <span style=move || format!(
                            "font-size:10px;color:{};padding:1px 6px;border-radius:5px;\
                             background:{};border:1px solid {};",
                            tok().text_muted, tok().bg_input, tok().border
                        )>{provider.get().label()}</span>
                    }.into_any()
                } else { view! { <span/> }.into_any() }}
            </div>
            <div style="display:flex;align-items:center;gap:4px">
                <button
                    style=move || format!(
                        "width:26px;height:26px;display:flex;align-items:center;justify-content:center;\
                         border-radius:5px;border:none;background:{};color:{};cursor:pointer;",
                        if !setup_done.get() { tok().accent_bg } else { tok().bg_input },
                        if !setup_done.get() { tok().accent } else { tok().text_muted }
                    )
                    on:click=move |_| set_setup_done.update(|v| *v = !*v)
                    title="Provider settings"
                >
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872l-.1-.34zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z"/>
                    </svg>
                </button>
                <button
                    style=move || format!(
                        "width:26px;height:26px;display:flex;align-items:center;justify-content:center;\
                         border-radius:5px;border:none;background:{};color:{};cursor:pointer;",
                        tok().bg_input, tok().text_muted
                    )
                    on:click=move |_| set_ai_open.set(false)
                    title="Close"
                >
                    <svg width="9" height="9" viewBox="0 0 10 10" fill="currentColor">
                        <path d="M1.707.293A1 1 0 0 0 .293 1.707L3.586 5 .293 8.293a1 1 0 1 0 1.414 1.414L5 6.414l3.293 3.293a1 1 0 0 0 1.414-1.414L6.414 5l3.293-3.293A1 1 0 0 0 8.293.293L5 3.586 1.707.293z"/>
                    </svg>
                </button>
            </div>
        </div>
    }
}
