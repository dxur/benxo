use common::{
    models::{
        Page, Pagination,
        product::{ProductModel, ProductModelCreate, ProductModelDelete, ProductModelPublic},
    },
    routes::{ApiRoutes, Routes},
};
use leptos::{html::*, prelude::*, task::spawn_local};

use crate::forms::Accessor;
use crate::forms::IntoForm;

#[component]
pub fn Products() -> AnyView {
    let (products, set_products) = signal(Option::<Result<Page<ProductModelPublic>, String>>::None);
    let (current_page, set_current_page) = signal::<usize>(1);
    let (page, set_page) = signal(1 as usize);
    let (on_create, set_on_create) = signal(());

    Effect::watch(
        move || (page.get(), on_create.get()),
        move |this_page, _, _| {
            let (req_page, _) = *this_page;
            spawn_local(async move {
                let res = ApiRoutes::get_all_products(Pagination {
                    page: Some(req_page),
                    per_page: None, // server default
                })
                .await
                .map_err(|e| e.to_string());
                if let Ok(data) = &res {
                    set_current_page.set(data.page);
                }
                set_products.set(Some(res));
            });
        },
        true,
    );

    let (create_modal, set_create_modal) = signal(false);

    view! {
        <Show
            when=move || { create_modal.get() }
        >
            <ProductCreate set_modal=set_create_modal set_on_create=set_on_create />
        </Show>
        <header>
            <title>Products</title>
            <h1>Products</h1>
            <button on:click=move |_| { set_create_modal.set(true); } >
                New
            </button>
        </header>
        {
            move || match products.get() {
                Some(Ok(data)) => {
                    let has_next = data.has_next();
                    let has_prev = data.has_prev();
                    view! {
                        <div data-table>
                            <table>
                                <thead>
                                    <tr>
                                        <th>Slug</th>
                                        <th>Name</th>
                                        <th>Category</th>
                                        <th>Featured</th>
                                        <th>Base Price</th>
                                        <th>Base Discount</th>
                                        <th>Options</th>
                                    </tr>
                                </thead>
                                <tbody>
                                    {
                                        data.data.into_iter().map(|product| view! {
                                            <tr>
                                                <td>{product.slug}</td>
                                                <td>{product.name}</td>
                                                <td>{product.category}</td>
                                                <td>{product.featured}</td>
                                                <td>{product.base_price}</td>
                                                <td>{product.base_discount}</td>
                                                <td>
                                                    <button
                                                        on:click=move |_| {
                                                            set_create_modal.set(true);
                                                        }
                                                    >
                                                        Edit
                                                    </button>
                                                    <button type="reset"
                                                        on:click=move |_| {
                                                            spawn_local(async move {
                                                                match ApiRoutes::delete_product(ProductModelDelete { id: product.id }).await {
                                                                    Ok(_) => set_page.set(current_page.get_untracked()),
                                                                    Err(err) => log::error!("Failed to delete product: {err}"),
                                                                }
                                                            });
                                                        }
                                                    >
                                                        Delete
                                                    </button>
                                                </td>
                                            </tr>
                                        }).collect_view()
                                    }
                                </tbody>
                            </table>
                        </div>
                        <nav>
                            <button
                                disabled={!has_prev}
                                on:click=move |_| {
                                    set_page.set(current_page.get_untracked() - 1);
                                }
                            >
                                Previous
                            </button>
                            <span> {"Page: "} {current_page.get()} </span>
                            <button
                                disabled={!has_next}
                                on:click=move |_| {
                                    set_page.set(current_page.get_untracked() + 1);
                                }
                            >
                                Next
                            </button>
                        </nav>
                    }
                }.into_any(),
                Some(Err(err)) => view! {
                    <div data-status>
                        <span> {err} </span>
                    </div>
                }.into_any(),
                None => view! {
                    <div data-status>
                        <span> Loading... </span>
                    </div>
                }.into_any(),
            }
        }
    }
    .into_any()
}

#[component]
fn ProductCreate(set_modal: WriteSignal<bool>, set_on_create: WriteSignal<()>) -> impl IntoView {
    let acc = <ProductModel as Accessor>::CreateAccessor::default();
    view! {
        <div data-modal>
            <div data-backdrop on:click=move |_| set_modal.set(false)></div>
            <section data-modal-box aria-modal="true">
                <header>
                    <h2>Create Product</h2>
                </header>
                <form on:submit=move |ev| {
                    ev.prevent_default();
                    match ProductModelCreate::try_from(acc) {
                        Ok(prod) => {
                            spawn_local(async move {
                                let res = ApiRoutes::create_product(prod).await;
                                log::debug!("Created product: {:?}", res);
                                set_on_create.set(());
                                set_modal.set(false);
                            });
                        }
                        Err(err) => {
                            log::error!("Failed to create product: {:?}", err);
                        }
                    }
                }>
                    {
                        ProductModel::build_create_form(
                            acc,
                            view! {
                                <button on:click=move |_| set_modal.set(false)>Close</button>
                                <button type="submit">Submit</button>
                            }.into_any())
                    }
                </form>
            </section>
        </div>
    }
}
