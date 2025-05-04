use leptos::prelude::*;

#[component]
pub fn Row(children: Children) -> impl IntoView {
    view! {
        <div data-row>
            {children()}
        </div>
    }
}

#[component]
pub fn Card(children: Children) -> impl IntoView {
    view! {
        <div data-card>
            {children()}
        </div>
    }
}

#[component]
pub fn Content(children: Children) -> impl IntoView {
    view! {
        <div data-role="content">
            {children()}
        </div>
    }
}

#[component]
pub fn Panel(children: Children) -> impl IntoView {
    view! {
        <div data-role="panel">
            {children()}
        </div>
    }
}

#[component]
pub fn Editor(header: impl IntoView, children: Children) -> impl IntoView {
    view! {
        <div data-editor>
            {header}
            <div data-content>
                {children()}
            </div>
        </div>
    }
}

#[component]
pub fn Divider(children: Children) -> impl IntoView {
    view! {
        <div data-divider>
            {children()}
        </div>
    }
}

#[component]
pub fn Badges(children: Children) -> impl IntoView {
    view! {
        <ul data-badges>
            {children()}
        </ul>
    }
}

#[component]
pub fn Badge(children: Children) -> impl IntoView {
    view! {
        <li data-badge>
            {children()}
        </li>
    }
}

#[component]
pub fn Timeline(children: Children) -> impl IntoView {
    view! {
        <ul data-timeline>
            {children()}
        </ul>
    }
}