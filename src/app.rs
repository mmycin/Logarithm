use crate::components::{FileBar, FileViewer, FilterTab, TitleBar};
use leptos::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

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
