use leptos::prelude::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div data-404>
            <h1>404<br></br>Not Found</h1>
        </div>
    }
}
