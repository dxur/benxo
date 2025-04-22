use leptos::prelude::*;

#[component]
pub fn Home() -> AnyView {
    view! {
        <div class="flex flex-col">
            <span> Home </span>
            <hr/>
            <span> Here you should see some analytics and varios stuff about your store </span>
        </div>
    }
    .into_any()
}
