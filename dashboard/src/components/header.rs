use leptos::prelude::*;
/// Displays a `render_prop` and some children within markup.
#[component]
pub fn Header(title: &'static str, children: Children) -> impl IntoView {
    view! {
        <header>
            <title>{ title }</title>
            <h1>{ title }</h1>
            {children()}
        </header>
    }
}