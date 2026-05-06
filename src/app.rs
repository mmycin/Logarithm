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

    let is_dark = move || theme.get() == Theme::Dark;

    view! {
        <div class=move || if is_dark() { "flex flex-col h-screen bg-[#1e1e2e] font-sans" } else { "flex flex-col h-screen bg-[#eff1f5] font-sans" }>
            <TitleBar theme set_theme />
            <FilterTab theme />
            <FileBar theme />
            <FileViewer theme />
        </div>
    }
}
