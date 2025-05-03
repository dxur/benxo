use leptos::prelude::*;
use slotmap::DefaultKey;

use crate::components::*;
use crate::pages::Page;
use super::state::{EditState as State, OptionEntry};

#[allow(non_upper_case_globals)]
pub const ProductEdit : Page = Page {
    title: "Product Edit",
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
                state.update_product();
            }>
                <Editor header=move || view! {
                    <header> {
                        let prod = state.product.get().unwrap();
                        view! {
                            <div>
                                <h2> { prod.name } </h2>
                                <h3> { prod.id.to_string() } </h3>
                            </div>
                            <Row>                                
                                <button type="reset"
                                    on:click=move |_| state.delete()
                                > Delete </button>
                                <button type="submit"> Save </button>
                            </Row>
                        }
                    } </header>
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
    }.into_any()
}

#[component]
fn Body(state: State) -> impl IntoView {
    view! {
        <Card>
            <fieldset>
                <label> Name <input type="text" bind:value=state.fields.name /></label>
            </fieldset>
            <fieldset>
                <label> Description <textarea rows="10" bind:value=state.fields.description /></label>
            </fieldset>
        </Card>

        <Card>
            <fieldset>
                <label> price <input type="number" bind:value=state.fields.base_price step=".01" /></label>
            </fieldset>
            <fieldset>
                <label> discount <input type="number" bind:value=state.fields.base_discount step=".01" /></label>
            </fieldset>
        </Card>

        <Card>
            <header><h3>Options</h3></header>
            <Show
                when=move || !state.fields.options.get().is_empty()
            >
                <Divider>
                    <For
                        each=move || state.fields.options.get()
                        key=|(key, _)| *key
                        let(opt)
                    >
                        <OptionBlock state=state opt=opt />
                    </For>
                </Divider>
            </Show>
            <a
                on:click=move |_| state.add_new_option()
            >
                Add another option
            </a>
        </Card>
    }
}

#[component]
fn OptionBlock(state: State, opt: (DefaultKey, OptionEntry)) -> impl IntoView {
    let (key, option) = opt;
    let OptionEntry { name, values, editing, new_value } = option;

    let add_value = move || {
        let val = new_value.get().trim().to_string();
        if !val.is_empty() {
            values.update(|v| {
                v.insert(val);
            });
            new_value.set("".into());
        }
    };

    view! {
        <div>
            <Show when=move || editing.get() fallback=move || {
                view! {
                    <Row>
                        <strong> {name} </strong>
                        <button on:click=move |_| editing.set(true)> Edit </button>
                    </Row>
                    <Badges>
                        <For each=move || values.get() key=|val| val.clone() let(value)>
                            <Badge> {value} </Badge>
                        </For>
                    </Badges>
                }
            }>
                <label> Option name
                    <input
                        type="text"
                        bind:value=name
                        placeholder="Option name (e.g. Color)"
                    />
                </label>
                <label> Option values
                    <fieldset>
                        <div>
                            <input
                                type="text"
                                bind:value=new_value
                                placeholder="Add value"
                                on:keydown=move |ev| if ev.key() == "Enter" { add_value(); }
                            />
                            <button type="button" on:click=move |_| add_value()> Add </button>
                        </div>
                        <Badges>
                            <For
                                each=move || values.get()
                                key=|val| val.clone()
                                let(val)
                            >
                                <Badge>
                                    <span> {val.clone()} </span>
                                    <button
                                        on:click=move |_| values.update(|v| v.retain(|x| x != &val))
                                    >"×"</button>
                                </Badge>
                            </For>
                        </Badges>
                    </fieldset>
                </label>
                <Row>
                    <button type="reset" on:click=move |_| state.remove_option(key)> Delete </button>
                    <button on:click=move |_| state.done_editing_option(key)> Done </button>
                </Row>
            </Show>
        </div>
    }
}

#[component]
fn Inspector(state: State) -> impl IntoView {
    view! {
        <Card>
            <fieldset>
                <label> Slug
                    <input
                        type="text"
                        pattern=r"^[\p{L}\p{N}]+(?:-[\p{L}\p{N}]+)*$"
                        bind:value=state.fields.slug
                        required
                    />
                </label>
            </fieldset>
            <fieldset>
                <label> Featured
                    <input type="checkbox" bind:checked=state.fields.featured />
                </label>
            </fieldset>
            <fieldset>
                <label> Category
                    <input type="text" bind:value=state.fields.category />
                </label>
            </fieldset>
        </Card>
    }
}