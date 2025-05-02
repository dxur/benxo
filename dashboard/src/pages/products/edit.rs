use leptos::prelude::*;

use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const ProductEdit : Page = Page {
    title: "Product Edit",
    view: View,
};

#[component]
fn View() -> AnyView {
    view! {}.into_any()
}