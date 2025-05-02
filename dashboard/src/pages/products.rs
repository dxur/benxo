use leptos::prelude::*;
use leptos_router::hooks::use_navigate;

use crate::components::*;
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
            <button on:click=move |_| { service.create_acc.set(Some(Default::default())); }>
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
    view! {
        <Dialog node_ref=service.dialog on_cancel=move || { service.create_acc.set(None); }>
            <header>
                <h2>Create Product</h2>
            </header>
            <form on:submit=move |ev| {
                ev.prevent_default();
                service.create_product(navigate.clone());
            }>
                // {
                //     Product::build_create_form(
                //         acc.clone(),
                //         view! {
                //         }.into_any()
                //     )
                // }
                <button type="button" 
                    on:click=move |_| { service.create_acc.set(None); }
                >Close</button>
                <button type="submit">Submit</button>
            </form>
        </Dialog>
    }
}
