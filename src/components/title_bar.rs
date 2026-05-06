use crate::app::Theme;
use leptos::ev::MouseEvent;
use leptos::prelude::*;

#[component]
pub fn TitleBar(theme: ReadSignal<Theme>, set_theme: WriteSignal<Theme>) -> impl IntoView {
    let (active_menu, set_active_menu) = signal(None::<&'static str>);

    let toggle_menu = move |menu: &'static str| {
        set_active_menu.update(|current| {
            *current = if *current == Some(menu) {
                None
            } else {
                Some(menu)
            };
        });
    };

    let close_menu = move |_: MouseEvent| {
        set_active_menu.set(None);
    };

    let is_dark = move || theme.get() == Theme::Dark;

    view! {
        <nav class=move || {
            if is_dark() {
                "bg-[#181825] border-b border-[#313244] w-full px-6 py-3 flex items-center space-x-1 font-sans shadow-sm"
            } else {
                "bg-[#e6e9ef] border-b border-[#9ca0b0] w-full px-6 py-3 flex items-center space-x-1 font-sans shadow-sm"
            }
        }>
            <div class="flex items-center space-x-2 mr-8">
                <div class=move || {
                    if is_dark() {
                        "w-8 h-8 rounded-lg bg-gradient-to-br from-[#89b4fa] to-[#cba6f7] flex items-center justify-center"
                    } else {
                        "w-8 h-8 rounded-lg bg-gradient-to-br from-[#89b4fa] to-[#cba6f7] flex items-center justify-center"
                    }
                }>
                    <span class="text-white font-bold text-sm">L</span>
                </div>
                <span class=move || {
                    if is_dark() {
                        "text-[#cdd6f4] font-semibold text-sm tracking-wide"
                    } else {
                        "text-[#4c4f69] font-semibold text-sm tracking-wide"
                    }
                }>
                    "Logarithm"
                </span>
            </div>

            <div class="relative">
                <button
                    class=move || {
                        if is_dark() {
                            "px-4 py-2 text-[#cdd6f4] text-sm font-medium rounded-lg hover:bg-[#313244] transition-all duration-200"
                        } else {
                            "px-4 py-2 text-[#4c4f69] text-sm font-medium rounded-lg hover:bg-[#ccd0da] transition-all duration-200"
                        }
                    }
                    on:click=move |_| toggle_menu("file")
                >
                    "File"
                </button>
                <Show when=move || active_menu.get() == Some("file")>
                    <div
                        class=move || {
                            if is_dark() {
                                "absolute top-full left-0 mt-2 bg-[#1e1e2e] border border-[#313244] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            } else {
                                "absolute top-full left-0 mt-2 bg-[#eff1f5] border border-[#9ca0b0] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            }
                        }
                        on:mouseleave=close_menu
                    >
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Open"
                        </button>
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Open Recent"
                        </button>
                        <hr class=move || {
                            if is_dark() {
                                "border-t border-[#313244] my-1"
                            } else {
                                "border-t border-[#9ca0b0] my-1"
                            }
                        } />
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Exit"
                        </button>
                    </div>
                </Show>
            </div>

            <div class="relative">
                <button
                    class=move || {
                        if is_dark() {
                            "px-4 py-2 text-[#cdd6f4] text-sm font-medium rounded-lg hover:bg-[#313244] transition-all duration-200"
                        } else {
                            "px-4 py-2 text-[#4c4f69] text-sm font-medium rounded-lg hover:bg-[#ccd0da] transition-all duration-200"
                        }
                    }
                    on:click=move |_| toggle_menu("edit")
                >
                    "Edit"
                </button>
                <Show when=move || active_menu.get() == Some("edit")>
                    <div
                        class=move || {
                            if is_dark() {
                                "absolute top-full left-0 mt-2 bg-[#1e1e2e] border border-[#313244] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            } else {
                                "absolute top-full left-0 mt-2 bg-[#eff1f5] border border-[#9ca0b0] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            }
                        }
                        on:mouseleave=close_menu
                    >
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Find"
                        </button>
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Find in Files"
                        </button>
                        <hr class=move || {
                            if is_dark() {
                                "border-t border-[#313244] my-1"
                            } else {
                                "border-t border-[#9ca0b0] my-1"
                            }
                        } />
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Cut"
                        </button>
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Copy"
                        </button>
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Paste"
                        </button>
                    </div>
                </Show>
            </div>

            <div class="relative">
                <button
                    class=move || {
                        if is_dark() {
                            "px-4 py-2 text-[#cdd6f4] text-sm font-medium rounded-lg hover:bg-[#313244] transition-all duration-200"
                        } else {
                            "px-4 py-2 text-[#4c4f69] text-sm font-medium rounded-lg hover:bg-[#ccd0da] transition-all duration-200"
                        }
                    }
                    on:click=move |_| toggle_menu("view")
                >
                    "View"
                </button>
                <Show when=move || active_menu.get() == Some("view")>
                    <div
                        class=move || {
                            if is_dark() {
                                "absolute top-full left-0 mt-2 bg-[#1e1e2e] border border-[#313244] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            } else {
                                "absolute top-full left-0 mt-2 bg-[#eff1f5] border border-[#9ca0b0] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            }
                        }
                        on:mouseleave=close_menu
                    >
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Toggle Left Bar"
                        </button>
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Toggle AI Chat"
                        </button>
                        <button class=move || {
                            if is_dark() {
                                "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150"
                            } else {
                                "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150"
                            }
                        }>
                            "Toggle Filter Menu"
                        </button>
                    </div>
                </Show>
            </div>

            <div class="flex-1"></div>

            <div class="relative">
                <button
                    class=move || {
                        if is_dark() {
                            "px-4 py-2 text-[#cdd6f4] text-sm font-medium rounded-lg hover:bg-[#313244] transition-all duration-200"
                        } else {
                            "px-4 py-2 text-[#4c4f69] text-sm font-medium rounded-lg hover:bg-[#ccd0da] transition-all duration-200"
                        }
                    }
                    on:click=move |_| toggle_menu("about")
                >
                    "About"
                </button>
                <Show when=move || active_menu.get() == Some("about")>
                    <div
                        class=move || {
                            if is_dark() {
                                "absolute top-full left-0 mt-2 bg-[#1e1e2e] border border-[#313244] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            } else {
                                "absolute top-full left-0 mt-2 bg-[#eff1f5] border border-[#9ca0b0] rounded-xl shadow-xl z-50 min-w-[200px] py-2"
                            }
                        }
                        on:mouseleave=close_menu
                    >
                    </div>
                </Show>
            </div>

            <div class="relative">
                <button
                    class=move || {
                        if is_dark() {
                            "px-3 py-2 text-[#cdd6f4] rounded-lg hover:bg-[#313244] transition-all duration-200 flex items-center space-x-2"
                        } else {
                            "px-3 py-2 text-[#4c4f69] rounded-lg hover:bg-[#ccd0da] transition-all duration-200 flex items-center space-x-2"
                        }
                    }
                    on:click=move |_| toggle_menu("settings")
                >
                    <span class="text-lg">{ "⚙️" }</span>
                </button>
                <Show when=move || active_menu.get() == Some("settings")>
                    <div
                        class=move || {
                            if is_dark() {
                                "absolute top-full right-0 mt-2 bg-[#1e1e2e] border border-[#313244] rounded-xl shadow-xl z-50 min-w-[240px] py-2"
                            } else {
                                "absolute top-full right-0 mt-2 bg-[#eff1f5] border border-[#9ca0b0] rounded-xl shadow-xl z-50 min-w-[240px] py-2"
                            }
                        }
                        on:mouseleave=close_menu
                    >
                        <button
                            class=move || {
                                if is_dark() {
                                    "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150 flex items-center space-x-2"
                                } else {
                                    "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150 flex items-center space-x-2"
                                }
                            }
                            on:click=move |_| {
                                set_theme.set(Theme::Dark);
                            }
                        >
                            <span class="text-base">{ "🌙" }</span>
                            { "Dark Mode" }
                        </button>
                        <button
                            class=move || {
                                if is_dark() {
                                    "w-full text-left px-4 py-2 text-sm text-[#cdd6f4] hover:bg-[#89b4fa] hover:text-[#1e1e2e] transition-all duration-150 flex items-center space-x-2"
                                } else {
                                    "w-full text-left px-4 py-2 text-sm text-[#4c4f69] hover:bg-[#89b4fa] hover:text-[#eff1f5] transition-all duration-150 flex items-center space-x-2"
                                }
                            }
                            on:click=move |_| {
                                set_theme.set(Theme::Light);
                            }
                        >
                            <span class="text-base">{ "☀️" }</span>
                            { "Light Mode" }
                        </button>
                    </div>
                </Show>
            </div>
        </nav>
    }
}
