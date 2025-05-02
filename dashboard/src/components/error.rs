use leptos::prelude::*;

#[component]
pub fn Error() -> impl IntoView {
    view! {
        <div data-status>
            <span> Error </span>
        </div>
    }
}
