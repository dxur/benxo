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
                        <div>
                            <h2> Settings </h2>
                            <h3> store123 </h3>
                        </div>
                        <button type="submit"> Save </button>
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
    let edit_basic = RwSignal::new(false);
    view! {
        <Card>
            <Row>
                <h3> Basic </h3>
                <button type="button"
                    on:click=move |_| edit_basic.update(|v| *v = !*v)
                > { move || if edit_basic.get() { "Done" } else { "Edit" } } </button>
            </Row>
            <fieldset>
                <label> Store Name
                    <input readonly=move || !edit_basic.get() type="text" required />
                </label>
            </fieldset>
            <fieldset>
                <label> Phone
                    <input readonly=move || !edit_basic.get() type="tel" required />
                </label>
            </fieldset>
            <fieldset>
                <label> Email
                    <input readonly=move || !edit_basic.get() type="text" required />
                </label>
            </fieldset>
            <fieldset>
                <label> Domain name
                    <input
                        readonly=move || !edit_basic.get()
                        type="text"
                        pattern=r"^[\p{L}\p{N}]+(?:-[\p{L}\p{N}]+)*$"
                        required
                    />
                </label>
            </fieldset>
        </Card>
    }
}

#[component]
fn Inspector(state: State) -> impl IntoView {
    view! {
        <Card>
            <h3> Plan </h3>
            Free Plan
        </Card>
    }
}
