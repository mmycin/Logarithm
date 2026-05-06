
use crate::app::{LoganAction, Theme, DARK, LIGHT};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use js_sys::Reflect;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// ── localStorage helpers ──────────────────────────────────────────────────────

fn ls_get(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage().ok()??
        .get_item(key).ok()?
}

fn ls_set(key: &str, val: &str) {
    if let Some(Ok(Some(ls))) = web_sys::window().map(|w| w.local_storage()) {
        let _ = ls.set_item(key, val);
    }
}

// ── Types ─────────────────────────────────────────────────────────────────────

#[derive(Clone, PartialEq)]
struct ChatMessage {
    role: String,   // "user" | "assistant" | "error"
    content: String,
}

#[derive(Clone, PartialEq, Debug)]
enum Provider { Gemini, OpenAI }

impl Provider {
    fn label(&self)       -> &'static str { match self { Self::Gemini => "Gemini",    Self::OpenAI => "OpenAI"       } }
    fn to_str(&self)      -> &'static str { match self { Self::Gemini => "gemini",    Self::OpenAI => "openai"       } }
    fn default_model(&self) -> &'static str { match self { Self::Gemini => "gemini-1.5-flash", Self::OpenAI => "gpt-4o-mini" } }
    fn key_label(&self)   -> &'static str { match self { Self::Gemini => "Google AI API Key", Self::OpenAI => "OpenAI API Key" } }
    fn key_hint(&self)    -> &'static str { match self { Self::Gemini => "AIza…",     Self::OpenAI => "sk-…"         } }
    fn docs_url(&self)    -> &'static str { match self {
        Self::Gemini => "https://aistudio.google.com/app/apikey",
        Self::OpenAI => "https://platform.openai.com/api-keys",
    }}
    fn ls_key(&self) -> &'static str { match self { Self::Gemini => "logan_gemini_key", Self::OpenAI => "logan_openai_key" } }
    fn from_str(s: &str) -> Self { if s == "openai" { Self::OpenAI } else { Self::Gemini } }
}

// ── Component ─────────────────────────────────────────────────────────────────

