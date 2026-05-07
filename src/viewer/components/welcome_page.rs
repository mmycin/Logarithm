/// Welcome page component.
/// 
/// Displays when no log file is open, showing quick start guide and shortcuts.

use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::Theme;
use js_sys::Reflect;
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn WelcomePage(theme: ReadSignal<Theme>) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
        <div style=move || format!(
            "flex:1;overflow-y:auto;padding:40px 48px;background:{};",
            tok().bg_base
        )>
            // Header with logo
            <div style="display:flex;align-items:center;gap:16px;margin-bottom:36px">
                <div style="width:52px;height:52px;border-radius:14px;\
                    background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                    display:flex;align-items:center;justify-content:center;\
                    box-shadow:0 8px 24px rgba(124,157,255,0.25);flex-shrink:0">
                    <img src="/public/StoreLogo.png" width="32" height="32" 
                         style="border-radius:6px;opacity:0.9" alt="Logarithm" />
                </div>
                <div>
                    <h1 style=move || format!(
                        "font-size:22px;font-weight:700;color:{};margin:0 0 3px;",
                        tok().text_primary
                    )>"Welcome to Logarithm"</h1>
                    <p style=move || format!(
                        "font-size:13px;color:{};margin:0;", tok().text_muted
                    )>"A modern, fast log file viewer"</p>
                </div>
            </div>
            
            // Quick Start section
            <QuickStartSection theme=theme />
            
            // Keyboard Shortcuts section
            <KeyboardShortcutsSection theme=theme />
            
            // Footer
            <Footer theme=theme />
        </div>
    }
}

#[component]
fn QuickStartSection(theme: ReadSignal<Theme>) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
        <div style=move || format!(
            "background:{};border:1px solid {};border-radius:10px;\
             padding:20px 24px;margin-bottom:20px;",
            tok().bg_surface, tok().border
        )>
            <h2 style=move || format!(
                "font-size:11px;font-weight:700;color:{};margin:0 0 14px;\
                 letter-spacing:0.06em;text-transform:uppercase;",
                tok().text_muted
            )>"Quick Start"</h2>
            <div style="display:flex;flex-direction:column;gap:10px">
                {[
                    ("📂", "Open a log file", "Ctrl+O or File → Open — only .log files"),
                    ("🔍", "Filter by level", "Use the left panel to pick a level"),
                    ("🔎", "Search messages", "Fuzzy, regex, case-sensitive, invert — all supported"),
                    ("📅", "Filter by date", "From / To date range in the left panel"),
                    ("✨", "Logan AI", "Click AI Chat in the bottom bar"),
                ].iter().map(|(icon, title, desc)| {
                    let icon = *icon; let title = *title; let desc = *desc;
                    view! {
                        <div style="display:flex;align-items:flex-start;gap:12px">
                            <span style="font-size:16px;flex-shrink:0;margin-top:1px">{icon}</span>
                            <div>
                                <span style=move || format!(
                                    "font-size:13px;font-weight:600;color:{};",
                                    tok().text_primary
                                )>{title}</span>
                                <span style=move || format!(
                                    "font-size:12px;color:{};margin-left:8px;",
                                    tok().text_secondary
                                )>{desc}</span>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn KeyboardShortcutsSection(theme: ReadSignal<Theme>) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
        <div style=move || format!(
            "background:{};border:1px solid {};border-radius:10px;\
             padding:20px 24px;margin-bottom:20px;",
            tok().bg_surface, tok().border
        )>
            <h2 style=move || format!(
                "font-size:11px;font-weight:700;color:{};margin:0 0 14px;\
                 letter-spacing:0.06em;text-transform:uppercase;",
                tok().text_muted
            )>"Keyboard Shortcuts"</h2>
            <div style="display:grid;grid-template-columns:1fr 1fr;gap:8px">
                {[
                    ("Ctrl+O", "Open file"), ("Ctrl+W", "Close tab"),
                    ("Ctrl+F", "Focus search"), ("Ctrl+T", "Toggle theme"),
                    ("Ctrl+B", "Toggle filters"), ("Ctrl+/", "Show shortcuts"),
                ].iter().map(|(key, desc)| {
                    let key = *key; let desc = *desc;
                    view! {
                        <div style="display:flex;align-items:center;justify-content:space-between;gap:8px">
                            <span style=move || format!("font-size:12px;color:{}", tok().text_secondary)>
                                {desc}
                            </span>
                            <span style=move || format!(
                                "font-size:11px;color:{};background:{};\
                                 border:1px solid {};border-radius:4px;\
                                 padding:1px 7px;font-family:'Fira Code',monospace;\
                                 white-space:nowrap;flex-shrink:0;",
                                tok().text_secondary, tok().bg_elevated, tok().border
                            )>{key}</span>
                        </div>
                    }
                }).collect_view()}
            </div>
        </div>
    }
}

#[component]
fn Footer(theme: ReadSignal<Theme>) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
        <p style=move || format!(
            "font-size:11px;color:{};text-align:center;margin:0;", tok().text_muted
        )>
            "Built with Tauri + Leptos + Rust · "
            <span
                style=move || format!("color:{};text-decoration:underline;cursor:pointer;", tok().accent)
                on:click=move |_| {
                    spawn_local(async {
                        let args = js_sys::Object::new();
                        let _ = Reflect::set(
                            &args,
                            &JsValue::from_str("url"),
                            &JsValue::from_str("https://github.com/mmycin/Logarithm")
                        );
                        let _ = invoke("open_url", JsValue::from(args)).await;
                    });
                }
            >
                "github.com/mmycin/Logarithm"
            </span>
        </p>
    }
}
