use leptos::*;
use leptos::html::ElementChild;

#[component]
pub fn TitleBar() -> impl IntoView {
    view! {
        <nav>
            <h1>"Title bar"</h1>
        </nav>
    }
}