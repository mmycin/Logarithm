
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
    open_files: ReadSignal<Vec<crate::app::LogFile>>,
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
    
    // Context chips stored separately from input text
    let (context_chips, set_context_chips) = signal(Vec::<(String, usize, String)>::new()); // (file, line, text)
    
    // @ mentioned files stored separately
    let (mentioned_files, set_mentioned_files) = signal(Vec::<String>::new()); // file names
    
    // @ mention autocomplete state
    let (mention_suggestions, set_mention_suggestions) = signal(Vec::<String>::new());
    let (mention_active, set_mention_active) = signal(false);
    let (mention_selected, set_mention_selected) = signal(0usize);

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
        let chips = context_chips.get();
        let mentions = mentioned_files.get();
        
        // Build full message with context chips for LLM
        let mut full_msg_parts = Vec::new();
        
        // Add context chips
        for (file, line, text) in &chips {
            full_msg_parts.push(format!("[{file}:{line}]\n{text}"));
        }
        
        // Add mentioned files
        for file_name in &mentions {
            // Find the full file data
            if let Some(file) = open_files.get().iter().find(|f| {
                let fname = f.name.rsplit('/').next()
                    .or_else(|| f.name.rsplit('\\').next())
                    .unwrap_or(&f.name);
                fname == file_name
            }) {
                let entries_text: Vec<String> = file.entries.iter()
                    .take(100) // Limit to first 100 lines to avoid token limits
                    .map(|e| format!("[Line {}] {} {} {}", e.line, e.datetime, e.status, e.message))
                    .collect();
                full_msg_parts.push(format!("File: @{}\n{}", file_name, entries_text.join("\n")));
            }
        }
        
        // Add user message
        if !msg.is_empty() {
            full_msg_parts.push(msg.clone());
        }
        
        let full_msg_for_llm = full_msg_parts.join("\n\n");
        
        // Build display message with only tags for UI
        let mut display_parts = Vec::new();
        for (file, line, _text) in &chips {
            display_parts.push(format!("[{file}:{line}]"));
        }
        for file_name in &mentions {
            display_parts.push(format!("@{}", file_name));
        }
        if !msg.is_empty() {
            display_parts.push(msg.clone());
        }
        let display_msg = display_parts.join(" ");
        
        if full_msg_for_llm.is_empty() || loading.get() { return; }
        let key = api_key.get();
        if key.is_empty() { return; }

        set_input.set(String::new());
        set_context_chips.set(Vec::new()); // Clear chips after sending
        set_mentioned_files.set(Vec::new()); // Clear mentioned files after sending
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

        // Push user message to UI (show only tags, not full content)
        set_messages.update(|m| m.push(ChatMessage { role: "user".into(), content: display_msg.clone() }));

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
                "You are Logan, an expert log analysis assistant. Be concise and precise. \
                 Answer directly without unnecessary explanations. Use markdown for code/logs."
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
            let _ = Reflect::set(&um, &JsValue::from_str("content"), &JsValue::from_str(&full_msg_for_llm));
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
                // Add a context chip (stored separately, not in input text)
                set_context_chips.update(|chips| {
                    chips.push((file, line, text));
                });
                // Ensure chat is visible
                if !setup_done.get() { set_setup_done.set(true); }
            }
            LoganAction::AddMultipleContext { items } => {
                // Add multiple context chips at once
                set_context_chips.update(|chips| {
                    for (file, line, text) in items {
                        chips.push((file, line, text));
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
                set_context_chips.set(Vec::new()); // Clear any existing chips
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
                        <img src="/public/LoganIcon.png" width="16" height="16" style="border-radius:3px;opacity:0.7" alt="Logan" />
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
                    {move || messages.get().into_iter().enumerate().map(|(idx, msg)| {
                        let is_user  = msg.role == "user";
                        let is_error = msg.role == "error";
                        let content_for_copy = msg.content.clone();
                        let content_for_html = msg.content.clone();
                        
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
                                    use crate::components::markdown::md_to_html;
                                    let is_dark = theme.get() == Theme::Dark;
                                    if is_user || is_error {
                                        // For user and error messages, use pre-wrap style
                                        format!("<div style='white-space:pre-wrap'>{}</div>", 
                                            content_for_html.replace('&', "&amp;")
                                                   .replace('<', "&lt;")
                                                   .replace('>', "&gt;"))
                                    } else {
                                        // For assistant messages, render markdown
                                        md_to_html(&content_for_html, is_dark)
                                    }
                                }>
                                </div>
                                
                                // Action buttons (copy + retry for assistant)
                                <div style="display:flex;gap:4px;align-items:center;">
                                    // Copy button (for all messages)
                                    <button
                                        style=move || format!(
                                            "padding:3px 7px;border-radius:4px;border:1px solid {};\
                                             background:{};color:{};font-size:10px;cursor:pointer;\
                                             display:flex;align-items:center;gap:4px;opacity:0.7;\
                                             transition:opacity 0.15s;",
                                            tok().border, tok().bg_input, tok().text_muted
                                        )
                                        on:click=move |_| {
                                            let content_to_copy = content_for_copy.clone();
                                            // Use JS eval to copy to clipboard
                                            let js_code = format!(
                                                "navigator.clipboard.writeText(`{}`)",
                                                content_to_copy.replace('`', "\\`").replace('\\', "\\\\")
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
                                    
                                    // Retry button (only for assistant messages)
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
                                                    // Find the user message before this assistant message
                                                    let msgs = messages.get();
                                                    if idx > 0 {
                                                        if let Some(prev_msg) = msgs.get(idx - 1) {
                                                            if prev_msg.role == "user" {
                                                                // Remove messages from this point onwards
                                                                set_messages.update(|m| m.truncate(idx - 1));
                                                                // Set the input to the previous user message and resend
                                                                set_input.set(prev_msg.content.clone());
                                                                send_message();
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
                    // Context chips display (above input)
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
                    
                    // Mentioned files chips display
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
                        // @ mention autocomplete dropdown
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
                                                // Add to mentioned files instead of inserting into text
                                                set_mentioned_files.update(|files| {
                                                    if !files.contains(&fname) {
                                                        files.push(fname.clone());
                                                    }
                                                });
                                                // Remove the @ and partial text from input
                                                let current = input.get();
                                                if let Some(at_pos) = current.rfind('@') {
                                                    let new_val = current[..at_pos].trim_end().to_string();
                                                    set_input.set(new_val);
                                                }
                                                set_mention_active.set(false);
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
                                
                                // Check for @ mention
                                if let Some(at_pos) = val.rfind('@') {
                                    let after_at = &val[at_pos + 1..];
                                    // Only show suggestions if @ is at start or after whitespace
                                    let before_at = if at_pos > 0 { &val[at_pos - 1..at_pos] } else { " " };
                                    if before_at.chars().all(|c| c.is_whitespace()) && !after_at.contains(char::is_whitespace) {
                                        // Show file suggestions
                                        let files = open_files.get();
                                        let query = after_at.to_lowercase();
                                        let suggestions: Vec<String> = files.iter()
                                            .map(|f| {
                                                // Extract just the filename from path
                                                f.name.rsplit('/').next()
                                                    .or_else(|| f.name.rsplit('\\').next())
                                                    .unwrap_or(&f.name)
                                                    .to_string()
                                            })
                                            .filter(|name| query.is_empty() || name.to_lowercase().contains(&query))
                                            .collect();
                                        
                                        if !suggestions.is_empty() {
                                            set_mention_suggestions.set(suggestions);
                                            set_mention_active.set(true);
                                            set_mention_selected.set(0);
                                        } else {
                                            set_mention_active.set(false);
                                        }
                                    } else {
                                        set_mention_active.set(false);
                                    }
                                } else {
                                    set_mention_active.set(false);
                                }
                            }
                            on:keydown=move |ev| {
                                if mention_active.get() {
                                    let suggestions = mention_suggestions.get();
                                    let selected = mention_selected.get();
                                    
                                    match ev.key().as_str() {
                                        "ArrowDown" => {
                                            ev.prevent_default();
                                            if selected + 1 < suggestions.len() {
                                                set_mention_selected.set(selected + 1);
                                            }
                                        }
                                        "ArrowUp" => {
                                            ev.prevent_default();
                                            if selected > 0 {
                                                set_mention_selected.set(selected - 1);
                                            }
                                        }
                                        "Enter" | "Tab" => {
                                            if !suggestions.is_empty() && selected < suggestions.len() {
                                                ev.prevent_default();
                                                let file_name = suggestions[selected].clone();
                                                // Add to mentioned files instead of inserting into text
                                                set_mentioned_files.update(|files| {
                                                    if !files.contains(&file_name) {
                                                        files.push(file_name);
                                                    }
                                                });
                                                // Remove the @ and partial text from input
                                                let current = input.get();
                                                if let Some(at_pos) = current.rfind('@') {
                                                    let new_val = current[..at_pos].trim_end().to_string();
                                                    set_input.set(new_val);
                                                }
                                                set_mention_active.set(false);
                                            }
                                        }
                                        "Escape" => {
                                            set_mention_active.set(false);
                                        }
                                        _ => {}
                                    }
                                }
                                
                                if ev.key() == "Enter" && !ev.shift_key() && !mention_active.get() {
                                    ev.prevent_default();
                                    send_message();
                                }
                            }
                        />
                        <button
                            style=move || {
                                let is_loading = loading.get();
                                if is_loading {
                                    // Stop button style
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
                                    // Stop generation
                                    set_loading.set(false);
                                } else {
                                    send_message();
                                }
                            }
                            title=move || if loading.get() { "Stop generation" } else { "Send (Enter)" }
                        >
                            {move || {
                                if loading.get() {
                                    // Stop icon
                                    view! {
                                        <svg width="10" height="10" viewBox="0 0 16 16" fill="white">
                                            <rect width="16" height="16" rx="2"/>
                                        </svg>
                                    }.into_any()
                                } else {
                                    // Send icon
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
            </Show>
        </div>
    }
}