#[component]
pub fn AiPanel(
    theme: ReadSignal<Theme>,
    ai_width: ReadSignal<u32>,
    set_ai_width: WriteSignal<u32>,
    set_ai_open: WriteSignal<bool>,
    logan_action: ReadSignal<Option<LoganAction>>,
    set_logan_action: WriteSignal<Option<LoganAction>>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    // Load saved provider + keys + model from localStorage
    let saved_prov = ls_get("logan_provider").unwrap_or_default();
    let init_prov  = Provider::from_str(&saved_prov);
    let init_key   = ls_get(init_prov.ls_key()).unwrap_or_default();
    let init_model = ls_get("logan_model").unwrap_or_default();
    let init_ready = !init_key.is_empty();

    let (provider, set_provider)     = signal(init_prov);
    let (api_key, set_api_key)       = signal(init_key);
    let (model, set_model)           = signal(init_model);
    let (input, set_input)           = signal(String::new());
    let (messages, set_messages)     = signal(Vec::<ChatMessage>::new());
    let (loading, set_loading)       = signal(false);
    let (setup_done, set_setup_done) = signal(init_ready);
    let (dragging, set_dragging)     = signal(false);
    let (drag_start_x, set_dsx)      = signal(0i32);
    let (drag_start_w, set_dsw)      = signal(0u32);

    // Greet on first open if already configured
    if init_ready {
        set_messages.set(vec![ChatMessage {
            role: "assistant".into(),
            content: "Hi! I'm Logan, your AI log analysis assistant. Open a log file and ask me anything.".into(),
        }]);
    }

    // ── send_message ─────────────────────────────────────────────────────
    let send_message = move || {
        let msg = input.get().trim().to_string();
        if msg.is_empty() || loading.get() { return; }
        let key = api_key.get();
        if key.is_empty() { return; }

        set_input.set(String::new());
        set_loading.set(true);

        let prov_str = provider.get().to_str();
        let mdl = {
            let m = model.get();
            if m.is_empty() { provider.get().default_model().to_string() } else { m }
        };

        // Snapshot history BEFORE pushing the new user message
        // so we don't double-send it (we add it manually to msgs_arr below)
        let history: Vec<(String, String)> = messages.get()
            .iter()
            .filter(|m| m.role != "error")
            .map(|m| (m.role.clone(), m.content.clone()))
            .collect();

        // Push user message to UI
        set_messages.update(|m| m.push(ChatMessage { role: "user".into(), content: msg.clone() }));

        spawn_local(async move {
            // Build the request object
            let req = js_sys::Object::new();
            let _ = Reflect::set(&req, &JsValue::from_str("provider"),  &JsValue::from_str(prov_str));
            let _ = Reflect::set(&req, &JsValue::from_str("api_key"),   &JsValue::from_str(&key));
            let _ = Reflect::set(&req, &JsValue::from_str("model"),     &JsValue::from_str(&mdl));

            let msgs_arr = js_sys::Array::new();

            // System prompt
            let sys = js_sys::Object::new();
            let _ = Reflect::set(&sys, &JsValue::from_str("role"),    &JsValue::from_str("system"));
            let _ = Reflect::set(&sys, &JsValue::from_str("content"), &JsValue::from_str(
                "You are Logan, an expert AI assistant specialised in analysing log files. \
                 Help the user understand their logs, find errors, detect patterns, and diagnose issues. \
                 Be concise and technical. Format code/log snippets in markdown code blocks."
            ));
            msgs_arr.push(&sys);

            // Conversation history
            for (role, content) in &history {
                let m = js_sys::Object::new();
                let _ = Reflect::set(&m, &JsValue::from_str("role"),    &JsValue::from_str(role));
                let _ = Reflect::set(&m, &JsValue::from_str("content"), &JsValue::from_str(content));
                msgs_arr.push(&m);
            }

            // New user message
            let um = js_sys::Object::new();
            let _ = Reflect::set(&um, &JsValue::from_str("role"),    &JsValue::from_str("user"));
            let _ = Reflect::set(&um, &JsValue::from_str("content"), &JsValue::from_str(&msg));
            msgs_arr.push(&um);

            let _ = Reflect::set(&req, &JsValue::from_str("messages"), &msgs_arr);

            // Wrap in { req: ... } for Tauri
            let args = js_sys::Object::new();
            let _ = Reflect::set(&args, &JsValue::from_str("req"), &req);

            let result = invoke("ai_chat", JsValue::from(args)).await;

            set_loading.set(false);

            // Tauri 2 serialises Result<String,String> as:
            //   Ok  → the string value directly (JsValue::from_str)
            //   Err → an object { "Err": "message" }
            // Try plain string first, then {"Ok":...}, then {"Err":...}
            if let Some(s) = result.as_string() {
                set_messages.update(|m| m.push(ChatMessage { role: "assistant".into(), content: s }));
            } else if let Ok(ok_val) = Reflect::get(&result, &JsValue::from_str("Ok")) {
                if let Some(s) = ok_val.as_string() {
                    set_messages.update(|m| m.push(ChatMessage { role: "assistant".into(), content: s }));
                } else {
                    set_messages.update(|m| m.push(ChatMessage { role: "error".into(), content: "Empty response from API.".into() }));
                }
            } else if let Ok(err_val) = Reflect::get(&result, &JsValue::from_str("Err")) {
                let e = err_val.as_string().unwrap_or_else(|| format!("{:?}", result));
                set_messages.update(|m| m.push(ChatMessage { role: "error".into(), content: format!("Error: {e}") }));
            } else {
                // Last resort: stringify the whole result for debugging
                let raw = js_sys::JSON::stringify(&result)
                    .ok().and_then(|s| s.as_string())
                    .unwrap_or_else(|| "Unknown response".into());
                set_messages.update(|m| m.push(ChatMessage { role: "error".into(), content: format!("Unexpected response: {raw}") }));
            }
        });
    };

    // ── React to LoganAction from FileViewer ─────────────────────────────
    Effect::new(move |_| {
        let Some(action) = logan_action.get() else { return };
        set_logan_action.set(None); // consume immediately

        match action {
            LoganAction::AddContext { file, line, text } => {
                // Add a context chip to the input box (don't send)
                let chip = format!("[{file}:{line}]\n{text}");
                set_input.update(|inp| {
                    if inp.is_empty() {
                        *inp = chip;
                    } else {
                        inp.push('\n');
                        inp.push_str(&chip);
                    }
                });
                // Ensure chat is visible
                if !setup_done.get() { set_setup_done.set(true); }
            }
            LoganAction::Explain { file, line, text } => {
                // Build the explain message and auto-send
                let msg = format!(
                    "Explain this log line from {file}:{line}\n\n```\n{text}\n```"
                );
                set_input.set(msg);
                // Ensure chat is visible then send
                if !setup_done.get() { set_setup_done.set(true); }
                // Trigger send on next tick via a small trick: set input then call send
                // We call send_message directly since we just set input
                send_message();
            }
        }
    });

    // ── View ─────────────────────────────────────────────────────────────
    view! {
        <div
            style=move || format!(
                "width:{}px;flex-shrink:0;display:flex;flex-direction:column;\
                 background:{};position:relative;min-width:280px;max-width:640px;",
                ai_width.get(), tok().bg_surface
            )
            on:mousemove=move |ev| {
                if dragging.get() {
                    let delta = drag_start_x.get() - ev.client_x();
                    let new_w = (drag_start_w.get() as i32 + delta).max(280).min(640) as u32;
                    set_ai_width.set(new_w);
                }
            }
            on:mouseup=move |_|    set_dragging.set(false)
            on:mouseleave=move |_| set_dragging.set(false)
        >
            // Resize handle (left edge)
            <div
                style="position:absolute;left:0;top:0;bottom:0;width:4px;cursor:col-resize;z-index:10;"
                on:mousedown=move |ev| {
                    ev.prevent_default();
                    set_dragging.set(true);
                    set_dsx.set(ev.client_x());
                    set_dsw.set(ai_width.get());
                }
            />
            <div style=move || format!("position:absolute;left:0;top:0;bottom:0;width:1px;background:{};", tok().border)/>

            // ── Header ────────────────────────────────────────────────────
            <div style=move || format!(
                "display:flex;align-items:center;justify-content:space-between;\
                 padding:0 14px;height:42px;border-bottom:1px solid {};flex-shrink:0;",
                tok().border
            )>
                <div style="display:flex;align-items:center;gap:8px">
                    <div style="width:24px;height:24px;border-radius:7px;\
                        background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                        display:flex;align-items:center;justify-content:center;flex-shrink:0">
                        <svg width="13" height="13" viewBox="0 0 16 16" fill="white">
                            <path d="M7.657 6.247c.11-.33.576-.33.686 0l.645 1.937a2.89 2.89 0 0 0 1.829 1.828l1.936.645c.33.11.33.576 0 .686l-1.937.645a2.89 2.89 0 0 0-1.828 1.829l-.645 1.936a.361.361 0 0 1-.686 0l-.645-1.937a2.89 2.89 0 0 0-1.828-1.828l-1.937-.645a.361.361 0 0 1 0-.686l1.937-.645a2.89 2.89 0 0 0 1.828-1.828l.645-1.937z"/>
                        </svg>
                    </div>
                    <span style=move || format!("font-size:13px;font-weight:700;color:{}", tok().text_primary)>"Logan"</span>
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
                            if !setup_done.get() { tok().accent }    else { tok().text_muted }
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

            // ── Setup panel ───────────────────────────────────────────────
            <Show when=move || !setup_done.get()>
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

                    // Provider tabs
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
                                        let saved = ls_get(p.ls_key()).unwrap_or_default();
                                        set_provider.set(p.clone());
                                        set_api_key.set(saved);
                                        ls_set("logan_provider", p.to_str());
                                    }
                                >{label}</button>
                            }
                        }).collect_view()}
                    </div>

                    // API key
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
                                ls_set(provider.get().ls_key(), &v);
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

                    // Model override
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
                                ls_set("logan_model", &v);
                                set_model.set(v);
                            }
                        />
                    </div>

                    // Start button
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
                                ls_set("logan_provider", provider.get().to_str());
                                set_setup_done.set(true);
                                if messages.get().is_empty() {
                                    set_messages.set(vec![ChatMessage {
                                        role: "assistant".into(),
                                        content: format!(
                                            "Hi! I'm Logan, your AI log analysis assistant powered by {}. \
                                             Open a log file and ask me anything — I can help you find errors, \
                                             understand patterns, and diagnose issues.",
                                            provider.get().label()
                                        ),
                                    }]);
                                }
                            }
                        }
                    >"Start chatting →"</button>
                </div>
            </Show>

            // ── Chat messages ─────────────────────────────────────────────
            <Show when=move || setup_done.get()>
                <div style="flex:1;overflow-y:auto;padding:12px;display:flex;flex-direction:column;gap:10px;">
                    {move || messages.get().into_iter().map(|msg| {
                        let is_user  = msg.role == "user";
                        let is_error = msg.role == "error";
                        view! {
                            <div style=if is_user { "display:flex;justify-content:flex-end;" }
                                       else       { "display:flex;justify-content:flex-start;" }>
                                <div style=move || {
                                    let t = tok();
                                    if is_error {
                                        "max-width:92%;padding:9px 12px;border-radius:10px;\
                                         font-size:12px;line-height:1.55;white-space:pre-wrap;\
                                         background:#f8717118;color:#fca5a5;border:1px solid #f8717130;".to_string()
                                    } else if is_user {
                                        format!("max-width:80%;padding:9px 12px;border-radius:10px 10px 3px 10px;\
                                                 font-size:12px;line-height:1.55;white-space:pre-wrap;\
                                                 background:linear-gradient(135deg,{},#a78bfa);color:white;",
                                            t.accent)
                                    } else {
                                        format!("max-width:92%;padding:9px 12px;border-radius:10px 10px 10px 3px;\
                                                 font-size:12px;line-height:1.55;white-space:pre-wrap;\
                                                 background:{};border:1px solid {};color:{};",
                                            t.bg_elevated, t.border, t.text_primary)
                                    }
                                }>
                                    {msg.content.clone()}
                                </div>
                            </div>
                        }
                    }).collect_view()}

                    // Thinking indicator
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
            </Show>

            // ── Input bar ─────────────────────────────────────────────────
            <Show when=move || setup_done.get()>
                <div style=move || format!(
                    "padding:10px 12px;border-top:1px solid {};flex-shrink:0;position:relative;", tok().border
                )>
                    <div style=move || format!(
                        "display:flex;align-items:flex-end;gap:8px;padding:8px 10px;\
                         background:{};border:1px solid {};border-radius:10px;",
                        tok().bg_input, tok().border
                    )>
                        // Context chip — shown when input starts with [file:line]
                        {move || {
                            let inp = input.get();
                            if inp.starts_with('[') {
                                if let Some(end) = inp.find("]\n") {
                                    let chip = &inp[1..end];
                                    return view! {
                                        <div style=move || format!(
                                            "position:absolute;bottom:calc(100% + 4px);left:12px;\
                                             display:flex;align-items:center;gap:5px;padding:3px 8px;\
                                             background:{};border:1px solid {};border-radius:5px;\
                                             font-size:11px;color:{};font-family:'Fira Code',monospace;",
                                            tok().bg_elevated, tok().accent_border, tok().accent
                                        )>
                                            <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor">
                                                <path d="M7.657 6.247c.11-.33.576-.33.686 0l.645 1.937a2.89 2.89 0 0 0 1.829 1.828l1.936.645c.33.11.33.576 0 .686l-1.937.645a2.89 2.89 0 0 0-1.828 1.829l-.645 1.936a.361.361 0 0 1-.686 0l-.645-1.937a2.89 2.89 0 0 0-1.828-1.828l-1.937-.645a.361.361 0 0 1 0-.686l1.937-.645a2.89 2.89 0 0 0 1.828-1.828l.645-1.937z"/>
                                            </svg>
                                            {chip.to_string()}
                                        </div>
                                    }.into_any();
                                }
                            }
                            view! { <span/> }.into_any()
                        }}
                        <textarea
                            style=move || format!(
                                "flex:1;border:none;background:transparent;font-size:12.5px;\
                                 color:{};outline:none;font-family:'Inter',sans-serif;\
                                 resize:none;min-height:20px;max-height:120px;line-height:1.5;cursor:text;",
                                tok().text_primary
                            )
                            placeholder="Ask Logan about your logs…"
                            rows="1"
                            prop:value=move || input.get()
                            on:input=move |ev| set_input.set(event_target_value(&ev))
                            on:keydown=move |ev| {
                                if ev.key() == "Enter" && !ev.shift_key() {
                                    ev.prevent_default();
                                    send_message();
                                }
                            }
                        />
                        <button
                            style=move || {
                                let ready = !input.get().trim().is_empty() && !loading.get();
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
                            on:click=move |_| send_message()
                            title="Send (Enter)"
                        >
                            <svg width="12" height="12" viewBox="0 0 16 16" fill="white">
                                <path d="M15.964.686a.5.5 0 0 0-.65-.65L.767 5.855H.766l-.452.18a.5.5 0 0 0-.082.887l.41.26.001.002 4.995 3.178 3.178 4.995.002.002.26.41a.5.5 0 0 0 .886-.083l6-15Zm-1.833 1.89L6.637 10.07l-.215-.338a.5.5 0 0 0-.154-.154l-.338-.215 7.494-7.494 1.178-.471-.47 1.178Z"/>
                            </svg>
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
            </Show>
        </div>
    }
}
