use crate::app::FilterState;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LogEntry, LogFile, Theme};
use leptos::ev::KeyboardEvent;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
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

fn save_open_files(files: &[LogFile]) {
    // Save file names and their content to localStorage
    let file_data: Vec<(String, Vec<LogEntry>)> = files.iter()
        .map(|f| (f.name.clone(), f.entries.clone()))
        .collect();
    if let Ok(json) = serde_json::to_string(&file_data) {
        ls_set("logan_open_files_data", &json);
    }
}

#[component]
pub fn FileBar(
    theme: ReadSignal<Theme>,
    open_files: ReadSignal<Vec<LogFile>>,
    set_open_files: WriteSignal<Vec<LogFile>>,
    active_file: ReadSignal<Option<usize>>,
    set_active_file: WriteSignal<Option<usize>>,
    open_dialog_trigger: ReadSignal<u32>,
    filter_state: FilterState,
) -> impl IntoView {
    #[allow(unused_variables)]
    let filter_state = filter_state;

    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let file_input_ref: NodeRef<leptos::html::Input> = NodeRef::new();

    // Restore previously opened files on mount
    let (restored, set_restored) = signal(false);
    Effect::new(move |_| {
        if !restored.get() && open_files.get().is_empty() {
            set_restored.set(true);
            if let Some(json) = ls_get("logan_open_files_data") {
                if let Ok(file_data) = serde_json::from_str::<Vec<(String, Vec<LogEntry>)>>(&json) {
                    set_open_files.set(file_data.into_iter().map(|(name, entries)| {
                        LogFile { name, entries }
                    }).collect());
                    // Set the first file as active
                    if !open_files.get().is_empty() {
                        set_active_file.set(Some(0));
                    }
                }
            }
        }
    });

    // Save open files whenever they change
    Effect::new(move |_| {
        let files = open_files.get();
        save_open_files(&files);
    });

    // Open dialog when trigger increments
    Effect::new(move |_| {
        if open_dialog_trigger.get() > 0 {
            if let Some(input) = file_input_ref.get() {
                input.click();
            }
        }
    });

    // Ctrl+W = close active tab, Ctrl+F = focus search
    window_event_listener(leptos::ev::keydown, move |ev: KeyboardEvent| {
        let ctrl = ev.ctrl_key() || ev.meta_key();
        if ctrl {
            match ev.key().as_str() {
                "w" | "W" => {
                    ev.prevent_default();
                    if let Some(idx) = active_file.get() {
                        set_open_files.update(|files| {
                            if idx < files.len() {
                                files.remove(idx);
                                let n = files.len();
                                set_active_file.set(if n == 0 { None } else if idx >= n { Some(n - 1) } else { Some(idx) });
                            }
                        });
                    }
                }
                "f" | "F" => {
                    ev.prevent_default();
                    if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                        if let Ok(Some(el)) = doc.query_selector("input[placeholder='Search messages…']") {
                            let _ = el.dyn_into::<HtmlInputElement>().map(|i| i.focus());
                        }
                    }
                }
                _ => {}
            }
        }
    });

    let handle_file_change = move |ev: web_sys::Event| {
        let target = ev.target().and_then(|t| t.dyn_into::<HtmlInputElement>().ok());
        let Some(input) = target else { return };
        let Some(file_list) = input.files() else { return };
        let Some(file) = file_list.get(0) else { return };

        let file_name = file.name();
        if !file_name.to_ascii_lowercase().ends_with(".log") {
            input.set_value("");
            return;
        }

        let reader = web_sys::FileReader::new().unwrap();
        let reader_clone = reader.clone();

        let onloadend = Closure::<dyn FnMut(web_sys::Event)>::new(move |_: web_sys::Event| {
            let text = reader_clone.result().ok().and_then(|v| v.as_string()).unwrap_or_default();
            let file_name = file_name.clone();
            spawn_local(async move {
                let args = js_sys::Object::new();
                let _ = Reflect::set(&args, &JsValue::from_str("text"), &JsValue::from_str(&text));
                let result = invoke("parse_log", JsValue::from(args)).await;
                let entries: Vec<LogEntry> = serde_wasm_bindgen::from_value(result).unwrap_or_default();
                set_open_files.update(|files| {
                    if !files.iter().any(|f| f.name == file_name) {
                        files.push(LogFile { name: file_name, entries });
                    }
                    let idx = files.len().saturating_sub(1);
                    set_active_file.set(Some(idx));
                });
            });
        });

        reader.set_onloadend(Some(onloadend.as_ref().unchecked_ref()));
        onloadend.forget();
        let _ = reader.read_as_text(&file);
        input.set_value("");
    };

    view! {
        <div style=move || format!(
            "background:{};border-bottom:1px solid {};display:flex;align-items:flex-end;\
             padding:0 8px;flex-shrink:0;min-height:34px;",
            tok().bg_surface, tok().border
        )>
            // Tabs
            <div style="display:flex;align-items:flex-end;gap:2px;flex:1;overflow-x:auto;scrollbar-width:none">
                {move || {
                    let files = open_files.get();
                    if files.is_empty() {
                        // Welcome tab — always shown when no files open
                        return view! {
                            <div style=move || {
                                let t = tok();
                                format!(
                                    "display:flex;align-items:center;gap:6px;padding:5px 12px;\
                                     border-radius:6px 6px 0 0;font-size:12px;font-weight:500;\
                                     background:{};color:{};border:1px solid {};\
                                     border-bottom:1px solid {};cursor:default;flex-shrink:0;",
                                    t.bg_active, t.text_primary, t.border, t.bg_active
                                )
                            }>
                                // Home icon
                                <svg viewBox="0 0 16 16" fill="currentColor"
                                    style=move || format!("width:12px;height:12px;flex-shrink:0;color:{}", tok().accent)>
                                    <path d="M8.354 1.146a.5.5 0 0 0-.708 0l-6 6A.5.5 0 0 0 1.5 7.5v7a.5.5 0 0 0 .5.5h4.5a.5.5 0 0 0 .5-.5v-4h2v4a.5.5 0 0 0 .5.5H14a.5.5 0 0 0 .5-.5v-7a.5.5 0 0 0-.146-.354L13 5.793V2.5a.5.5 0 0 0-.5-.5h-1a.5.5 0 0 0-.5.5v1.293L8.354 1.146z"/>
                                </svg>
                                "Welcome"
                            </div>
                        }.into_any();
                    }
                    files.into_iter().enumerate().map(|(index, file)| {
                        let is_active = move || active_file.get() == Some(index);
                        view! {
                            <div
                                style=move || {
                                    let t = tok();
                                    if is_active() {
                                        format!(
                                            "display:flex;align-items:center;gap:6px;padding:5px 10px;\
                                             border-radius:6px 6px 0 0;font-size:12px;font-weight:500;\
                                             background:{};color:{};border:1px solid {};\
                                             border-bottom:1px solid {};cursor:default;\
                                             flex-shrink:0;transition:all 0.1s;",
                                            t.bg_active, t.text_primary, t.border, t.bg_active
                                        )
                                    } else {
                                        format!(
                                            "display:flex;align-items:center;gap:6px;padding:5px 10px;\
                                             border-radius:6px 6px 0 0;font-size:12px;font-weight:400;\
                                             background:transparent;color:{};border:1px solid transparent;\
                                             cursor:default;flex-shrink:0;transition:all 0.1s;",
                                            t.text_muted
                                        )
                                    }
                                }
                                on:click=move |_| set_active_file.set(Some(index))
                            >
                                // Log file icon
                                <svg viewBox="0 0 16 16" fill="currentColor"
                                    style=move || format!(
                                        "width:12px;height:12px;flex-shrink:0;color:{}",
                                        if is_active() { tok().accent } else { tok().text_muted }
                                    )
                                >
                                    <path d="M4 0h5.293A1 1 0 0 1 10 .293L13.707 4a1 1 0 0 1 .293.707V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2zm5.5 1.5v2a1 1 0 0 0 1 1h2L9.5 1.5z"/>
                                    <rect x="4.5" y="7" width="7" height="1" rx="0.5" fill="currentColor" opacity="0.4"/>
                                    <rect x="4.5" y="9.2" width="5" height="1" rx="0.5" fill="currentColor" opacity="0.3"/>
                                    <rect x="4.5" y="11.4" width="6" height="1" rx="0.5" fill="currentColor" opacity="0.25"/>
                                </svg>
                                <span style="max-width:130px;overflow:hidden;text-overflow:ellipsis;white-space:nowrap">
                                    {file.name.clone()}
                                </span>
                                // Close button
                                <button
                                    style=move || format!(
                                        "width:14px;height:14px;display:flex;align-items:center;justify-content:center;\
                                         border-radius:3px;border:none;background:transparent;color:{};\
                                         cursor:default;flex-shrink:0;margin-left:1px;transition:all 0.1s;",
                                        tok().text_muted
                                    )
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        let cur = active_file.get();
                                        set_open_files.update(|files| {
                                            if index < files.len() {
                                                files.remove(index);
                                                let n = files.len();
                                                match cur {
                                                    None => {}
                                                    Some(ai) if ai == index => {
                                                        set_active_file.set(if n == 0 { None } else if index >= n { Some(n-1) } else { Some(index) });
                                                    }
                                                    Some(ai) if ai > index => set_active_file.set(Some(ai - 1)),
                                                    _ => {}
                                                }
                                            }
                                        });
                                    }
                                    type="button"
                                >
                                    <svg width="8" height="8" viewBox="0 0 10 10" fill="currentColor">
                                        <path d="M1.707.293A1 1 0 0 0 .293 1.707L3.586 5 .293 8.293a1 1 0 1 0 1.414 1.414L5 6.414l3.293 3.293a1 1 0 0 0 1.414-1.414L6.414 5l3.293-3.293A1 1 0 0 0 8.293.293L5 3.586 1.707.293z"/>
                                    </svg>
                                </button>
                            </div>
                        }
                    }).collect_view().into_any()
                }}
            </div>

            // Open button
            <button
                style=move || format!(
                    "width:22px;height:22px;display:flex;align-items:center;justify-content:center;\
                     border-radius:4px;border:none;background:transparent;color:{};\
                     cursor:default;flex-shrink:0;margin-bottom:5px;margin-left:4px;transition:all 0.1s;",
                    tok().text_muted
                )
                on:click=move |_| { if let Some(i) = file_input_ref.get() { i.click(); } }
                title="Open .log file (Ctrl+O)"
            >
                <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor">
                    <path d="M8 4a.5.5 0 0 1 .5.5v3h3a.5.5 0 0 1 0 1h-3v3a.5.5 0 0 1-1 0v-3h-3a.5.5 0 0 1 0-1h3v-3A.5.5 0 0 1 8 4z"/>
                </svg>
            </button>

            <input node_ref=file_input_ref type="file" accept=".log" class="hidden" on:change=handle_file_change />
        </div>
    }
}
