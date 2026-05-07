/// Main file viewer component.
/// 
/// Orchestrates the log file viewing experience with filtering and context menu.

use super::{ContextMenu, WelcomePage};
use crate::app::FilterState;
use crate::components::severity::level_colors;
use crate::shared::constants::{DARK, LIGHT};
use crate::shared::types::{LoganAction, LogFile, Theme};
use crate::viewer::filters::apply_filters;
use crate::viewer::rendering::assign_groups;
use crate::viewer::types::{DisplayEntry, FilterParams};
use leptos::prelude::*;

#[component]
pub fn FileViewer(
    theme: ReadSignal<Theme>,
    open_files: ReadSignal<Vec<LogFile>>,
    active_file: ReadSignal<Option<usize>>,
    filter_state: FilterState,
    set_logan_action: WriteSignal<Option<LoganAction>>,
    set_ai_open: WriteSignal<bool>,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    let dark = move || theme.get() == Theme::Dark;

    // Context menu state
    let (ctx_visible, set_ctx_visible) = signal(false);
    let (ctx_x, set_ctx_x) = signal(0i32);
    let (ctx_y, set_ctx_y) = signal(0i32);
    let (ctx_entry, set_ctx_entry) = signal(None::<(String, usize, String)>);

    // Selection state
    let (focused_line, set_focused_line) = signal(None::<usize>);
    let (selected_lines, set_selected_lines) = signal(Vec::<usize>::new());

    let selected_file = move || -> Option<LogFile> {
        active_file
            .get()
            .and_then(|idx| open_files.get().get(idx).cloned())
    };

    let display_entries = move || -> Vec<DisplayEntry> {
        let Some(file) = selected_file() else {
            return vec![];
        };
        
        let lf = filter_state.line_from.get().parse::<usize>().unwrap_or(0);
        let lt = filter_state.line_to.get().parse::<usize>().unwrap_or(0);
        
        let filtered = apply_filters(
            &file.entries,
            &FilterParams {
                status: filter_state.selected_status.get(),
                custom_status: filter_state.custom_status.get(),
                inherit_level: filter_state.inherit_level.get(),
                query: filter_state.search_query.get(),
                match_case: filter_state.match_case.get(),
                fuzzy: filter_state.fuzzy_find.get(),
                regex_mode: filter_state.regex_mode.get(),
                invert_match: filter_state.invert_match.get(),
                search_in_datetime: filter_state.search_in_datetime.get(),
                from_datetime: filter_state.from_datetime.get(),
                to_datetime: filter_state.to_datetime.get(),
                line_from: lf,
                line_to: lt,
                hide_no_level: filter_state.hide_no_level.get(),
                min_severity: filter_state.min_severity.get(),
            },
        );
        
        assign_groups(filtered)
    };

    view! {
        <div
            style=move || format!(
                "flex:1;display:flex;flex-direction:column;background:{};overflow:hidden;position:relative;",
                tok().bg_base
            )
            on:click=move |_| set_ctx_visible.set(false)
            on:contextmenu=move |ev| ev.prevent_default()
        >
            {move || {
                if selected_file().is_none() {
                    view! { <WelcomePage theme /> }.into_any()
                } else {
                    view! {
                        <div style="flex:1;display:flex;flex-direction:column;overflow:hidden">
                            // Column header
                            <ColumnHeader theme=theme display_entries=display_entries selected_file=selected_file />
                            
                            // Log rows
                            <LogRows
                                theme=theme
                                display_entries=display_entries
                                focused_line=focused_line
                                set_focused_line=set_focused_line
                                selected_lines=selected_lines
                                set_selected_lines=set_selected_lines
                                set_ctx_x=set_ctx_x
                                set_ctx_y=set_ctx_y
                                set_ctx_entry=set_ctx_entry
                                set_ctx_visible=set_ctx_visible
                                selected_file=selected_file
                                dark=dark
                            />
                        </div>
                    }.into_any()
                }
            }}

            // Context menu
            <Show when=move || ctx_visible.get()>
                <ContextMenu
                    theme=theme
                    ctx_x=ctx_x
                    ctx_y=ctx_y
                    ctx_entry=ctx_entry
                    selected_lines=selected_lines
                    set_selected_lines=set_selected_lines
                    set_ctx_visible=set_ctx_visible
                    display_entries=display_entries
                    selected_file=selected_file
                    set_logan_action=set_logan_action
                    set_ai_open=set_ai_open
                />
            </Show>
        </div>
    }
}

