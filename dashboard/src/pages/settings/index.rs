use leptos::prelude::*;

use super::state::IndexState as State;
use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const SettingsIndex: Page = Page {
    title: "Settings",
    view: SettingsView,
};

#[component]
pub fn SettingsView() -> AnyView {
    let state = State::new();
    view! {
        <LazyShow
            when=move || state.status.get()
        >
            <form on:submit=move |ev| {
                ev.prevent_default();
                // state.update();
            }>
                <Editor header=move || view! {
                    <header>
                        <h2> Settings </h2>
                        <Row>
                            <button type="reset"
                                // on:click=move |_| state.delete()
                            > Delete </button>
                            <button type="submit"> Save </button>
                        </Row>
                    </header>
                }>
                    <Content>
                        <Body state=state />
                    </Content>
                    <Panel>
                        <Inspector state=state />
                    </Panel>
                </Editor>
            </form>
        </LazyShow>
    }
    .into_any()
}

#[component]
fn Body(state: State) -> impl IntoView {
    view! {}
}

#[component]
fn Inspector(state: State) -> impl IntoView {
    view! {}
}
