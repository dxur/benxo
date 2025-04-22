use common::{
    models::{Page, Pagination, product::ProductModelPublic},
    routes::{ApiRoutes, Routes},
};
use leptos::{html::*, prelude::*, task::spawn_local};

#[component]
pub fn Products() -> AnyView {
    let (products, set_products) = signal(Option::<Result<Page<ProductModelPublic>, String>>::None);
    let (current_page, set_current_page) = signal::<usize>(1);

    Effect::watch(
        move || current_page.get(),
        move |curr_page, prev_page, _| {
            log::info!("{}, {:?}", curr_page, prev_page);
            let page = *curr_page;
            if page > 10 {
                set_current_page.set(1);
                return;
            }
            spawn_local(async move {
                let res = ApiRoutes::get_all_products(Pagination {
                    page: Some(page),
                    per_page: None, // server default
                })
                .await
                .map_err(|e| e.to_string());
                set_products.set(Some(res));
            });
        },
        true,
    );

    let (sig, set_sig) = signal(false);

    view! {
        <Show
            when=move || { sig.get() }
        >
            <ProductCreate set_sig=set_sig />
        </Show>

        <header>
            <title>Products</title>
            <h1>Products</h1>
            <button on:click=move |_| { set_sig.set(true); } >
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
                                    set_current_page.update(|n| *n -= 1);
                                }
                            >
                                Previous
                            </button>
                            <span> {"Page: "} {current_page.get()} </span>
                            <button
                                disabled={!has_next}
                                on:click=move |_| {
                                    set_current_page.update(|n| *n += 1);
                                }
                            >
                                Next
                            </button>
                        </nav>
                    }
                }.into_any(),
                Some(Err(err)) => view! {
                    <section data-status>
                        <span> {err} </span>
                    </section>
                }.into_any(),
                None => view! {
                    <section data-status>
                        <span> Loading... </span>
                    </section>
                }.into_any(),
            }
        }
    }
    .into_any()
}

#[component]
fn ProductCreate(set_sig: WriteSignal<bool>) -> impl IntoView {
    view! {
        <div data-modal>
            <div data-backdrop on:click=move |_| set_sig.set(false)></div>

            <section data-modal-box aria-modal="true">
                <header>
                    <h2>Create Product</h2>
                </header>

                <form>
                    <fieldset>
                        <label for="product-name">Name</label>
                        <input id="product-name" type="text" />
                    </fieldset>

                    <fieldset>
                        <label for="product-category">Category</label>
                        <input id="product-category" type="text" />
                    </fieldset>

                    <nav data-actions>
                        <button type="button" on:click=move |_| set_sig.set(false)>
                            Cancel
                        </button>
                        <button type="submit">Create</button>
                    </nav>
                </form>
            </section>
        </div>
    }
}
