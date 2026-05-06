use crate::app::{Theme, DARK, LIGHT};
use leptos::prelude::*;

#[component]
pub fn AiPanel(
    theme: ReadSignal<Theme>,
    ai_width: ReadSignal<u32>,
    set_ai_width: WriteSignal<u32>,
    set_ai_open: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    let (dragging, set_dragging) = signal(false);
    let (drag_start_x, set_drag_start_x) = signal(0i32);
    let (drag_start_w, set_drag_start_w) = signal(0u32);

    view! {
        <div
            style=move || format!(
                "width:{}px;flex-shrink:0;display:flex;flex-direction:column;\
                 background:{};position:relative;min-width:260px;max-width:600px;",
                ai_width.get(), tok().bg_surface
            )
            on:mousemove=move |ev| {
                if dragging.get() {
                    let delta = drag_start_x.get() - ev.client_x();
                    let new_w = (drag_start_w.get() as i32 + delta).max(260).min(600) as u32;
                    set_ai_width.set(new_w);
                }
            }
            on:mouseup=move |_| set_dragging.set(false)
        >
            // Resize handle (left edge)
            <div
                style="position:absolute;left:0;top:0;bottom:0;width:5px;cursor:col-resize;z-index:10;"
                on:mousedown=move |ev| {
                    ev.prevent_default();
                    set_dragging.set(true);
                    set_drag_start_x.set(ev.client_x());
                    set_drag_start_w.set(ai_width.get());
                }
            />
            // borderline
            <div style=move || format!(
                "position:absolute;left:0;top:0;bottom:0;width:1px;background:{};",
                tok().border
            )/>

            // Header
            <div style=move || format!(
                "display:flex;align-items:center;justify-content:space-between;\
                 padding:0 14px;height:40px;border-bottom:1px solid {};flex-shrink:0;",
                tok().border
            )>
                <div style="display:flex;align-items:center;gap:8px">
                    // Logar logo mark
                    <div style="width:24px;height:24px;border-radius:7px;\
                        background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                        display:flex;align-items:center;justify-content:center;flex-shrink:0;\
                        box-shadow:0 2px 8px rgba(124,157,255,0.35)">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="white">
                            <path d="M7.657 6.247c.11-.33.576-.33.686 0l.645 1.937a2.89 2.89 0 0 0 1.829 1.828l1.936.645c.33.11.33.576 0 .686l-1.937.645a2.89 2.89 0 0 0-1.828 1.829l-.645 1.936a.361.361 0 0 1-.686 0l-.645-1.937a2.89 2.89 0 0 0-1.828-1.828l-1.937-.645a.361.361 0 0 1 0-.686l1.937-.645a2.89 2.89 0 0 0 1.828-1.828l.645-1.937z"/>
                        </svg>
                    </div>
                    <span style=move || format!(
                        "font-size:14px;font-weight:700;color:{};letter-spacing:-0.01em;",
                        tok().text_primary
                    )>"Logar"</span>
                    <span style="font-size:9px;padding:2px 6px;border-radius:8px;\
                        background:linear-gradient(135deg,rgba(124,157,255,0.2),rgba(167,139,250,0.2));\
                        color:#a78bfa;font-weight:700;letter-spacing:0.06em;border:1px solid rgba(167,139,250,0.25)">
                        "AI"
                    </span>
                </div>
                <button
                    style=move || format!(
                        "width:26px;height:26px;display:flex;align-items:center;\
                         justify-content:center;border-radius:5px;border:none;\
                         background:{};color:{};cursor:pointer;font-size:13px;",
                        tok().bg_input, tok().text_muted
                    )
                    on:click=move |_| set_ai_open.set(false)
                    title="Close Logar AI"
                >"✕"</button>
            </div>

            // Chat area
            <div style="flex:1;display:flex;flex-direction:column;align-items:center;\
                justify-content:center;gap:18px;padding:28px 20px;overflow-y:auto">
                // Animated logo
                <div style="width:64px;height:64px;border-radius:18px;\
                    background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                    display:flex;align-items:center;justify-content:center;\
                    box-shadow:0 8px 32px rgba(124,157,255,0.3)">
                    <svg width="32" height="32" viewBox="0 0 16 16" fill="white">
                        <path d="M7.657 6.247c.11-.33.576-.33.686 0l.645 1.937a2.89 2.89 0 0 0 1.829 1.828l1.936.645c.33.11.33.576 0 .686l-1.937.645a2.89 2.89 0 0 0-1.828 1.829l-.645 1.936a.361.361 0 0 1-.686 0l-.645-1.937a2.89 2.89 0 0 0-1.828-1.828l-1.937-.645a.361.361 0 0 1 0-.686l1.937-.645a2.89 2.89 0 0 0 1.828-1.828l.645-1.937zM3.794 1.148a.217.217 0 0 1 .412 0l.387 1.162c.173.518.579.924 1.097 1.097l1.162.387a.217.217 0 0 1 0 .412l-1.162.387A1.734 1.734 0 0 0 4.593 5.69l-.387 1.162a.217.217 0 0 1-.412 0L3.407 5.69A1.734 1.734 0 0 0 2.31 4.593l-1.162-.387a.217.217 0 0 1 0-.412l1.162-.387A1.734 1.734 0 0 0 3.407 2.31l.387-1.162z"/>
                    </svg>
                </div>
                <div style="text-align:center">
                    <p style=move || format!(
                        "font-size:16px;font-weight:700;color:{};margin:0 0 6px;letter-spacing:-0.01em;",
                        tok().text_primary
                    )>"Logar AI"</p>
                    <p style=move || format!(
                        "font-size:12px;color:{};margin:0;line-height:1.6;max-width:200px;",
                        tok().text_muted
                    )>"Intelligent log analysis, pattern detection, and root cause insights."</p>
                </div>
                // Feature chips
                <div style="display:flex;flex-direction:column;gap:5px;width:100%;max-width:210px">
                    {[
                        ("🔍", "Anomaly detection",   "#60a5fa"),
                        ("📊", "Pattern analysis",    "#34d399"),
                        ("💡", "Root cause hints",    "#fbbf24"),
                        ("📝", "Log summarization",   "#a78bfa"),
                        ("⚡", "Real-time insights",  "#f87171"),
                    ].iter().map(|(icon, label, color)| {
                        let icon = *icon; let label = *label; let color = *color;
                        view! {
                            <div style=move || format!(
                                "display:flex;align-items:center;gap:10px;padding:8px 12px;\
                                 border-radius:8px;background:{};border:1px solid {};\
                                 cursor:default;",
                                tok().bg_elevated, tok().border
                            )>
                                <span style="font-size:14px;flex-shrink:0">{icon}</span>
                                <span style=move || format!(
                                    "font-size:12px;color:{};flex:1;", tok().text_secondary
                                )>{label}</span>
                                <div style=format!(
                                    "width:6px;height:6px;border-radius:50%;background:{};flex-shrink:0;opacity:0.5;",
                                    color
                                )/>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            // Input area
            <div style=move || format!(
                "padding:12px 14px;border-top:1px solid {};flex-shrink:0;",
                tok().border
            )>
                <div style=move || format!(
                    "display:flex;align-items:center;gap:8px;padding:9px 12px;\
                     background:{};border:1px solid {};border-radius:10px;",
                    tok().bg_input, tok().border
                )>
                    <span style=move || format!(
                        "font-size:12px;color:{};flex:1;user-select:none;", tok().text_muted
                    )>"Ask Logar about your logs…"</span>
                    <div style=move || "width:26px;height:26px;border-radius:7px;background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                         display:flex;align-items:center;justify-content:center;flex-shrink:0;cursor:pointer;".to_string()>
                        <svg width="12" height="12" viewBox="0 0 16 16" fill="white">
                            <path d="M15.964.686a.5.5 0 0 0-.65-.65L.767 5.855H.766l-.452.18a.5.5 0 0 0-.082.887l.41.26.001.002 4.995 3.178 3.178 4.995.002.002.26.41a.5.5 0 0 0 .886-.083l6-15Zm-1.833 1.89L6.637 10.07l-.215-.338a.5.5 0 0 0-.154-.154l-.338-.215 7.494-7.494 1.178-.471-.47 1.178Z"/>
                        </svg>
                    </div>
                </div>
                <p style=move || format!(
                    "font-size:10px;color:{};text-align:center;margin:6px 0 0;",
                    tok().text_muted
                )>"Logar AI — coming in a future update"</p>
            </div>
        </div>
    }
}
