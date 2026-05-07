use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::Theme;
use leptos::ev::{KeyboardEvent, MouseEvent};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use js_sys::Reflect;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn open_url_in_browser(url: &str) {
    let url = url.to_string();
    spawn_local(async move {
        let args = js_sys::Object::new();
        let _ = Reflect::set(&args, &JsValue::from_str("url"), &JsValue::from_str(&url));
        let _ = invoke("open_url", JsValue::from(args)).await;
    });
}

#[component]
pub fn TitleBar(
    theme: ReadSignal<Theme>,
    set_theme: WriteSignal<Theme>,
    set_open_dialog_trigger: WriteSignal<u32>,
    set_show_shortcuts: WriteSignal<bool>,
    set_show_about: WriteSignal<bool>,
    filter_open: ReadSignal<bool>,
    set_filter_open: WriteSignal<bool>,
    set_ai_open: WriteSignal<bool>,
) -> impl IntoView {
    let (active_menu, set_active_menu) = signal(None::<&'static str>);
    let toggle_menu = move |menu: &'static str| {
        set_active_menu.update(|cur| {
            *cur = if *cur == Some(menu) { None } else { Some(menu) };
        });
    };
    let close_menu = move |_: MouseEvent| set_active_menu.set(None);
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };

    // Global shortcuts
    window_event_listener(leptos::ev::keydown, move |ev: KeyboardEvent| {
        let ctrl = ev.ctrl_key() || ev.meta_key();
        if ctrl {
            match ev.key().as_str() {
                "o" | "O" => { ev.prevent_default(); set_open_dialog_trigger.update(|v| *v = v.wrapping_add(1)); }
                "/" => { ev.prevent_default(); set_show_shortcuts.set(true); }
                "t" | "T" => { ev.prevent_default(); set_theme.update(|t| *t = if *t == Theme::Dark { Theme::Light } else { Theme::Dark }); }
                "b" | "B" => { ev.prevent_default(); set_filter_open.update(|v| *v = !*v); }
                "l" | "L" => { ev.prevent_default(); set_ai_open.update(|v| *v = !*v); }
                _ => {}
            }
        }
    });

    // Styles
    let mbtn = move |name: &'static str| -> String {
        let t = tok();
        let active = active_menu.get() == Some(name);
        if active {
            format!("padding:2px 10px;font-size:12px;font-weight:500;border-radius:5px;\
                     border:none;cursor:pointer;background:{};color:{};transition:all 0.1s;",
                t.bg_elevated, t.text_primary)
        } else {
            format!("padding:2px 10px;font-size:12px;font-weight:500;border-radius:5px;\
                     border:none;cursor:pointer;background:transparent;color:{};transition:all 0.1s;",
                t.text_secondary)
        }
    };

    let dd = move || -> String {
        let t = tok();
        format!("position:absolute;top:calc(100% + 5px);left:0;background:{};\
                 border:1px solid {};border-radius:8px;\
                 box-shadow:0 16px 48px rgba(0,0,0,0.55);z-index:200;\
                 min-width:210px;padding:4px 0;overflow:hidden;",
            t.bg_elevated, t.border)
    };

    let ditem = move || -> String {
        let t = tok();
        format!("display:flex;align-items:center;gap:9px;width:100%;text-align:left;\
                 padding:7px 12px;font-size:12.5px;color:{};background:transparent;\
                 border:none;cursor:pointer;transition:background 0.08s,color 0.08s;",
            t.text_primary)
    };

    let kbd = move || -> String {
        let t = tok();
        format!("margin-left:auto;font-size:10px;color:{};background:{};\
                 border:1px solid {};border-radius:4px;padding:1px 6px;\
                 font-family:'Fira Code',monospace;white-space:nowrap;",
            t.text_muted, t.bg_input, t.border)
    };

    let hr_s = move || -> String {
        format!("border:none;border-top:1px solid {};margin:3px 0;", tok().border)
    };

    let icon_btn = move || -> String {
        let t = tok();
        format!("width:28px;height:28px;display:flex;align-items:center;justify-content:center;\
                 border-radius:5px;border:none;background:transparent;color:{};\
                 cursor:pointer;transition:all 0.1s;", t.text_secondary)
    };

    view! {
        <div style=move || format!(
            "background:{};border-bottom:1px solid {};display:flex;align-items:center;\
             height:32px;padding:0 8px;gap:0;flex-shrink:0;z-index:20;",
            tok().bg_surface, tok().border
        )>

            // ── File ──────────────────────────────────────────────────────
            <div style="position:relative">
                <button style=move || mbtn("file") on:click=move |_| toggle_menu("file")
                    on:mouseenter=move |_| { if active_menu.get().is_some() { set_active_menu.set(Some("file")); } }>
                    "File"
                </button>
                <Show when=move || active_menu.get() == Some("file")>
                    <div style=move || dd() on:mouseleave=close_menu>
                        <button style=move || ditem() on:click=move |_| {
                            set_open_dialog_trigger.update(|v| *v = v.wrapping_add(1));
                            set_active_menu.set(None);
                        }>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M1 3.5A1.5 1.5 0 0 1 2.5 2h2.764c.958 0 1.76.56 2.311 1.184C7.985 3.648 8.48 4 9 4h4.5A1.5 1.5 0 0 1 15 5.5v.64c.57.265.94.876.856 1.546l-.64 5.124A2.5 2.5 0 0 1 12.733 15H3.266a2.5 2.5 0 0 1-2.481-2.19l-.64-5.124A1.5 1.5 0 0 1 1 6.14V3.5zM2 6h12v-.5a.5.5 0 0 0-.5-.5H9c-.964 0-1.71-.629-2.174-1.154C6.374 3.334 5.82 3 5.264 3H2.5a.5.5 0 0 0-.5.5V6z"/>
                            </svg>
                            "Open…"
                            <span style=move || kbd()>"Ctrl+O"</span>
                        </button>
                        <hr style=move || hr_s() />
                        <button style=move || ditem()>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M4.5 11a.5.5 0 0 0 0 1h7a.5.5 0 0 0 0-1h-7zm-2-4a.5.5 0 0 0 0 1h11a.5.5 0 0 0 0-1h-11zm-2-4a.5.5 0 0 0 0 1h15a.5.5 0 0 0 0-1H.5z"/>
                            </svg>
                            "Close Tab"
                            <span style=move || kbd()>"Ctrl+W"</span>
                        </button>
                        <hr style=move || hr_s() />
                        <button style=move || ditem()>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M2 10a.5.5 0 0 0 .5.5h9.793l-3.147 3.146a.5.5 0 0 0 .708.708l4-4a.5.5 0 0 0 0-.708l-4-4a.5.5 0 0 0-.708.708L12.293 9.5H2.5A.5.5 0 0 0 2 10z"/>
                            </svg>
                            "Exit"
                        </button>
                    </div>
                </Show>
            </div>

            // ── Edit ──────────────────────────────────────────────────────
            <div style="position:relative">
                <button style=move || mbtn("edit") on:click=move |_| toggle_menu("edit")
                    on:mouseenter=move |_| { if active_menu.get().is_some() { set_active_menu.set(Some("edit")); } }>
                    "Edit"
                </button>
                <Show when=move || active_menu.get() == Some("edit")>
                    <div style=move || dd() on:mouseleave=close_menu>
                        <button style=move || ditem()>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.099zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"/>
                            </svg>
                            "Find"
                            <span style=move || kbd()>"Ctrl+F"</span>
                        </button>
                        <hr style=move || hr_s() />
                        <button style=move || ditem()>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M4 1.5H3a2 2 0 0 0-2 2V14a2 2 0 0 0 2 2h10a2 2 0 0 0 2-2V3.5a2 2 0 0 0-2-2h-1v1h1a1 1 0 0 1 1 1V14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V3.5a1 1 0 0 1 1-1h1v-1z"/>
                                <path d="M9.5 1a.5.5 0 0 1 .5.5v1a.5.5 0 0 1-.5.5h-3a.5.5 0 0 1-.5-.5v-1a.5.5 0 0 1 .5-.5h3zm-3-1A1.5 1.5 0 0 0 5 1.5v1A1.5 1.5 0 0 0 6.5 4h3A1.5 1.5 0 0 0 11 2.5v-1A1.5 1.5 0 0 0 9.5 0h-3z"/>
                            </svg>
                            "Copy Line"
                            <span style=move || kbd()>"Ctrl+C"</span>
                        </button>
                    </div>
                </Show>
            </div>

            // ── View ──────────────────────────────────────────────────────
            <div style="position:relative">
                <button style=move || mbtn("view") on:click=move |_| toggle_menu("view")
                    on:mouseenter=move |_| { if active_menu.get().is_some() { set_active_menu.set(Some("view")); } }>
                    "View"
                </button>
                <Show when=move || active_menu.get() == Some("view")>
                    <div style=move || dd() on:mouseleave=close_menu>
                        <button style=move || ditem() on:click=move |_| {
                            set_filter_open.update(|v| *v = !*v);
                            set_active_menu.set(None);
                        }>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5v-2z"/>
                            </svg>
                            "Toggle Filters"
                            <span style=move || kbd()>"Ctrl+B"</span>
                            {move || if filter_open.get() {
                                view! { <span style="margin-left:4px;color:#7c9dff;font-size:11px">"✓"</span> }.into_any()
                            } else { view! { <span/> }.into_any() }}
                        </button>
                        <button style=move || ditem() on:click=move |_| {
                            set_theme.update(|t| *t = if *t == Theme::Dark { Theme::Light } else { Theme::Dark });
                            set_active_menu.set(None);
                        }>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M8 11a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm0 1a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8z"/>
                            </svg>
                            "Toggle Theme"
                            <span style=move || kbd()>"Ctrl+T"</span>
                        </button>
                    </div>
                </Show>
            </div>

            // ── Help ──────────────────────────────────────────────────────
            <div style="position:relative">
                <button style=move || mbtn("help") on:click=move |_| toggle_menu("help")
                    on:mouseenter=move |_| { if active_menu.get().is_some() { set_active_menu.set(Some("help")); } }>
                    "Help"
                </button>
                <Show when=move || active_menu.get() == Some("help")>
                    <div style=move || dd() on:mouseleave=close_menu>
                        <button style=move || ditem() on:click=move |_| {
                            set_show_shortcuts.set(true);
                            set_active_menu.set(None);
                        }>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M5.255 5.786a.237.237 0 0 0 .241.247h.825c.138 0 .248-.113.266-.25.09-.656.54-1.134 1.342-1.134.686 0 1.314.343 1.314 1.168 0 .635-.374.927-.965 1.371-.673.489-1.206 1.06-1.168 1.987l.003.217a.25.25 0 0 0 .25.246h.811a.25.25 0 0 0 .25-.25v-.105c0-.718.273-.927 1.01-1.486.609-.463 1.244-.977 1.244-2.056 0-1.511-1.276-2.241-2.673-2.241-1.267 0-2.655.59-2.75 2.286zm1.557 5.763c0 .533.425.927 1.01.927.609 0 1.028-.394 1.028-.927 0-.552-.42-.94-1.029-.94-.584 0-1.009.388-1.009.94z"/>
                            </svg>
                            "Keyboard Shortcuts"
                            <span style=move || kbd()>"Ctrl+/"</span>
                        </button>
                        <hr style=move || hr_s() />
                        <button style=move || ditem() on:click=move |_| {
                            open_url_in_browser("https://github.com/mmycin/Logarithm");
                            set_active_menu.set(None);
                        }>
                            <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M8 0C3.58 0 0 3.58 0 8c0 3.54 2.29 6.53 5.47 7.59.4.07.55-.17.55-.38 0-.19-.01-.82-.01-1.49-2.01.37-2.53-.49-2.69-.94-.09-.23-.48-.94-.82-1.13-.28-.15-.68-.52-.01-.53.63-.01 1.08.58 1.23.82.72 1.21 1.87.87 2.33.66.07-.52.28-.87.51-1.07-1.78-.2-3.64-.89-3.64-3.95 0-.87.31-1.59.82-2.15-.08-.2-.36-1.02.08-2.12 0 0 .67-.21 2.2.82.64-.18 1.32-.27 2-.27.68 0 1.36.09 2 .27 1.53-1.04 2.2-.82 2.2-.82.44 1.1.16 1.92.08 2.12.51.56.82 1.27.82 2.15 0 3.07-1.87 3.75-3.65 3.95.29.25.54.73.54 1.48 0 1.07-.01 1.93-.01 2.2 0 .21.15.46.55.38A8.012 8.012 0 0 0 16 8c0-4.42-3.58-8-8-8z"/>
                            </svg>
                            "GitHub"
                            <svg width="10" height="10" viewBox="0 0 16 16" fill="currentColor" style="margin-left:auto;opacity:0.35">
                                <path d="M8.636 3.5a.5.5 0 0 0-.5-.5H1.5A1.5 1.5 0 0 0 0 4.5v10A1.5 1.5 0 0 0 1.5 16h10a1.5 1.5 0 0 0 1.5-1.5V7.864a.5.5 0 0 0-1 0V14.5a.5.5 0 0 1-.5.5h-10a.5.5 0 0 1-.5-.5v-10a.5.5 0 0 1 .5-.5h6.636a.5.5 0 0 0 .5-.5z"/>
                                <path d="M16 .5a.5.5 0 0 0-.5-.5h-5a.5.5 0 0 0 0 1h3.793L6.146 9.146a.5.5 0 1 0 .708.708L15 1.707V5.5a.5.5 0 0 0 1 0v-5z"/>
                            </svg>
                        </button>
                    </div>
                </Show>
            </div>

            // ── AI ────────────────────────────────────────────────────────
            <div style="position:relative">
                <button style=move || {
                    let t = tok();
                    let active = active_menu.get() == Some("ai");
                    let base = if active {
                        format!("padding:2px 10px;font-size:12px;font-weight:600;border-radius:5px;\
                                 border:none;cursor:pointer;transition:all 0.1s;\
                                 background:linear-gradient(135deg,rgba(124,157,255,0.2),rgba(167,139,250,0.2));\
                                 color:#a78bfa;")
                    } else {
                        format!("padding:2px 10px;font-size:12px;font-weight:600;border-radius:5px;\
                                 border:none;cursor:pointer;background:transparent;color:{};\
                                 transition:all 0.1s;", t.text_secondary)
                    };
                    base
                }
                    on:click=move |_| toggle_menu("ai")
                    on:mouseenter=move |_| { if active_menu.get().is_some() { set_active_menu.set(Some("ai")); } }
                >
                    // Sparkle + text
                    <span style="display:flex;align-items:center;gap:4px">
                        "AI"
                    </span>
                </button>
                <Show when=move || active_menu.get() == Some("ai")>
                    <div style=move || dd() on:mouseleave=close_menu>
                        <div style=move || format!(
                            "padding:8px 12px 6px;display:flex;align-items:center;gap:8px;\
                             border-bottom:1px solid {};margin-bottom:2px;",
                            tok().border
                        )>
                            <div style="width:20px;height:20px;border-radius:6px;\
                                background:linear-gradient(135deg,#7c9dff,#a78bfa);\
                                display:flex;align-items:center;justify-content:center;flex-shrink:0">
                                <img src="/public/LoganIcon.png" width="11" height="11" style="border-radius:2px;opacity:0.7" alt="Logan" />
                            </div>
                            <div>
                                <div style=move || format!("font-size:12px;font-weight:700;color:{}", tok().text_primary)>"Logan AI"</div>
                                <div style=move || format!("font-size:10px;color:{}", tok().text_muted)>"Intelligent log analysis"</div>
                            </div>
                        </div>
                        <button style=move || ditem()
                            on:click=move |_| {
                                // Trigger AI panel open via bottom bar — we just close menu
                                // The actual toggle is in BottomBar; here we signal via a custom event
                                if let Some(doc) = web_sys::window().and_then(|w| w.document()) {
                                    let _ = doc.query_selector("[title='Toggle AI Assistant (Ctrl+L)']")
                                        .ok().flatten()
                                        .and_then(|el| el.dyn_into::<web_sys::HtmlElement>().ok())
                                        .map(|el| el.click());
                                }
                                set_active_menu.set(None);
                            }
                        >
                            <img src="/public/LoganIcon.png" width="13" height="13" style="border-radius:2px;opacity:0.5" alt="Logan" />
                            "Open Logan Chat"
                            <span style=move || kbd()>"Ctrl+Shift+A"</span>
                        </button>
                        <hr style=move || hr_s() />
                        <div style=move || format!(
                            "padding:6px 12px 8px;font-size:11px;color:{};line-height:1.5;",
                            tok().text_muted
                        )>
                            "AI-powered log analysis coming soon. Logan will help you detect anomalies, find patterns, and understand root causes."
                        </div>
                    </div>
                </Show>
            </div>

            // ── About ─────────────────────────────────────────────────────
            <div style="position:relative">
                <button style=move || mbtn("about") on:click=move |_| {
                    set_show_about.set(true);
                    set_active_menu.set(None);
                }
                    on:mouseenter=move |_| { if active_menu.get().is_some() { set_active_menu.set(Some("about")); } }>
                    "About"
                </button>
            </div>

            // ── Settings ──────────────────────────────────────────────────
            <div style="position:relative;margin-left:auto">
                <button style=move || icon_btn() on:click=move |_| toggle_menu("settings") title="Settings">
                    <svg width="14" height="14" viewBox="0 0 16 16" fill="currentColor">
                        <path d="M9.405 1.05c-.413-1.4-2.397-1.4-2.81 0l-.1.34a1.464 1.464 0 0 1-2.105.872l-.31-.17c-1.283-.698-2.686.705-1.987 1.987l.169.311c.446.82.023 1.841-.872 2.105l-.34.1c-1.4.413-1.4 2.397 0 2.81l.34.1a1.464 1.464 0 0 1 .872 2.105l-.17.31c-.698 1.283.705 2.686 1.987 1.987l.311-.169a1.464 1.464 0 0 1 2.105.872l.1.34c.413 1.4 2.397 1.4 2.81 0l.1-.34a1.464 1.464 0 0 1 2.105-.872l.31.17c1.283.698 2.686-.705 1.987-1.987l-.169-.311a1.464 1.464 0 0 1 .872-2.105l.34-.1c1.4-.413 1.4-2.397 0-2.81l-.34-.1a1.464 1.464 0 0 1-.872-2.105l.17-.31c.698-1.283-.705-2.686-1.987-1.987l-.311.169a1.464 1.464 0 0 1-2.105-.872l-.1-.34zM8 10.93a2.929 2.929 0 1 1 0-5.86 2.929 2.929 0 0 1 0 5.858z"/>
                    </svg>
                </button>
                <Show when=move || active_menu.get() == Some("settings")>
                    <div style=move || {
                        let t = tok();
                        format!("position:absolute;top:calc(100% + 5px);right:0;background:{};\
                                 border:1px solid {};border-radius:8px;\
                                 box-shadow:0 16px 48px rgba(0,0,0,0.55);z-index:200;\
                                 min-width:180px;padding:4px 0;overflow:hidden;",
                            t.bg_elevated, t.border)
                    } on:mouseleave=close_menu>
                        <div style=move || format!(
                            "padding:6px 12px 4px;font-size:10px;font-weight:700;\
                             color:{};letter-spacing:0.08em;text-transform:uppercase;",
                            tok().text_muted)>
                            "Appearance"
                        </div>
                        <button style=move || ditem() on:click=move |_| { set_theme.set(Theme::Dark); set_active_menu.set(None); }>
                            <svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M6 .278a.768.768 0 0 1 .08.858 7.208 7.208 0 0 0-.878 3.46c0 4.021 3.278 7.277 7.318 7.277.527 0 1.04-.055 1.533-.16a.787.787 0 0 1 .81.316.733.733 0 0 1-.031.893A8.349 8.349 0 0 1 8.344 16C3.734 16 0 12.286 0 7.71 0 4.266 2.114 1.312 5.124.06A.752.752 0 0 1 6 .278z"/>
                            </svg>
                            "Dark"
                            {move || if theme.get() == Theme::Dark {
                                view! { <span style="margin-left:auto;color:#7c9dff;font-size:12px">"✓"</span> }.into_any()
                            } else { view! { <span/> }.into_any() }}
                        </button>
                        <button style=move || ditem() on:click=move |_| { set_theme.set(Theme::Light); set_active_menu.set(None); }>
                            <svg width="13" height="13" viewBox="0 0 16 16" fill="currentColor" style="opacity:0.5;flex-shrink:0">
                                <path d="M8 11a3 3 0 1 1 0-6 3 3 0 0 1 0 6zm0 1a4 4 0 1 0 0-8 4 4 0 0 0 0 8zM8 0a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 0zm0 13a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-1 0v-2A.5.5 0 0 1 8 13zm8-5a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2a.5.5 0 0 1 .5.5zM3 8a.5.5 0 0 1-.5.5h-2a.5.5 0 0 1 0-1h2A.5.5 0 0 1 3 8z"/>
                            </svg>
                            "Light"
                            {move || if theme.get() == Theme::Light {
                                view! { <span style="margin-left:auto;color:#7c9dff;font-size:12px">"✓"</span> }.into_any()
                            } else { view! { <span/> }.into_any() }}
                        </button>
                    </div>
                </Show>
            </div>
        </div>
    }
}
