use crate::app::FilterState;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LogFile, Theme};
use crate::components::filter_section::FilterSection;
use crate::components::severity::{LEVEL_PILLS, MIN_SEV};
use leptos::prelude::*;

#[component]
pub fn FilterPanel(
    theme: ReadSignal<Theme>,
    filter_state: FilterState,
    open_files: ReadSignal<Vec<LogFile>>,
    active_file: ReadSignal<Option<usize>>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let fs = filter_state;

    // ── Shared style helpers ──────────────────────────────────────────────

    let inp = move || format!(
        "width:100%;height:27px;padding:0 9px;background:{};border:1px solid {};\
         border-radius:6px;font-size:12px;color:{};outline:none;\
         font-family:'Inter',sans-serif;box-sizing:border-box;cursor:text;",
        tok().bg_input, tok().border, tok().text_primary
    );

    let small_inp = move || format!(
        "width:100%;height:25px;padding:0 8px;background:{};border:1px solid {};\
         border-radius:5px;font-size:11.5px;color:{};outline:none;\
         font-family:'Inter',sans-serif;box-sizing:border-box;cursor:text;",
        tok().bg_input, tok().border, tok().text_primary
    );

    let knob_wrap = move |active: bool| -> String {
        let t = tok();
        format!(
            "width:30px;height:16px;border-radius:8px;background:{};display:flex;\
             align-items:center;padding:2px;transition:background 0.2s;flex-shrink:0;cursor:pointer;",
            if active { t.accent } else { t.border }
        )
    };
    let knob_dot = |active: bool| -> &'static str {
        if active {
            "width:12px;height:12px;border-radius:50%;background:white;margin-left:auto;transition:margin 0.2s;"
        } else {
            "width:12px;height:12px;border-radius:50%;background:white;margin-left:0;transition:margin 0.2s;"
        }
    };

    let trow = move || format!(
        "display:flex;align-items:center;justify-content:space-between;padding:4px 14px;cursor:pointer;"
    );

    let active_count = move || {
        let mut n = 0u32;
        if fs.selected_status.get() != "all" { n += 1; }
        if !fs.search_query.get().is_empty() { n += 1; }
        if !fs.from_datetime.get().is_empty() || !fs.to_datetime.get().is_empty() { n += 1; }
        if !fs.line_from.get().is_empty() || !fs.line_to.get().is_empty() { n += 1; }
        if fs.hide_no_level.get() { n += 1; }
        if fs.min_severity.get() != "all" { n += 1; }
        n
    };

    // Pre-compute static color strings from current theme at render time.
    // These are used as `&'static str` props on FilterSection.
    // Since theme can change, we use a reactive wrapper that re-renders the panel.
    let muted  = move || tok().text_muted;
    let border = move || tok().border_subtle;
    let accent = move || tok().accent;
    let abg    = move || tok().accent_bg;

    view! {
        <div style=move || format!(
            "width:240px;flex-shrink:0;display:flex;flex-direction:column;\
             background:{};border-right:1px solid {};overflow-y:auto;overflow-x:hidden;",
            tok().bg_surface, tok().border
        )>

            // ── Panel header ──────────────────────────────────────────────
            <div style=move || format!(
                "display:flex;align-items:center;justify-content:space-between;\
                 padding:9px 14px;border-bottom:1px solid {};flex-shrink:0;",
                tok().border
            )>
                <div style="display:flex;align-items:center;gap:7px">
                    <svg width="12" height="12" viewBox="0 0 16 16" fill="currentColor"
                        style=move || format!("color:{}", tok().accent)>
                        <path d="M1.5 1.5A.5.5 0 0 1 2 1h12a.5.5 0 0 1 .5.5v2a.5.5 0 0 1-.128.334L10 8.692V13.5a.5.5 0 0 1-.342.474l-3 1A.5.5 0 0 1 6 14.5V8.692L1.628 3.834A.5.5 0 0 1 1.5 3.5v-2z"/>
                    </svg>
                    <span style=move || format!(
                        "font-size:12px;font-weight:600;color:{}", tok().text_primary
                    )>"Filters"</span>
                    {move || {
                        let n = active_count();
                        if n > 0 {
                            view! {
                                <span style=move || format!(
                                    "font-size:9.5px;font-weight:700;color:{};background:{};\
                                     border-radius:8px;padding:1px 6px;",
                                    tok().accent, tok().accent_bg
                                )>{n}</span>
                            }.into_any()
                        } else { view! { <span/> }.into_any() }
                    }}
                </div>
                <button
                    style=move || format!(
                        "font-size:11px;color:{};background:transparent;border:none;\
                         cursor:pointer;padding:2px 6px;border-radius:4px;",
                        tok().text_muted
                    )
                    on:click=move |_| {
                        fs.set_selected_status.set("all".to_string());
                        fs.set_search_query.set(String::new());
                        fs.set_from_datetime.set(String::new());
                        fs.set_to_datetime.set(String::new());
                        fs.set_line_from.set(String::new());
                        fs.set_line_to.set(String::new());
                        fs.set_hide_no_level.set(false);
                        fs.set_min_severity.set("all".to_string());
                        fs.set_invert_match.set(false);
                        fs.set_regex_mode.set(false);
                        fs.set_match_case.set(false);
                        fs.set_fuzzy_find.set(false);
                    }
                    title="Clear all filters"
                >"Clear all"</button>
            </div>

            // ── 1. Search ─────────────────────────────────────────────────
            {move || {
                let m = muted(); let b = border(); let a = accent(); let ab = abg();
                let badge_sig: Signal<Option<String>> = Signal::derive(move || {
                    let q = fs.search_query.get();
                    if q.is_empty() { None } else { Some("●".to_string()) }
                });
                view! {
                    <FilterSection title="Search" badge=badge_sig
                        muted=m border=b accent=a accent_bg=ab>
                        <div style="padding:0 10px 8px;display:flex;flex-direction:column;gap:5px">
                            <div style="position:relative">
                                <svg width="11" height="11" viewBox="0 0 16 16" fill="currentColor"
                                    style=move || format!(
                                        "position:absolute;left:8px;top:50%;transform:translateY(-50%);\
                                         color:{};pointer-events:none;", tok().text_muted
                                    )>
                                    <path d="M11.742 10.344a6.5 6.5 0 1 0-1.397 1.398h-.001l3.85 3.85a1 1 0 0 0 1.415-1.414l-3.85-3.85a1.007 1.007 0 0 0-.115-.099zM12 6.5a5.5 5.5 0 1 1-11 0 5.5 5.5 0 0 1 11 0z"/>
                                </svg>
                                <input type="text" placeholder="Search messages…"
                                    style=move || format!(
                                        "width:100%;height:27px;padding:0 9px 0 26px;background:{};\
                                         border:1px solid {};border-radius:6px;font-size:12px;color:{};\
                                         outline:none;font-family:'Inter',sans-serif;box-sizing:border-box;cursor:text;",
                                        tok().bg_input, tok().border, tok().text_primary
                                    )
                                    prop:value=move || fs.search_query.get()
                                    on:input=move |ev| fs.set_search_query.set(event_target_value(&ev))
                                />
                            </div>
                            <div style="display:grid;grid-template-columns:1fr 1fr;gap:3px">
                                {[
                                    ("Aa", "Match case",  fs.match_case,   fs.set_match_case),
                                    (".*", "Fuzzy",       fs.fuzzy_find,   fs.set_fuzzy_find),
                                    ("Re", "Regex",       fs.regex_mode,   fs.set_regex_mode),
                                    ("¬",  "Invert",      fs.invert_match, fs.set_invert_match),
                                ].into_iter().map(|(sym, tip, sig, setter)| {
                                    view! {
                                        <button
                                            style=move || {
                                                let t = tok();
                                                if sig.get() {
                                                    format!("display:flex;align-items:center;gap:4px;padding:4px 7px;\
                                                             border-radius:5px;font-size:11px;font-weight:700;\
                                                             border:1px solid {};background:{};color:{};\
                                                             cursor:pointer;transition:all 0.1s;",
                                                        t.accent_border, t.accent_bg, t.accent)
                                                } else {
                                                    format!("display:flex;align-items:center;gap:4px;padding:4px 7px;\
                                                             border-radius:5px;font-size:11px;font-weight:500;\
                                                             border:1px solid {};background:transparent;color:{};\
                                                             cursor:pointer;transition:all 0.1s;",
                                                        t.border, t.text_muted)
                                                }
                                            }
                                            on:click=move |_| setter.update(|v| *v = !*v)
                                            title=tip
                                        >
                                            <span style="font-family:'Fira Code',monospace">{sym}</span>
                                            {tip}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                            <div style=move || trow()
                                on:click=move |_| fs.set_search_in_datetime.update(|v| *v = !*v)>
                                <span style=move || format!("font-size:11.5px;color:{}", tok().text_secondary)>
                                    "Include timestamp"
                                </span>
                                <div style=move || knob_wrap(fs.search_in_datetime.get())>
                                    <div style=move || knob_dot(fs.search_in_datetime.get())/>
                                </div>
                            </div>
                        </div>
                    </FilterSection>
                }
            }}

            // ── 2. Level ──────────────────────────────────────────────────
            {move || {
                let m = muted(); let b = border(); let a = accent(); let ab = abg();
                let badge_sig: Signal<Option<String>> = Signal::derive(move || {
                    let s = fs.selected_status.get();
                    if s == "all" { None } else { Some(s) }
                });
                view! {
                    <FilterSection title="Level" badge=badge_sig
                        muted=m border=b accent=a accent_bg=ab>
                        <div style="padding:0 10px 8px;display:flex;flex-wrap:wrap;gap:3px">
                            {LEVEL_PILLS.iter().map(|(val, label, color)| {
                                let val = *val; let label = *label; let color = *color;
                                view! {
                                    <button
                                        style=move || {
                                            let t = tok();
                                            if fs.selected_status.get() == val {
                                                format!("padding:3px 9px;border-radius:5px;font-size:11.5px;\
                                                         font-weight:600;border:1px solid {color}55;\
                                                         background:{color}18;color:{color};\
                                                         cursor:pointer;transition:all 0.1s;")
                                            } else {
                                                format!("padding:3px 9px;border-radius:5px;font-size:11.5px;\
                                                         font-weight:500;border:1px solid transparent;\
                                                         background:transparent;color:{};\
                                                         cursor:pointer;transition:all 0.1s;",
                                                    t.text_secondary)
                                            }
                                        }
                                        on:click=move |_| fs.set_selected_status.set(val.to_string())
                                    >{label}</button>
                                }
                            }).collect_view()}
                            <button
                                style=move || {
                                    let t = tok();
                                    if fs.selected_status.get() == "custom" {
                                        "padding:3px 9px;border-radius:5px;font-size:11.5px;font-weight:600;\
                                         border:1px solid #a78bfa55;background:#a78bfa18;color:#a78bfa;\
                                         cursor:pointer;transition:all 0.1s;".to_string()
                                    } else {
                                        format!("padding:3px 9px;border-radius:5px;font-size:11.5px;font-weight:500;\
                                                 border:1px solid transparent;background:transparent;color:{};\
                                                 cursor:pointer;transition:all 0.1s;", t.text_secondary)
                                    }
                                }
                                on:click=move |_| fs.set_selected_status.set("custom".to_string())
                            >"Custom"</button>
                        </div>
                        <Show when=move || fs.selected_status.get() == "custom">
                            <div style="padding:0 10px 8px">
                                <input type="text" placeholder="INFO, WARN, FATAL…"
                                    style=move || inp()
                                    prop:value=move || fs.custom_status.get()
                                    on:input=move |ev| fs.set_custom_status.set(event_target_value(&ev))
                                />
                                <div style=move || format!(
                                    "font-size:10px;color:{};margin-top:3px;padding:0 2px;",
                                    tok().text_muted
                                )>"Comma-separated level names"</div>
                            </div>
                        </Show>
                        <div style=move || trow()
                            on:click=move |_| fs.set_inherit_level.update(|v| *v = !*v)>
                            <span style=move || format!("font-size:11.5px;color:{}", tok().text_secondary)>
                                "Inherit (continuation lines)"
                            </span>
                            <div style=move || knob_wrap(fs.inherit_level.get())>
                                <div style=move || knob_dot(fs.inherit_level.get())/>
                            </div>
                        </div>
                    </FilterSection>
                }
            }}

            // ── 3. Date & Time ────────────────────────────────────────────
            {move || {
                let m = muted(); let b = border(); let a = accent(); let ab = abg();
                let badge_sig: Signal<Option<String>> = Signal::derive(move || {
                    let has = !fs.from_datetime.get().is_empty() || !fs.to_datetime.get().is_empty();
                    if has { Some("●".to_string()) } else { None }
                });
                view! {
                    <FilterSection title="Date & Time" badge=badge_sig
                        muted=m border=b accent=a accent_bg=ab>
                        <div style="padding:0 10px 8px;display:flex;flex-direction:column;gap:5px">
                            <div style=move || format!(
                                "background:{};border:1px solid {};border-radius:7px;padding:5px 9px;",
                                tok().bg_input, tok().border
                            )>
                                <div style=move || format!(
                                    "font-size:9px;font-weight:700;color:{};letter-spacing:0.09em;\
                                     text-transform:uppercase;margin-bottom:2px;", tok().text_muted
                                )>"From"</div>
                                <input type="datetime-local"
                                    style=move || format!(
                                        "border:none;background:transparent;font-size:11.5px;color:{};\
                                         outline:none;font-family:'Inter',sans-serif;width:100%;padding:0;cursor:pointer;",
                                        tok().text_primary
                                    )
                                    prop:value=move || fs.from_datetime.get()
                                    on:input=move |ev| fs.set_from_datetime.set(event_target_value(&ev))
                                />
                            </div>
                            <div style=move || format!(
                                "background:{};border:1px solid {};border-radius:7px;padding:5px 9px;",
                                tok().bg_input, tok().border
                            )>
                                <div style=move || format!(
                                    "font-size:9px;font-weight:700;color:{};letter-spacing:0.09em;\
                                     text-transform:uppercase;margin-bottom:2px;", tok().text_muted
                                )>"To"</div>
                                <input type="datetime-local"
                                    style=move || format!(
                                        "border:none;background:transparent;font-size:11.5px;color:{};\
                                         outline:none;font-family:'Inter',sans-serif;width:100%;padding:0;cursor:pointer;",
                                        tok().text_primary
                                    )
                                    prop:value=move || fs.to_datetime.get()
                                    on:input=move |ev| fs.set_to_datetime.set(event_target_value(&ev))
                                />
                            </div>
                            <Show when=move || !fs.from_datetime.get().is_empty() || !fs.to_datetime.get().is_empty()>
                                <button
                                    style=move || format!(
                                        "padding:4px 8px;border-radius:5px;border:1px solid {};background:transparent;\
                                         color:{};font-size:11px;cursor:pointer;text-align:left;",
                                        tok().border, tok().text_muted
                                    )
                                    on:click=move |_| {
                                        fs.set_from_datetime.set(String::new());
                                        fs.set_to_datetime.set(String::new());
                                    }
                                >"✕  Clear date range"</button>
                            </Show>
                        </div>
                    </FilterSection>
                }
            }}

            // ── 4. Line Range ─────────────────────────────────────────────
            {move || {
                let m = muted(); let b = border(); let a = accent(); let ab = abg();
                let badge_sig: Signal<Option<String>> = Signal::derive(move || {
                    let has = !fs.line_from.get().is_empty() || !fs.line_to.get().is_empty();
                    if has { Some("●".to_string()) } else { None }
                });
                view! {
                    <FilterSection title="Line Range" badge=badge_sig
                        muted=m border=b accent=a accent_bg=ab>
                        <div style="padding:0 10px 8px;display:flex;gap:6px;align-items:flex-end">
                            <div style="flex:1">
                                <div style=move || format!(
                                    "font-size:9px;font-weight:700;color:{};letter-spacing:0.08em;\
                                     text-transform:uppercase;margin-bottom:3px;", tok().text_muted
                                )>"From"</div>
                                <input type="number" placeholder="1" min="1"
                                    style=move || small_inp()
                                    prop:value=move || fs.line_from.get()
                                    on:input=move |ev| fs.set_line_from.set(event_target_value(&ev))
                                />
                            </div>
                            <span style=move || format!(
                                "font-size:13px;color:{};padding-bottom:4px;flex-shrink:0;", tok().text_muted
                            )>"–"</span>
                            <div style="flex:1">
                                <div style=move || format!(
                                    "font-size:9px;font-weight:700;color:{};letter-spacing:0.08em;\
                                     text-transform:uppercase;margin-bottom:3px;", tok().text_muted
                                )>"To"</div>
                                <input type="number" placeholder="∞" min="1"
                                    style=move || small_inp()
                                    prop:value=move || fs.line_to.get()
                                    on:input=move |ev| fs.set_line_to.set(event_target_value(&ev))
                                />
                            </div>
                        </div>
                    </FilterSection>
                }
            }}

            // ── 5. Advanced ───────────────────────────────────────────────
            {move || {
                let m = muted(); let b = border(); let a = accent(); let ab = abg();
                view! {
                    <FilterSection title="Advanced" badge=Signal::derive(|| None)
                        muted=m border=b accent=a accent_bg=ab>
                        <div style="padding:0 0 8px">
                            <div style="padding:0 10px 8px">
                                <div style=move || format!(
                                    "font-size:10.5px;color:{};margin-bottom:5px;", tok().text_secondary
                                )>"Minimum severity"</div>
                                <div style="display:flex;flex-wrap:wrap;gap:3px">
                                    {MIN_SEV.iter().map(|(val, label, color)| {
                                        let val = *val; let label = *label; let color = *color;
                                        view! {
                                            <button
                                                style=move || {
                                                    let t = tok();
                                                    if fs.min_severity.get() == val {
                                                        format!("padding:3px 8px;border-radius:5px;font-size:11px;\
                                                                 font-weight:600;border:1px solid {color}55;\
                                                                 background:{color}18;color:{color};cursor:pointer;")
                                                    } else {
                                                        format!("padding:3px 8px;border-radius:5px;font-size:11px;\
                                                                 font-weight:500;border:1px solid {};background:transparent;\
                                                                 color:{};cursor:pointer;",
                                                            t.border, t.text_muted)
                                                    }
                                                }
                                                on:click=move |_| fs.set_min_severity.set(val.to_string())
                                            >{label}</button>
                                        }
                                    }).collect_view()}
                                </div>
                            </div>
                            <div style=move || trow()
                                on:click=move |_| fs.set_hide_no_level.update(|v| *v = !*v)>
                                <span style=move || format!("font-size:11.5px;color:{}", tok().text_secondary)>
                                    "Hide lines without level"
                                </span>
                                <div style=move || knob_wrap(fs.hide_no_level.get())>
                                    <div style=move || knob_dot(fs.hide_no_level.get())/>
                                </div>
                            </div>
                        </div>
                    </FilterSection>
                }
            }}

            // ── 6. Open Files ─────────────────────────────────────────────
            {move || {
                let m = muted(); let b = border(); let a = accent(); let ab = abg();
                view! {
                    <FilterSection title="Open Files" badge=Signal::derive(|| None)
                        muted=m border=b accent=a accent_bg=ab>
                        <div style="padding:0 10px 10px;display:flex;flex-direction:column;gap:2px">
                            {move || {
                                let files = open_files.get();
                                if files.is_empty() {
                                    return view! {
                                        <span style=move || format!(
                                            "font-size:11.5px;color:{};font-style:italic;padding:4px 2px;",
                                            tok().text_muted
                                        )>"No files open"</span>
                                    }.into_any();
                                }
                                files.into_iter().enumerate().map(|(idx, file)| {
                                    let is_active = move || active_file.get() == Some(idx);
                                    view! {
                                        <div style=move || {
                                            let t = tok();
                                            if is_active() {
                                                format!("display:flex;align-items:center;gap:7px;padding:5px 8px;\
                                                         border-radius:6px;background:{};border:1px solid {};cursor:default;",
                                                    t.bg_active, t.border)
                                            } else {
                                                "display:flex;align-items:center;gap:7px;padding:5px 8px;\
                                                 border-radius:6px;background:transparent;border:1px solid transparent;\
                                                 cursor:default;".to_string()
                                            }
                                        }>
                                            <svg viewBox="0 0 16 16" fill="currentColor"
                                                style=move || format!(
                                                    "width:11px;height:11px;flex-shrink:0;color:{}",
                                                    if is_active() { tok().accent } else { tok().text_muted }
                                                )>
                                                <path d="M4 0h5.293A1 1 0 0 1 10 .293L13.707 4a1 1 0 0 1 .293.707V14a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V2a2 2 0 0 1 2-2zm5.5 1.5v2a1 1 0 0 0 1 1h2L9.5 1.5z"/>
                                            </svg>
                                            <span style=move || format!(
                                                "font-size:11.5px;color:{};overflow:hidden;text-overflow:ellipsis;\
                                                 white-space:nowrap;flex:1;",
                                                if is_active() { tok().text_primary } else { tok().text_secondary }
                                            )>{file.name.clone()}</span>
                                            <span style=move || format!(
                                                "font-size:10px;color:{};flex-shrink:0;", tok().text_muted
                                            )>{file.entries.len()}</span>
                                        </div>
                                    }
                                }).collect_view().into_any()
                            }}
                        </div>
                    </FilterSection>
                }
            }}
        </div>
    }
}
