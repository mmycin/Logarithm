use crate::app::Theme;
use leptos::prelude::*;

#[component]
pub fn FileBar(
    theme: ReadSignal<Theme>,
    open_files: ReadSignal<Vec<String>>,
    set_open_files: WriteSignal<Vec<String>>,
    active_file: ReadSignal<Option<usize>>,
    set_active_file: WriteSignal<Option<usize>>,
) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;

    view! {
        <div class=move || {
            if is_dark() {
                "bg-[#181825] border-b border-[#313244] flex items-center font-sans px-3 py-0"
            } else {
                "bg-[#e6e9ef] border-b border-[#9ca0b0] flex items-center font-sans px-3 py-0"
            }
        }>
            <div class="flex items-end space-x-0 h-full">
                {move || {
                    let files = open_files.get();
                    files
                        .into_iter()
                        .enumerate()
                        .map(|(index, file)| {
                            let file_for_close = file.clone();
                            let is_active = move || active_file.get() == Some(index);

                            view! {
                                <div class="flex items-end">
                                    <div
                                        class=move || {
                                            if is_active() {
                                                if is_dark() {
                                                    "px-4 py-2 rounded-t-md text-xs font-medium bg-[#1e1e2e] text-[#cdd6f4] border border-[#313244] border-b-0 mb-0 transition-all duration-100 cursor-pointer"
                                                } else {
                                                    "px-4 py-2 rounded-t-md text-xs font-medium bg-[#eff1f5] text-[#4c4f69] border border-[#9ca0b0] border-b-0 mb-0 transition-all duration-100 cursor-pointer"
                                                }
                                            } else {
                                                if is_dark() {
                                                    "px-4 py-2 rounded-t-md text-xs font-medium text-[#a6adc8] hover:text-[#cdd6f4] hover:bg-[#313244]/20 mb-0 transition-all duration-100 cursor-pointer"
                                                } else {
                                                    "px-4 py-2 rounded-t-md text-xs font-medium text-[#6c6f85] hover:text-[#4c4f69] hover:bg-[#ccd0da]/20 mb-0 transition-all duration-100 cursor-pointer"
                                                }
                                            }
                                        }
                                        on:click=move |_| set_active_file.set(Some(index))
                                    >
                                        <span class="flex items-center space-x-2">
                                            <span class="text-sm">{ "📄" }</span>
                                            <span>{file.clone()}</span>
                                            <button
                                                class=move || {
                                                    if is_dark() {
                                                        "ml-1 p-0.5 rounded hover:bg-[#313244] text-[#a6adc8] hover:text-[#cdd6f4]"
                                                    } else {
                                                        "ml-1 p-0.5 rounded hover:bg-[#ccd0da] text-[#6c6f85] hover:text-[#4c4f69]"
                                                    }
                                                }
                                                on:click=move |ev| {
                                                    ev.stop_propagation();
                                                    let current_active = active_file.get();

                                                    set_open_files.update(|files| {
                                                        let removed_index =
                                                            files.iter().position(|f| f == &file_for_close);
                                                        if let Some(pos) = removed_index {
                                                            files.remove(pos);

                                                            let new_len = files.len();
                                                            match current_active {
                                                                None => {}
                                                                Some(ai) if ai == pos => {
                                                                    if new_len == 0 {
                                                                        set_active_file.set(None);
                                                                    } else if pos >= new_len {
                                                                        set_active_file.set(Some(new_len - 1));
                                                                    } else {
                                                                        set_active_file.set(Some(pos));
                                                                    }
                                                                }
                                                                Some(ai) if ai > pos => {
                                                                    set_active_file.set(Some(ai - 1));
                                                                }
                                                                _ => {}
                                                            }
                                                        }
                                                    });
                                                }
                                                type="button"
                                            >
                                                "×"
                                            </button>
                                        </span>
                                    </div>
                                </div>
                            }
                        })
                        .collect_view()
                }}
            </div>
            <div class="flex-1"></div>
            <button
                class=move || {
                    if is_dark() {
                        "p-1.5 rounded-md hover:bg-[#313244] transition-all duration-100"
                    } else {
                        "p-1.5 rounded-md hover:bg-[#ccd0da] transition-all duration-100"
                    }
                }
            >
                <span class="text-sm">{ "➕" }</span>
            </button>
        </div>
    }
}
