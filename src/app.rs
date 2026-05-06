use crate::components::{FileBar, FileViewer, FilterTab, TitleBar};
use leptos::prelude::*;


#[component]
pub fn App() -> impl IntoView {
    view! {
        <main class="flex items-center justify-center h-screen">
            <TitleBar />

            <FilterTab />

            <FileBar />

            <FileViewer />
        </main>
    }
}
