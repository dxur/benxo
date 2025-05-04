use leptos::prelude::*;

use crate::components::*;
use crate::pages::Page;
use super::state::IndexState as State;

#[allow(non_upper_case_globals)]
pub const ProductsIndex : Page = Page {
    title: "Products",
    view: View,
};

#[component]
fn View() -> AnyView {
    let state = State::new();
    view! {
        <Header title=ProductsIndex.title>
            <button on:click=move |_| { state.dialog.get().map(|d| d.show()); }>
                New
            </button>
        </Header>
        <LazyShow
            when=move || state.status.get()
        >
            <ProductsTable state=state />
        </LazyShow>
        <ProductCreate state=state />
    }.into_any()
}

#[component]
fn ProductsTable(state: State) -> impl IntoView {
    view! {
        <Table head=move || {
            view! {
                <tr>
                    <th>Slug</th>
                    <th>Name</th>
                    <th>Category</th>
                    <th>Featured</th>
                    <th>Base Price</th>
                    <th>Base Discount</th>
                    <th>Options</th>
                </tr>
            }
        }>
            <For
                each=move || state.products.get().data
                key=|_| ()
                let(product)
            >
                <tr>
                    <td>{product.slug}</td>
                    <td>{product.name}</td>
                    <td>{product.category}</td>
                    <td>{if product.featured { "Yes" } else { "No" }}</td>
                    <td>{product.base_price}</td>
                    <td>{product.base_compare_price}</td>
                    <td>
                        <button on:click=move |_| {
                            State::edit(product.id);
                        }> Edit </button>
                        <button type="reset" on:click=move |_| {
                            state.delete(product.id);
                        }> Delete </button>
                    </td>
                </tr>
            </For>
        </Table>
        <TablePagination page=state.page total=state.total.read_only() />
    }
}

#[component]
fn ProductCreate(state: State) -> impl IntoView {
    view! {
        <Dialog
            node_ref=state.dialog
            on_cancel=move || {
                state.dialog.get().map(|d| d.close());
            }
        >
            <header>
                <h2>Create Product</h2>
            </header>
            <form on:submit=move |ev| {
                ev.prevent_default();
                state.create();
            }>
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
                    <label> Name
                        <input type="text" bind:value=state.fields.name required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Category
                        <input type="text" bind:value=state.fields.category required />
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
