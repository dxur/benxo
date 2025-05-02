use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::*;
use crate::forms::product::ProductCreateAccessor;
use crate::services::products::ProductsService;

#[allow(non_upper_case_globals)]
pub const Products : super::Page = super::Page {
    title: "Products",
    view: ProductsView,
};

#[component]
fn ProductsView() -> AnyView {
    let service = ProductsService::new();
    view! {
        <Header title=Products.title>
            <button on:click=move |_| { service.dialog.get().map(|d| d.show()); }>
                New
            </button>
        </Header>
        <LazyShow
            when=move || service.status.get()
        >
            <ProductsTable service=service />
        </LazyShow>
        <ProductCreate service=service />
    }.into_any()
}

#[component]
fn ProductsTable(service: ProductsService) -> impl IntoView {
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
                each=move || service.products.get().data
                key=|_| ()
                let(product)
            >
                <tr>
                    <td>{product.slug}</td>
                    <td>{product.name}</td>
                    <td>{product.category}</td>
                    <td>{product.featured}</td>
                    <td>{product.base_price}</td>
                    <td>{product.base_discount}</td>
                    <td>
                        <button type="reset"
                            on:click=move |_| { service.delete_product(product.id); }
                        >
                            Archive
                        </button>
                    </td>
                </tr>
            </For>
        </Table>
        <TablePagination page=service.page total=service.total.read_only() />
    }
}

#[component]
fn ProductCreate(service: ProductsService) -> impl IntoView {
    let navigate = use_navigate();
    let acc: ProductCreateAccessor = Default::default();
    let new_attr = RwSignal::<String>::default();
    
    let add_attr = move |_| {
        if !new_attr.get_untracked().is_empty() {
            acc.attributes.update(|attrs| {attrs.insert(new_attr.get_untracked().trim().to_string());});
            new_attr.set("".to_string());
        }
    };

    let remove_attr = move |attr: String| {
        acc.attributes.update(|attrs| {attrs.remove(&attr);});
        new_attr.set(attr.clone());
    };

    view! {
        <Dialog node_ref=service.dialog on_cancel=move || { service.dialog.get().map(|d| d.close()); }>
            <header>
                <h2>Create Product</h2>
            </header>
            <form on:submit=move |ev| {
                ev.prevent_default();
                service.create_product(&acc, navigate.clone());
            }>
                <fieldset>
                    <label> Slug
                        <input
                            type="text"
                            pattern=r"^[\p{L}\p{N}]+(?:-[\p{L}\p{N}]+)*$"
                            bind:value=acc.slug
                            required
                        />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Name
                        <input type="text" bind:value=acc.name required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Description
                        <input type="text" bind:value=acc.description required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Featured
                        <input type="checkbox" bind:checked=acc.featured />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Category
                        <input type="text" bind:value=acc.category required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Base price
                        <input type="number" step=".01" bind:value=acc.base_price required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Base discount
                        <input type="number" step=".01" bind:value=acc.base_discount required />
                    </label>
                </fieldset>
                <fieldset>
                    <label> Attributes
                        <div>
                            <input type="text" bind:value=new_attr />
                            <button
                                type="button"
                                on:click=add_attr> Add </button>
                        </div>
                        <ul>
                            <For
                                each=move || acc.attributes.get()
                                key=|attr| attr.clone()
                                let(attr)
                            >
                                <li>
                                    <span> {attr.clone()} </span>
                                    <button
                                        type="button"
                                        on:click=move |_| {
                                            remove_attr(attr.clone())
                                    }>"×"</button>
                                </li>
                            </For>
                        </ul>
                    </label>
                </fieldset>
                // TODO: base_images
                <button type="button" 
                    on:click=move |_| { service.dialog.get().map(|d| d.close()); }
                >Close</button>
                <button type="submit">Submit</button>
            </form>
        </Dialog>
    }
}
