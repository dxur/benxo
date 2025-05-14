use leptos::prelude::*;

use super::state::IndexState as State;
use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const StoreIndex: Page = Page {
    title: "Online Store",
    view: View,
};

#[component]
fn View() -> AnyView {
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
                            <div>
                                <h2> Online Store </h2>
                            </div>
                            <Row>
                                <button type="submit"> Save </button>
                            </Row>
                    </header>
                }>
                    <Panel>
                        <Inspector state=state />
                    </Panel>
                    <Content>
                        <Body state=state />
                    </Content>
                </Editor>
            </form>
        </LazyShow>
    }
    .into_any()
}

#[component]
fn Body(state: State) -> impl IntoView {
    view! {
        <Card>
            Hello world
        </Card>
    }
}

#[component]
fn Inspector(state: State) -> impl IntoView {
    view! {
        <Card>
            Hello world
        </Card>
    }
}
