use leptos::prelude::*;
use slotmap::DefaultKey;

use super::state::{EditState as State, OptionEntry, VariantEntry};
use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const ProductEdit: Page = Page {
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
                state.update();
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
    }
    .into_any()
}

#[component]
fn Body(state: State) -> impl IntoView {
    view! {
        <Card>
            <h3> Basic </h3>
            <fieldset>
                <label> Name <input type="text" bind:value=state.fields.name required /></label>
            </fieldset>
            <fieldset>
                <label> Description <textarea rows="10" bind:value=state.fields.description /></label>
            </fieldset>
        </Card>

        <Card>
            <h3> Pricing </h3>
            <fieldset>
                <Row>
                    <label> price <input type="number" bind:value=state.fields.base_price step=".01" /></label>
                    <label> Compare-at price <input type="number" bind:value=state.fields.base_compare_price step=".01" /></label>
                </Row>
            </fieldset>
        </Card>

        <Card>
            <h3> Options </h3>
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

        <Card>
            <h3> Variants </h3>
            <Show
                when=move || !state.fields.variants.get().is_empty()
            >
                <Divider>
                    <For
                        each=move || state.fields.variants.get()
                        key=|(key, _)| *key
                        let(opt)
                    >
                        <VariantBlock state=state opt=opt />
                    </For>
                </Divider>
            </Show>
            <a
                on:click=move |_| state.add_new_variant()
            >
                Add another variant
            </a>
        </Card>
    }
}

#[component]
fn OptionBlock(state: State, opt: (DefaultKey, OptionEntry)) -> impl IntoView {
    let (key, option) = opt;
    let OptionEntry {
        name,
        values,
        editing,
        new_value,
    } = option;

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
                        <button type="button" on:click=move |_| editing.set(true)> Edit </button>
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
                                on:keydown=move |ev| {
                                    if ev.key() == "Enter" {
                                        ev.prevent_default();
                                        add_value();
                                    }
                                }
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
                                        type="button"
                                        on:click=move |_| values.update(|v| v.retain(|x| x != &val))
                                    >"×"</button>
                                </Badge>
                            </For>
                        </Badges>
                    </fieldset>
                </label>
                <Row>
                    <button type="reset" on:click=move |_| state.remove_option(key)> Delete </button>
                    <button type="button" on:click=move |_| state.done_editing_option(key)> Done </button>
                </Row>
            </Show>
        </div>
    }
}

#[component]
fn VariantBlock(state: State, opt: (DefaultKey, VariantEntry)) -> impl IntoView {
    let (key, variant) = opt;
    let VariantEntry {
        sku,
        options,
        editing,
        price,
        compare_price,
        availability,
    } = variant;

    view! {
        <div>
            <Show when=move || editing.get() fallback=move || {
                view! {
                    <Row>
                        <strong> {sku} </strong>
                        <button type="button" on:click=move |_| editing.set(true)> Edit </button>
                    </Row>
                    <Badges>
                        <For each=move || options.get() key=|val| val.clone() let((_, value))>
                            <Badge> {value} </Badge>
                        </For>
                    </Badges>
                }
            }>
                <label> SKU
                    <input
                        type="text"
                        bind:value=sku
                        placeholder="Variant SKU"
                    />
                </label>
                <For
                    each=move || state.fields.options.get()
                    key=|(key, _)| key.clone()
                    let((_, value))
                >
                    <label> {value.name}
                        <select bind:value=state.bind_option(key, value.name.get())> {
                            value.values.get().into_iter().enumerate().map(|(i, v)| view! {
                                <option 
                                    // selected=move || state.bind_option(key, value.name.get()).get() == v
                                    disabled=move || !state.option_available(key, value.name.get(), v.clone())
                                > {v.clone()} </option>
                            }).collect_view()
                        } </select>
                    </label>
                </For>
                <Row>
                    <label> price <input type="number" bind:value=price step=".01" /></label>
                    <label> Compare-at price <input type="number" bind:value=compare_price step=".01" /></label>
                </Row>
                <label> Availability
                    <input type="number" bind:value=availability step="1"/>
                </label>
                <Row>
                    <button type="reset" on:click=move |_| state.remove_variant(key)> Delete </button>
                    <button type="button" on:click=move |_| state.done_editing_variant(key)> Done </button>
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
