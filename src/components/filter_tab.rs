use crate::app::Theme;
use leptos::prelude::*;

#[component]
pub fn FilterStatusGroup(theme: ReadSignal<Theme>) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;
    let (selected_status, set_selected_status) = signal("all");

    view! {
        <div class=move || {
            if is_dark() {
                "w-48 bg-[#1e1e2e] border border-[#313244] rounded-lg p-3 flex flex-col flex-shrink-0"
            } else {
                "w-48 bg-[#eff1f5] border border-[#9ca0b0] rounded-lg p-3 flex flex-col flex-shrink-0"
            }
        }>
            <h3 class=move || {
                if is_dark() {
                    "text-xs font-semibold text-[#cdd6f4] mb-2"
                } else {
                    "text-xs font-semibold text-[#4c4f69] mb-2"
                }
            }>
                "Status"
            </h3>
            <div class="grid grid-cols-2 gap-1.5">
                <div
                    class=move || {
                        let is_selected = selected_status.get() == "all";
                        if is_selected {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#89b4fa]/20 border border-[#89b4fa]/50 cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#89b4fa]/20 border border-[#89b4fa]/50 cursor-pointer transition-all duration-150"
                            }
                        } else {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#313244] cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#ccd0da] cursor-pointer transition-all duration-150"
                            }
                        }
                    }
                    on:click=move |_| set_selected_status.set("all")
                >
                    <input
                        type="radio"
                        name="status"
                        class="w-3.5 h-3.5 cursor-pointer text-[#89b4fa]"
                        prop:checked=move || selected_status.get() == "all"
                    />
                    <span class=move || {
                        if is_dark() {
                            "text-xs text-[#cdd6f4]"
                        } else {
                            "text-xs text-[#4c4f69]"
                        }
                    }>
                        "All"
                    </span>
                </div>
                <div
                    class=move || {
                        let is_selected = selected_status.get() == "success";
                        if is_selected {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#a6e3a1]/20 border border-[#a6e3a1]/50 cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#a6e3a1]/20 border border-[#a6e3a1]/50 cursor-pointer transition-all duration-150"
                            }
                        } else {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#313244] cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#ccd0da] cursor-pointer transition-all duration-150"
                            }
                        }
                    }
                    on:click=move |_| set_selected_status.set("success")
                >
                    <input
                        type="radio"
                        name="status"
                        class="w-3.5 h-3.5 cursor-pointer text-[#a6e3a1]"
                        prop:checked=move || selected_status.get() == "success"
                    />
                    <span class=move || {
                        if is_dark() {
                            "text-xs text-[#a6e3a1]"
                        } else {
                            "text-xs text-[#40a02b]"
                        }
                    }>
                        "Success"
                    </span>
                </div>
                <div
                    class=move || {
                        let is_selected = selected_status.get() == "info";
                        if is_selected {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#f9e2af]/20 border border-[#f9e2af]/50 cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#f9e2af]/20 border border-[#f9e2af]/50 cursor-pointer transition-all duration-150"
                            }
                        } else {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#313244] cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#ccd0da] cursor-pointer transition-all duration-150"
                            }
                        }
                    }
                    on:click=move |_| set_selected_status.set("info")
                >
                    <input
                        type="radio"
                        name="status"
                        class="w-3.5 h-3.5 cursor-pointer text-[#f9e2af]"
                        prop:checked=move || selected_status.get() == "info"
                    />
                    <span class=move || {
                        if is_dark() {
                            "text-xs text-[#f9e2af]"
                        } else {
                            "text-xs text-[#df8e1d]"
                        }
                    }>
                        "Info"
                    </span>
                </div>
                <div
                    class=move || {
                        let is_selected = selected_status.get() == "error";
                        if is_selected {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#f38ba8]/20 border border-[#f38ba8]/50 cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md bg-[#f38ba8]/20 border border-[#f38ba8]/50 cursor-pointer transition-all duration-150"
                            }
                        } else {
                            if is_dark() {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#313244] cursor-pointer transition-all duration-150"
                            } else {
                                "flex items-center space-x-1.5 px-2 py-1.5 rounded-md hover:bg-[#ccd0da] cursor-pointer transition-all duration-150"
                            }
                        }
                    }
                    on:click=move |_| set_selected_status.set("error")
                >
                    <input
                        type="radio"
                        name="status"
                        class="w-3.5 h-3.5 cursor-pointer text-[#f38ba8]"
                        prop:checked=move || selected_status.get() == "error"
                    />
                    <span class=move || {
                        if is_dark() {
                            "text-xs text-[#f38ba8]"
                        } else {
                            "text-xs text-[#d20f39]"
                        }
                    }>
                        "Error"
                    </span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn FilterSearchGroup(theme: ReadSignal<Theme>) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;
    let (search_query, set_search_query) = signal(String::new());
    let (match_case, set_match_case) = signal(false);
    let (fuzzy_find, set_fuzzy_find) = signal(false);

    view! {
        <div class=move || {
            if is_dark() {
                "flex-1 bg-[#1e1e2e] border border-[#313244] rounded-lg p-3 flex flex-col"
            } else {
                "flex-1 bg-[#eff1f5] border border-[#9ca0b0] rounded-lg p-3 flex flex-col"
            }
        }>
            <h3 class=move || {
                if is_dark() {
                    "text-xs font-semibold text-[#cdd6f4] mb-2"
                } else {
                    "text-xs font-semibold text-[#4c4f69] mb-2"
                }
            }>
                "Search"
            </h3>
            <input
                type="text"
                placeholder="Search..."
                class=move || {
                    if is_dark() {
                        "w-full px-3 py-1.5 bg-[#181825] border border-[#313244] rounded-md text-xs text-[#cdd6f4] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                    } else {
                        "w-full px-3 py-1.5 bg-[#e6e9ef] border border-[#9ca0b0] rounded-md text-xs text-[#4c4f69] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                    }
                }
                prop:value=move || search_query.get()
                on:input=move |ev| {
                    set_search_query.set(event_target_value(&ev));
                }
            />
            <div class="mt-2 flex flex-col space-y-1">
                <div
                    class=move || {
                        if is_dark() {
                            "flex items-center space-x-1.5 px-2 py-1 rounded-md hover:bg-[#313244] cursor-pointer transition-all duration-150"
                        } else {
                            "flex items-center space-x-1.5 px-2 py-1 rounded-md hover:bg-[#ccd0da] cursor-pointer transition-all duration-150"
                        }
                    }
                    on:click=move |_| set_match_case.update(|v| *v = !*v)
                >
                    <input
                        type="checkbox"
                        class="w-3.5 h-3.5 rounded cursor-pointer"
                        prop:checked=move || match_case.get()
                    />
                    <span class=move || {
                        if is_dark() {
                            "text-xs text-[#cdd6f4]"
                        } else {
                            "text-xs text-[#4c4f69]"
                        }
                    }>
                        "Match Case"
                    </span>
                </div>
                <div
                    class=move || {
                        if is_dark() {
                            "flex items-center space-x-1.5 px-2 py-1 rounded-md hover:bg-[#313244] cursor-pointer transition-all duration-150"
                        } else {
                            "flex items-center space-x-1.5 px-2 py-1 rounded-md hover:bg-[#ccd0da] cursor-pointer transition-all duration-150"
                        }
                    }
                    on:click=move |_| set_fuzzy_find.update(|v| *v = !*v)
                >
                    <input
                        type="checkbox"
                        class="w-3.5 h-3.5 rounded cursor-pointer"
                        prop:checked=move || fuzzy_find.get()
                    />
                    <span class=move || {
                        if is_dark() {
                            "text-xs text-[#cdd6f4]"
                        } else {
                            "text-xs text-[#4c4f69]"
                        }
                    }>
                        "Fuzzy Find"
                    </span>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn FilterDateTimeGroup(theme: ReadSignal<Theme>) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;
    let (from_date, set_from_date) = signal(String::new());
    let (to_date, set_to_date) = signal(String::new());
    let (from_time, set_from_time) = signal(String::new());
    let (to_time, set_to_time) = signal(String::new());

    view! {
        <div class=move || {
            if is_dark() {
                "flex-1 bg-[#1e1e2e] border border-[#313244] rounded-lg p-3 flex flex-col"
            } else {
                "flex-1 bg-[#eff1f5] border border-[#9ca0b0] rounded-lg p-3 flex flex-col"
            }
        }>
            <h3 class=move || {
                if is_dark() {
                    "text-xs font-semibold text-[#cdd6f4] mb-2"
                } else {
                    "text-xs font-semibold text-[#4c4f69] mb-2"
                }
            }>
                "Date & Time"
            </h3>
            <div class="space-y-2">
                <div class="space-y-1">
                    <label class=move || {
                        if is_dark() {
                            "text-xs font-medium text-[#a6adc8]"
                        } else {
                            "text-xs font-medium text-[#6c6f85]"
                        }
                    }>
                        "From"
                    </label>
                    <div class="flex flex-row gap-1.5">
                        <input
                            type="date"
                            class=move || {
                                if is_dark() {
                                    "flex-1 px-2 py-1.5 bg-[#181825] border border-[#313244] rounded-md text-xs text-[#cdd6f4] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                } else {
                                    "flex-1 px-2 py-1.5 bg-[#e6e9ef] border border-[#9ca0b0] rounded-md text-xs text-[#4c4f69] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                }
                            }
                            prop:value=move || from_date.get()
                            on:input=move |ev| {
                                set_from_date.set(event_target_value(&ev));
                            }
                        />
                        <input
                            type="time"
                            class=move || {
                                if is_dark() {
                                    "flex-1 px-2 py-1.5 bg-[#181825] border border-[#313244] rounded-md text-xs text-[#cdd6f4] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                } else {
                                    "flex-1 px-2 py-1.5 bg-[#e6e9ef] border border-[#9ca0b0] rounded-md text-xs text-[#4c4f69] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                }
                            }
                            prop:value=move || from_time.get()
                            on:input=move |ev| {
                                set_from_time.set(event_target_value(&ev));
                            }
                        />
                    </div>
                </div>
                <div class="space-y-1">
                    <label class=move || {
                        if is_dark() {
                            "text-xs font-medium text-[#a6adc8]"
                        } else {
                            "text-xs font-medium text-[#6c6f85]"
                        }
                    }>
                        "To"
                    </label>
                    <div class="flex flex-row gap-1.5">
                        <input
                            type="date"
                            class=move || {
                                if is_dark() {
                                    "flex-1 px-2 py-1.5 bg-[#181825] border border-[#313244] rounded-md text-xs text-[#cdd6f4] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                } else {
                                    "flex-1 px-2 py-1.5 bg-[#e6e9ef] border border-[#9ca0b0] rounded-md text-xs text-[#4c4f69] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                }
                            }
                            prop:value=move || to_date.get()
                            on:input=move |ev| {
                                set_to_date.set(event_target_value(&ev));
                            }
                        />
                        <input
                            type="time"
                            class=move || {
                                if is_dark() {
                                    "flex-1 px-2 py-1.5 bg-[#181825] border border-[#313244] rounded-md text-xs text-[#cdd6f4] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                } else {
                                    "flex-1 px-2 py-1.5 bg-[#e6e9ef] border border-[#9ca0b0] rounded-md text-xs text-[#4c4f69] outline-none focus:border-[#89b4fa] focus:ring-1 focus:ring-[#89b4fa]/20 transition-all duration-150"
                                }
                            }
                            prop:value=move || to_time.get()
                            on:input=move |ev| {
                                set_to_time.set(event_target_value(&ev));
                            }
                        />
                    </div>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn FilterTab(theme: ReadSignal<Theme>) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;
    let (collapsed, set_collapsed) = signal(false);

    view! {
        <div class=move || {
            if is_dark() {
                "bg-[#181825] border-b border-[#313244] relative"
            } else {
                "bg-[#e6e9ef] border-b border-[#9ca0b0] relative"
            }
        }>
            <div class=move || {
                if is_dark() {
                    "px-4 py-2 border-b border-[#313244] bg-[#1e1e2e] flex items-center justify-between"
                } else {
                    "px-4 py-2 border-b border-[#9ca0b0] bg-[#eff1f5] flex items-center justify-between"
                }
            }>
                <h2 class=move || {
                    if is_dark() {
                        "text-sm font-semibold text-[#cdd6f4] font-sans"
                    } else {
                        "text-sm font-semibold text-[#4c4f69] font-sans"
                    }
                }>
                    "Filters"
                </h2>
                <button
                    class=move || {
                        if is_dark() {
                            "p-1.5 rounded-md hover:bg-[#313244] transition-all duration-150"
                        } else {
                            "p-1.5 rounded-md hover:bg-[#ccd0da] transition-all duration-150"
                        }
                    }
                    on:click=move |_| {
                        set_collapsed.update(|c| *c = !*c);
                    }
                >
                    <span class=move || {
                        if is_dark() {
                            "text-base text-[#cdd6f4]"
                        } else {
                            "text-base text-[#4c4f69]"
                        }
                    }>
                        {move || if collapsed.get() { "🔼" } else { "🔽" }}
                    </span>
                </button>
            </div>

            <Show when=move || !collapsed.get()>
                <div class="p-3 font-sans">
                    <div class="flex flex-row gap-3">
                        <FilterStatusGroup theme=theme />
                        <FilterSearchGroup theme=theme />
                        <FilterDateTimeGroup theme=theme />
                    </div>
                </div>
            </Show>
        </div>
    }
}