#[component]
fn ColumnHeader(
    theme: ReadSignal<Theme>,
    display_entries: impl Fn() -> Vec<DisplayEntry> + 'static + Copy + Send,
    selected_file: impl Fn() -> Option<LogFile> + 'static + Copy + Send,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
        <div style=move || format!(
            "display:flex;align-items:center;padding:0 12px;height:26px;\
             border-bottom:1px solid {};background:{};flex-shrink:0;",
            tok().border_subtle, tok().bg_surface
        )>
            <span style=move || format!(
                "width:44px;text-align:right;font-size:10px;font-weight:600;\
                 color:{};margin-right:12px;flex-shrink:0;letter-spacing:0.06em;",
                tok().text_muted
            )>"LINE"</span>
            <span style=move || format!(
                "width:160px;font-size:10px;font-weight:600;color:{};\
                 margin-right:12px;flex-shrink:0;letter-spacing:0.06em;",
                tok().text_muted
            )>"TIMESTAMP"</span>
            <span style=move || format!(
                "width:72px;font-size:10px;font-weight:600;color:{};\
                 margin-right:12px;flex-shrink:0;letter-spacing:0.06em;",
                tok().text_muted
            )>"LEVEL"</span>
            <span style=move || format!(
                "font-size:10px;font-weight:600;color:{};letter-spacing:0.06em;",
                tok().text_muted
            )>"MESSAGE"</span>
            <span style=move || format!(
                "margin-left:auto;font-size:10px;color:{};font-variant-numeric:tabular-nums;",
                tok().text_muted
            )>
                {move || {
                    let count = display_entries().len();
                    let total = selected_file().map(|f| f.entries.len()).unwrap_or(0);
                    if count == total {
                        format!("{} entries", count)
                    } else {
                        format!("{} / {} entries", count, total)
                    }
                }}
            </span>
        </div>
    }
}

