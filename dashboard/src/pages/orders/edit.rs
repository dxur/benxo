use leptos::prelude::*;

use super::state::EditState as State;
use crate::components::*;
use crate::pages::Page;

#[allow(non_upper_case_globals)]
pub const OrderEdit: Page = Page {
    title: "Order",
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
                    <header>
                            <div>
                                <Row>
                                    <h2> { move || state.fields.full_name.get() } </h2>
                                    <Badge> { move || state.fields.status.get().to_string() } </Badge>
                                </Row>
                                <h3> { state.id.unwrap().to_string() } </h3>
                            </div>
                            <Row>
                                <button type="reset"
                                    on:click=move |_| state.delete()
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
        <ProductSelect state=state />
    }.into_any()
}

#[component]
fn Body(state: State) -> impl IntoView {
    let edit_basic = RwSignal::new(false);
    let edit_shipping = RwSignal::new(false);
    let edit_cart = RwSignal::new(false);
    view! {
        <Card>
            <Row>
                <h3> Basic </h3>
                <button type="button"
                    on:click=move |_| edit_basic.update(|v| *v = !*v)
                > { move || if edit_basic.get() { "Done" } else { "Edit" } } </button>
            </Row>
            <fieldset>
                <label> Full Name
                    <input readonly=move || !edit_basic.get() type="text" bind:value=state.fields.full_name required />
                </label>
            </fieldset>
            <fieldset>
                <label> Phone
                    <input readonly=move || !edit_basic.get() type="tel" bind:value=state.fields.phone required />
                </label>
            </fieldset>
            <fieldset>
                <label> Email
                    <input readonly=move || !edit_basic.get() type="text" bind:value=state.fields.email required />
                </label>
            </fieldset>
        </Card>
        <Card>
            <Row>
                <h3> Shipping </h3>
                <button type="button"
                    on:click=move |_| edit_shipping.update(|v| *v = !*v)
                > { move || if edit_shipping.get() { "Done" } else { "Edit" } } </button>
            </Row>
            <fieldset>
                <label> Province
                    <input readonly=move || !edit_shipping.get() type="text" bind:value=state.fields.province required />
                </label>
            </fieldset>
            <fieldset>
                <label> Address
                    <input readonly=move || !edit_shipping.get() type="text" bind:value=state.fields.address required />
                </label>
            </fieldset>
            <fieldset>
                <label> Note
                    <textarea readonly=move || !edit_shipping.get() rows="10" bind:value=state.fields.note required />
                </label>
            </fieldset>
        </Card>
        <Card>
            <Row>
                <h3> Cart </h3>
                <button type="button"
                    on:click=move |_| edit_cart.update(|v| *v = !*v)
                > { move || if edit_cart.get() { "Done" } else { "Edit" } } </button>
            </Row>
            <Show
                when=move || !state.fields.items.get().is_empty()
            >
                <Table head=view! {
                    <tr>
                        <th> SKU </th>
                        <th> Quantity </th>
                        <th> Price </th>
                        <th> Options </th>
                    </tr>
                }>
                    <For
                        each=move || state.fields.items.get()
                        key=|item| item.product_sku.clone()
                        let(item)
                    >
                        <tr>
                            <td>{item.product_sku.clone()}</td>
                            <td>
                                <input
                                    type="number"
                                    readonly=move || !edit_cart.get()
                                    min="1"
                                    step="1" bind:value=item.quantity
                                />
                            </td>
                            <td>
                                <input
                                    type="number"
                                    readonly=move || !edit_cart.get()
                                    min="0"
                                    step="0.01" bind:value=item.price
                                />
                            </td>
                            <td>
                                <button type="reset"
                                    disabled=move || !edit_cart.get()
                                    on:click=move |_| {
                                        state.remove_item(item.product_sku.as_str());
                                }> Remove </button>
                            </td>
                        </tr>
                    </For>
                </Table>
            </Show>
            <Show
                when=move || edit_cart.get()
            >
                <a
                    on:click=move |_| state.add_new_item()
                >
                    Add another item
                </a>
            </Show>
        </Card>
    }
}

#[component]
fn Inspector(state: State) -> impl IntoView {
    view! {
        <Card>
            <h3> Progress </h3>
            <Timeline>
                <For
                    each=move || state.fields.history.get().into_iter().enumerate()
                    key=|(idx, _)| *idx
                    let((_, entry))
                >
                    <li>
                        <strong> {entry.status.to_string()} </strong>
                        <time> May 1, 2024 - 10:00 AM </time>
                    </li>
                </For>
            </Timeline>
            <Show
                when=move || state.fields.progress.get().is_some()
            >
                <button type="submit"
                    on:click=move |ev| {
                        ev.prevent_default();
                        state.progress();
                    }
                > "Mark as " { state.fields.progress.get().unwrap().to_string() } </button>
            </Show>
            <Show
                when=move || state.fields.regress.get().is_some()
            >
                <button type="reset"
                    on:click=move |ev| {
                        ev.prevent_default();
                        state.regress();
                    }
                > "Mark as " { state.fields.regress.get().unwrap().to_string() } </button>
            </Show>
        </Card>
    }
}

#[component]
fn ProductSelect(state: State) -> impl IntoView {
    view! {
        <Dialog
            node_ref=state.dialog
            on_cancel=move || {
                state.dialog.get().map(|d| d.close());
            }
        >

            <header>
                <h2>Select Product</h2>
            </header>
            <form on:submit=move |ev| {
                ev.prevent_default();
                state.add_item();
            }>
                <fieldset>
                    <input
                        type="text"
                        placeholder="Search"
                        bind:value=state.search
                    />
                </fieldset>
                <fieldset>
                    <label> Quantity
                        <input type="number" bind:value=state.fields.item_quantity required />
                    </label>
                </fieldset>
                <button type="button"
                    on:click=move |_| { state.dialog.get().map(|d| d.close()); }
                >Close</button>
                <button type="submit">Submit</button>
            </form>
        </Dialog>
    }
}
