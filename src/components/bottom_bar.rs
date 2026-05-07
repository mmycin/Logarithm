use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::Theme;
use leptos::prelude::*;

#[component]
pub fn BottomBar(
    theme: ReadSignal<Theme>,
    total_lines: impl Fn() -> usize + Send + 'static,
    ai_open: ReadSignal<bool>,
    set_ai_open: WriteSignal<bool>,
    filter_open: ReadSignal<bool>,
    set_filter_open: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    view! {
        <div style=move || format!(
            "display:flex;align-items:center;justify-content:space-between;\
             height:24px;padding:0 10px;flex-shrink:0;\
             background:{};border-top:1px solid {};",
            tok().bg_surface, tok().border
        )>
            // Left: filter toggle + line count
            <div style="display:flex;align-items:center;gap:8px">
                <button
                    style=move || {
                        let t = tok();
                        let on = filter_open.get();
                        if on {
                            format!("display:flex;align-items:center;gap:4px;height:16px;padding:0 7px;\
                                     border-radius:3px;border:1px solid {};background:{};\
                                     color:{};font-size:10.5px;font-weight:600;cursor:pointer;",
                                t.accent_border, t.accent_bg, t.accent)
                        } else {
                            format!("display:flex;align-items:center;gap:4px;height:16px;padding:0 7px;\
                                     border-radius:3px;border:1px solid transparent;background:transparent;\
                                     color:{};font-size:10.5px;font-weight:500;cursor:pointer;",
                                t.text_muted)
                        }
                    }
                    on:click=move |_| set_filter_open.update(|v| *v = !*v)
                    title="Toggle filter panel (Ctrl+B)"
                >
                    <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5v-2z"/>
                    </svg>
                    "Filters"
                </button>
                <span style=move || format!(
                    "font-size:10.5px;color:{};font-variant-numeric:tabular-nums;",
                    tok().text_muted
                )>
                    {move || {
                        let n = total_lines();
                        if n == 0 { String::new() }
                        else { format!("{} lines", n) }
                    }}
                </span>
            </div>

            // Right: AI chat toggle
            <button
                style=move || {
                    let t = tok();
                    let open = ai_open.get();
                    if open {
                        format!("display:flex;align-items:center;gap:5px;height:16px;padding:0 8px;\
                                 border-radius:3px;border:1px solid {};background:{};\
                                 color:{};font-size:10.5px;font-weight:600;cursor:pointer;",
                            t.accent_border, t.accent_bg, t.accent)
                    } else {
                        format!("display:flex;align-items:center;gap:5px;height:16px;padding:0 8px;\
                                 border-radius:3px;border:1px solid transparent;background:transparent;\
                                 color:{};font-size:10.5px;font-weight:500;cursor:pointer;",
                            t.text_muted)
                    }
                }
                on:click=move |_| set_ai_open.update(|v| *v = !*v)
                title="Toggle AI Assistant (Ctrl+L)"
            >
                "AI Chat"
            </button>
        </div>
    }
}
