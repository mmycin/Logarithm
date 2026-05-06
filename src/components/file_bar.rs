use crate::app::Theme;
use leptos::prelude::*;

#[component]
pub fn FileBar(theme: ReadSignal<Theme>) -> impl IntoView {
    let is_dark = move || theme.get() == Theme::Dark;
    let (active_file, set_active_file) = signal(0);
    let files = vec!["log1.txt", "log2.txt", "data.log"];

    view! {
        <div class=move || {
            if is_dark() {
                "bg-[#181825] border-b border-[#313244] flex items-center font-sans px-3 py-0"
            } else {
                "bg-[#e6e9ef] border-b border-[#9ca0b0] flex items-center font-sans px-3 py-0"
            }
        }>
            <div class="flex items-end space-x-0 h-full">
                {files
                    .iter()
                    .enumerate()
                    .map(|(index, file)| {
                        let is_active = move || active_file.get() == index;
                        view! {
                            <button
                                class=move || {
                                    if is_active() {
                                        if is_dark() {
                                            "px-4 py-2 rounded-t-md text-xs font-medium bg-[#1e1e2e] text-[#cdd6f4] border border-[#313244] border-b-0 mb-0 transition-all duration-100"
                                        } else {
                                            "px-4 py-2 rounded-t-md text-xs font-medium bg-[#eff1f5] text-[#4c4f69] border border-[#9ca0b0] border-b-0 mb-0 transition-all duration-100"
                                        }
                                    } else {
                                        if is_dark() {
                                            "px-4 py-2 rounded-t-md text-xs font-medium text-[#a6adc8] hover:text-[#cdd6f4] hover:bg-[#313244]/20 mb-0 transition-all duration-100"
                                        } else {
                                            "px-4 py-2 rounded-t-md text-xs font-medium text-[#6c6f85] hover:text-[#4c4f69] hover:bg-[#ccd0da]/20 mb-0 transition-all duration-100"
                                        }
                                    }
                                }
                                on:click=move |_| set_active_file.set(index)
                            >
                                <span class="flex items-center space-x-1.5">
                                    <span class="text-sm">{ "📄" }</span>
                                    <span>{file.to_string()}</span>
                                </span>
                            </button>
                        }
                    })
                    .collect_view()}
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
