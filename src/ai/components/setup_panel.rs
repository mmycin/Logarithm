/// Provider setup panel component.
/// 
/// Allows users to configure AI provider and API key.

use crate::ai::types::Provider;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::storage::set_item;
use crate::shared::types::Theme;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use js_sys::Reflect;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn SetupPanel(
    theme: ReadSignal<Theme>,
    provider: ReadSignal<Provider>,
    set_provider: WriteSignal<Provider>,
    api_key: ReadSignal<String>,
    set_api_key: WriteSignal<String>,
    model: ReadSignal<String>,
    set_model: WriteSignal<String>,
    set_setup_done: WriteSignal<bool>,
    set_messages: WriteSignal<Vec<crate::ai::types::ChatMessage>>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    view! {
        <div style=move || format!(
            "padding:16px;border-bottom:1px solid {};flex-shrink:0;\
             display:flex;flex-direction:column;gap:10px;overflow-y:auto;",
            tok().border
        )>
            <p style=move || format!(
                "font-size:12px;color:{};margin:0;line-height:1.5;", tok().text_secondary
            )>
                "Choose a provider and enter your API key. Keys are saved locally for future sessions."
            </p>

            <div style="display:flex;gap:6px">
                {[Provider::Gemini, Provider::OpenAI].into_iter().map(|p| {
                    let label = p.label();
                    let p2 = p.clone();
                    view! {
                        <button
                            style=move || {
                                let t = tok();
                                if provider.get() == p2 {
                                    format!("flex:1;padding:7px;border-radius:7px;font-size:12px;\
                                             font-weight:600;border:1px solid {};background:{};\
                                             color:{};cursor:pointer;transition:all 0.1s;",
                                        t.accent_border, t.accent_bg, t.accent)
                                } else {
                                    format!("flex:1;padding:7px;border-radius:7px;font-size:12px;\
                                             font-weight:500;border:1px solid {};background:transparent;\
                                             color:{};cursor:pointer;transition:all 0.1s;",
                                        t.border, t.text_secondary)
                                }
                            }
                            on:click=move |_| {
                                use crate::shared::storage::get_item;
                                let saved = get_item(p.ls_key()).unwrap_or_default();
                                set_provider.set(p.clone());
                                set_api_key.set(saved);
                                set_item("logan_provider", p.to_str());
                            }
                        >{label}</button>
                    }
                }).collect_view()}
            </div>

            <div>
                <div style=move || format!(
                    "font-size:10px;font-weight:700;color:{};letter-spacing:0.07em;\
                     text-transform:uppercase;margin-bottom:4px;", tok().text_muted
                )>{move || provider.get().key_label()}</div>
                <input
                    type="password"
                    style=move || format!(
                        "width:100%;height:30px;padding:0 10px;background:{};border:1px solid {};\
                         border-radius:6px;font-size:12px;color:{};outline:none;\
                         font-family:'Inter',sans-serif;box-sizing:border-box;cursor:text;",
                        tok().bg_input, tok().border, tok().text_primary
                    )
                    placeholder=move || provider.get().key_hint()
                    prop:value=move || api_key.get()
                    on:input=move |ev| {
                        let v = event_target_value(&ev);
                        set_item(provider.get().ls_key(), &v);
                        set_api_key.set(v);
                    }
                />
                <div style=move || format!(
                    "font-size:10.5px;color:{};margin-top:4px;display:flex;align-items:center;gap:6px;",
                    tok().text_muted
                )>
                    "Saved to localStorage. "
                    <span
                        style=move || format!("color:{};cursor:pointer;text-decoration:underline;", tok().accent)
                        on:click=move |_| {
                            let url = provider.get().docs_url().to_string();
                            spawn_local(async move {
                                let args = js_sys::Object::new();
                                let _ = Reflect::set(&args, &JsValue::from_str("url"), &JsValue::from_str(&url));
                                let _ = invoke("open_url", JsValue::from(args)).await;
                            });
                        }
                    >"Get API key ↗"</span>
                </div>
            </div>

            <div>
                <div style=move || format!(
                    "font-size:10px;font-weight:700;color:{};letter-spacing:0.07em;\
                     text-transform:uppercase;margin-bottom:4px;", tok().text_muted
                )>"Model (optional)"</div>
                <input
                    type="text"
                    style=move || format!(
                        "width:100%;height:30px;padding:0 10px;background:{};border:1px solid {};\
                         border-radius:6px;font-size:12px;color:{};outline:none;\
                         font-family:'Inter',sans-serif;box-sizing:border-box;cursor:text;",
                        tok().bg_input, tok().border, tok().text_primary
                    )
                    placeholder=move || provider.get().default_model()
                    prop:value=move || model.get()
                    on:input=move |ev| {
                        let v = event_target_value(&ev);
                        set_item("logan_model", &v);
                        set_model.set(v);
                    }
                />
            </div>

            <button
                style=move || {
                    let t = tok();
                    if !api_key.get().is_empty() {
                        "width:100%;padding:9px;border-radius:7px;border:none;\
                         background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                         color:white;font-size:13px;font-weight:600;cursor:pointer;".to_string()
                    } else {
                        format!("width:100%;padding:9px;border-radius:7px;border:1px solid {};\
                                 background:transparent;color:{};font-size:13px;font-weight:500;\
                                 cursor:not-allowed;opacity:0.45;", t.border, t.text_muted)
                    }
                }
                on:click=move |_| {
                    if !api_key.get().is_empty() {
                        set_item("logan_provider", provider.get().to_str());
                        set_setup_done.set(true);
                        set_messages.update(|m| {
                            if m.is_empty() {
                                m.push(crate::ai::types::ChatMessage::assistant(format!(
                                    "Hi! I'm Logan, your AI log analysis assistant powered by {}. \
                                     Open a log file and ask me anything — I can help you find errors, \
                                     understand patterns, and diagnose issues.",
                                    provider.get().label()
                                )));
                            }
                        });
                    }
                }
            >"Start chatting →"</button>
        </div>
    }
}
