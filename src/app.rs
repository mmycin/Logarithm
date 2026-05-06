use crate::components::{FileBar, FileViewer, FilterTab, TitleBar};
use leptos::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
}

#[component]
pub fn App() -> impl IntoView {
    let (theme, set_theme) = signal(Theme::Dark);

    let (open_files, set_open_files) = signal(vec![
        "log1.txt".to_string(),
        "log2.txt".to_string(),
        "data.log".to_string(),
    ]);
    let (active_file, set_active_file) = signal(Some(0usize));

    let is_dark = move || theme.get() == Theme::Dark;

    view! {
        <div class=move || if is_dark() { "flex flex-col h-screen bg-[#1e1e2e] font-sans" } else { "flex flex-col h-screen bg-[#eff1f5] font-sans" }>
            <TitleBar theme set_theme />
            <FilterTab theme />
            <FileBar theme open_files set_open_files active_file set_active_file />
            <FileViewer theme open_files active_file />
        </div>
    }
}
