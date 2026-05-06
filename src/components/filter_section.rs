use leptos::prelude::*;

/// A collapsible section used inside the filter panel.
/// `badge` is a reactive `Signal<Option<String>>` — pass `Signal::derive(|| None)` for no badge.
#[component]
pub fn FilterSection(
    #[prop(into)] title: &'static str,
    badge: Signal<Option<String>>,
    #[prop(into)] muted: &'static str,
    #[prop(into)] border: &'static str,
    #[prop(into)] accent: &'static str,
    #[prop(into)] accent_bg: &'static str,
    children: ChildrenFn,
) -> impl IntoView {
    let (open, set_open) = signal(true);

    view! {
        <div>
            <button
                style=format!(
                    "display:flex;align-items:center;justify-content:space-between;\
                     width:100%;padding:9px 14px 7px;background:transparent;border:none;\
                     cursor:pointer;text-align:left;gap:6px;user-select:none;"
                )
                on:click=move |_| set_open.update(|v| *v = !*v)
            >
                <div style="display:flex;align-items:center;gap:6px;flex:1;min-width:0">
                    <span style=format!(
                        "font-size:10px;font-weight:700;color:{muted};letter-spacing:0.09em;\
                         text-transform:uppercase;white-space:nowrap;"
                    )>{title}</span>
                    {move || badge.get().map(|text| view! {
                        <span style=format!(
                            "font-size:9.5px;font-weight:700;color:{accent};background:{accent_bg};\
                             border-radius:8px;padding:1px 6px;flex-shrink:0;"
                        )>{text}</span>
                    })}
                </div>
                <svg width="9" height="9" viewBox="0 0 16 16" fill="currentColor"
                    style=move || format!(
                        "color:{muted};transition:transform 0.18s;transform:rotate({}deg);flex-shrink:0;",
                        if open.get() { 0 } else { -90 }
                    )>
                    <path d="M7.247 11.14 2.451 5.658C1.885 5.013 2.345 4 3.204 4h9.592a1 1 0 0 1 .753 1.659l-4.796 5.48a1 1 0 0 1-1.506 0z"/>
                </svg>
            </button>
            <Show when=move || open.get()>
                {children()}
            </Show>
            <div style=format!("height:1px;background:{border};margin:2px 0;")/>
        </div>
    }
}
