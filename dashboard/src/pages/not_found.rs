use leptos::prelude::*;

use super::Page;

#[allow(non_upper_case_globals)]
pub const NotFound: Page = Page {
    title: "Not Found",
    view: NotFoundView,
};

#[component]
fn NotFoundView() -> AnyView {
    view! {
        <div data-404>
            <h1>404<br></br>Not Found</h1>
        </div>
    }.into_any()
}