#[component]
fn LogRows(
    theme: ReadSignal<Theme>,
    display_entries: impl Fn() -> Vec<DisplayEntry> + 'static + Copy + Send,
    focused_line: ReadSignal<Option<usize>>,
    set_focused_line: WriteSignal<Option<usize>>,
    selected_lines: ReadSignal<Vec<usize>>,
    set_selected_lines: WriteSignal<Vec<usize>>,
    set_ctx_x: WriteSignal<i32>,
    set_ctx_y: WriteSignal<i32>,
    set_ctx_entry: WriteSignal<Option<(String, usize, String)>>,
    set_ctx_visible: WriteSignal<bool>,
    selected_file: impl Fn() -> Option<LogFile> + 'static + Copy + Send,
    dark: impl Fn() -> bool + 'static + Copy + Send,
) -> impl IntoView {
    let tok = move || if theme.get() == Theme::Dark { &DARK } else { &LIGHT };
    
    view! {
        <div style="flex:1;overflow-y:auto;overflow-x:auto;font-family:'Fira Code',monospace;">
            {move || {
                let entries = display_entries();
                if entries.is_empty() {
                    return view! {
                        <div style="display:flex;align-items:center;justify-content:center;height:80px">
                            <span style=move || format!("font-size:13px;color:{}", tok().text_muted)>
                                "No entries match the current filters."
                            </span>
                        </div>
                    }.into_any();
                }

                entries
                    .into_iter()
                    .map(|de| {
                        let level = de.entry.status.clone();
                        let (lc, lb, lborder, accent) = level_colors(&level, dark());
                        let show_badge = !level.is_empty() && !de.is_continuation;
                        let is_focused = move || focused_line.get() == Some(de.group_line);
                        let is_selected = move || selected_lines.get().contains(&de.group_line);

                        // Styling
                        let row_bg = if !de.is_continuation && accent != "transparent" {
                            format!("background:linear-gradient(90deg, {}08 0%, transparent 100%);", accent)
                        } else {
                            String::new()
                        };

                        let row_bl = if accent != "transparent" && !de.is_continuation {
                            format!("border-left:3px solid {};", accent)
                        } else if de.is_continuation {
                            format!("border-left:3px solid {}22;", accent)
                        } else {
                            "border-left:3px solid transparent;".to_string()
                        };

                        let msg_opacity = if de.is_continuation { "opacity:0.65;" } else { "" };

                        // Context menu data
                        let file_name = selected_file()
                            .map(|f| {
                                f.name
                                    .rsplit('/')
                                    .next()
                                    .or_else(|| f.name.rsplit('\\').next())
                                    .unwrap_or(&f.name)
                                    .to_string()
                            })
                            .unwrap_or_default();
                        let entry_line = de.group_line;
                        let entry_msg = de.entry.message.clone();

                        view! {
                            <div
                                style=move || {
                                    let base = format!(
                                        "display:flex;align-items:baseline;padding:{}12px;\
                                         border-bottom:1px solid {};{}{}cursor:pointer;\
                                         transition:all 0.15s ease;",
                                        if de.is_continuation { "1px " } else { "2px " },
                                        tok().border_subtle,
                                        row_bl,
                                        row_bg
                                    );
                                    if is_selected() {
                                        format!(
                                            "{}background:{};box-shadow:inset 0 0 0 2px {};",
                                            base,
                                            tok().accent_bg,
                                            tok().accent_border
                                        )
                                    } else if is_focused() {
                                        format!(
                                            "{}background:{};box-shadow:inset 0 0 0 1px {};",
                                            base,
                                            tok().bg_elevated,
                                            tok().border
                                        )
                                    } else {
                                        base
                                    }
                                }
                                class="log-row"
                                on:click=move |ev| {
                                    if ev.ctrl_key() {
                                        set_selected_lines.update(|lines| {
                                            if let Some(pos) = lines.iter().position(|&l| l == entry_line) {
                                                lines.remove(pos);
                                            } else {
                                                lines.push(entry_line);
                                            }
                                        });
                                    } else {
                                        set_focused_line.set(Some(entry_line));
                                        set_selected_lines.set(Vec::new());
                                    }
                                }
                                on:contextmenu=move |ev| {
                                    ev.prevent_default();
                                    ev.stop_propagation();
                                    set_ctx_x.set(ev.client_x());
                                    set_ctx_y.set(ev.client_y());
                                    set_ctx_entry.set(Some((file_name.clone(), entry_line, entry_msg.clone())));
                                    set_ctx_visible.set(true);
                                }
                            >
                                // Line number
                                <span style=move || format!(
                                    "width:44px;text-align:right;font-size:11px;\
                                     color:{};margin-right:12px;flex-shrink:0;\
                                     user-select:none;font-variant-numeric:tabular-nums;",
                                    if de.is_continuation { tok().border } else { tok().text_muted }
                                )>
                                    {if de.is_continuation { String::new() } else { de.group_line.to_string() }}
                                </span>
                                
                                // Timestamp
                                <span style=move || format!(
                                    "width:160px;font-size:11px;color:{};\
                                     margin-right:12px;flex-shrink:0;\
                                     font-variant-numeric:tabular-nums;{}",
                                    tok().text_secondary,
                                    if de.is_continuation { "opacity:0.4;" } else { "" }
                                )>{de.entry.datetime.clone()}</span>
                                
                                // Level badge
                                <span style="width:72px;margin-right:12px;flex-shrink:0;display:flex;align-items:center">
                                    {if show_badge {
                                        view! {
                                            <span style=format!(
                                                "display:inline-block;padding:1px 6px;\
                                                 border-radius:4px;font-size:10px;\
                                                 font-weight:700;letter-spacing:0.05em;\
                                                 white-space:nowrap;line-height:1.4;\
                                                 color:{lc};background:{lb};border:1px solid {lborder};"
                                            )>{level.clone()}</span>
                                        }.into_any()
                                    } else {
                                        view! { <span/> }.into_any()
                                    }}
                                </span>
                                
                                // Message
                                <span style=move || format!(
                                    "font-size:12px;color:{};line-height:1.6;\
                                     word-break:break-all;user-select:text;{}",
                                    tok().text_primary,
                                    msg_opacity
                                )>{de.entry.message}</span>
                            </div>
                        }
                    })
                    .collect_view()
                    .into_any()
            }}
        </div>
    }
}
