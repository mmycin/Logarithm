/// Message sending handler.
/// 
/// Handles building and sending messages to the AI provider,
/// including context chips and file mentions.

use crate::ai::types::{ChatMessage, Provider};
use crate::shared::types::LogFile;
use js_sys::Reflect;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

/// Create the send message handler
pub fn create_send_handler(
    input: ReadSignal<String>,
    set_input: WriteSignal<String>,
    context_chips: ReadSignal<Vec<(String, usize, String)>>,
    set_context_chips: WriteSignal<Vec<(String, usize, String)>>,
    mentioned_files: ReadSignal<Vec<String>>,
    set_mentioned_files: WriteSignal<Vec<String>>,
    loading: ReadSignal<bool>,
    set_loading: WriteSignal<bool>,
    api_key: ReadSignal<String>,
    provider: ReadSignal<Provider>,
    model: ReadSignal<String>,
    messages: ReadSignal<Vec<ChatMessage>>,
    set_messages: WriteSignal<Vec<ChatMessage>>,
    open_files: ReadSignal<Vec<LogFile>>,
) -> Callback<()> {
    Callback::new(move |_| {
        let msg = input.get().trim().to_string();
        let chips = context_chips.get();
        let mentions = mentioned_files.get();
        
        let full_msg_for_llm = build_full_message(&msg, &chips, &mentions, &open_files.get());
        let display_msg = build_display_message(&msg, &chips, &mentions);
        
        if full_msg_for_llm.is_empty() || loading.get() { return; }
        let key = api_key.get();
        if key.is_empty() { return; }

        set_input.set(String::new());
        set_context_chips.set(Vec::new());
        set_mentioned_files.set(Vec::new());
        set_loading.set(true);

        let prov_str = provider.get().to_str();
        let mdl = get_model_name(&model.get(), &provider.get());
        let history = get_message_history(&messages.get());

        set_messages.update(|m| m.push(ChatMessage::user(display_msg)));

        spawn_local(async move {
            let result = send_ai_request(prov_str, &key, &mdl, &history, &full_msg_for_llm).await;
            set_loading.set(false);
            handle_ai_response(result, set_messages);
        });
    })
}

fn build_full_message(
    msg: &str,
    chips: &[(String, usize, String)],
    mentions: &[String],
    open_files: &[LogFile],
) -> String {
    let mut parts = Vec::new();
    
    for (file, line, text) in chips {
        parts.push(format!("[{file}:{line}]\n{text}"));
    }
    
    for file_name in mentions {
        if let Some(file) = open_files.iter().find(|f| {
            let fname = f.name.rsplit('/').next()
                .or_else(|| f.name.rsplit('\\').next())
                .unwrap_or(&f.name);
            fname == file_name
        }) {
            let entries_text: Vec<String> = file.entries.iter()
                .take(100)
                .map(|e| format!("[Line {}] {} {} {}", e.line, e.datetime, e.status, e.message))
                .collect();
            parts.push(format!("File: @{}\n{}", file_name, entries_text.join("\n")));
        }
    }
    
    if !msg.is_empty() {
        parts.push(msg.to_string());
    }
    
    parts.join("\n\n")
}

fn build_display_message(
    msg: &str,
    chips: &[(String, usize, String)],
    mentions: &[String],
) -> String {
    let mut parts = Vec::new();
    
    for (file, line, _) in chips {
        parts.push(format!("[{file}:{line}]"));
    }
    
    for file_name in mentions {
        parts.push(format!("@{}", file_name));
    }
    
    if !msg.is_empty() {
        parts.push(msg.to_string());
    }
    
    parts.join(" ")
}

fn get_model_name(model: &str, provider: &Provider) -> String {
    if model.is_empty() {
        provider.default_model().to_string()
    } else {
        model.to_string()
    }
}

fn get_message_history(messages: &[ChatMessage]) -> Vec<(String, String)> {
    messages.iter()
        .filter(|m| m.role != "error")
        .map(|m| (m.role.clone(), m.content.clone()))
        .collect()
}

async fn send_ai_request(
    provider: &str,
    api_key: &str,
    model: &str,
    history: &[(String, String)],
    user_message: &str,
) -> JsValue {
    let req = js_sys::Object::new();
    let _ = Reflect::set(&req, &JsValue::from_str("provider"), &JsValue::from_str(provider));
    let _ = Reflect::set(&req, &JsValue::from_str("api_key"), &JsValue::from_str(api_key));
    let _ = Reflect::set(&req, &JsValue::from_str("model"), &JsValue::from_str(model));

    let msgs_arr = js_sys::Array::new();

    let sys = js_sys::Object::new();
    let _ = Reflect::set(&sys, &JsValue::from_str("role"), &JsValue::from_str("system"));
    let _ = Reflect::set(&sys, &JsValue::from_str("content"), &JsValue::from_str(
        "You are Logan, an expert log analysis assistant. Be concise and precise. \
         Answer directly without unnecessary explanations. Use markdown for code/logs."
    ));
    msgs_arr.push(&sys);

    for (role, content) in history {
        let m = js_sys::Object::new();
        let _ = Reflect::set(&m, &JsValue::from_str("role"), &JsValue::from_str(role));
        let _ = Reflect::set(&m, &JsValue::from_str("content"), &JsValue::from_str(content));
        msgs_arr.push(&m);
    }

    let um = js_sys::Object::new();
    let _ = Reflect::set(&um, &JsValue::from_str("role"), &JsValue::from_str("user"));
    let _ = Reflect::set(&um, &JsValue::from_str("content"), &JsValue::from_str(user_message));
    msgs_arr.push(&um);

    let _ = Reflect::set(&req, &JsValue::from_str("messages"), &msgs_arr);

    let args = js_sys::Object::new();
    let _ = Reflect::set(&args, &JsValue::from_str("req"), &req);

    invoke("ai_chat", JsValue::from(args)).await
}

fn handle_ai_response(result: JsValue, set_messages: WriteSignal<Vec<ChatMessage>>) {
    if let Some(s) = result.as_string() {
        set_messages.update(|m| m.push(ChatMessage::assistant(s)));
    } else if let Ok(ok_val) = Reflect::get(&result, &JsValue::from_str("Ok")) {
        if let Some(s) = ok_val.as_string() {
            set_messages.update(|m| m.push(ChatMessage::assistant(s)));
        } else {
            set_messages.update(|m| m.push(ChatMessage::error("Empty response from API.".into())));
        }
    } else if let Ok(err_val) = Reflect::get(&result, &JsValue::from_str("Err")) {
        let e = err_val.as_string().unwrap_or_else(|| format!("{:?}", result));
        set_messages.update(|m| m.push(ChatMessage::error(format!("Error: {e}"))));
    } else {
        let raw = js_sys::JSON::stringify(&result)
            .ok().and_then(|s| s.as_string())
            .unwrap_or_else(|| "Unknown response".into());
        set_messages.update(|m| m.push(ChatMessage::error(format!("Unexpected response: {raw}"))));
    }
}
