use common::models::product::{self, ProductModel, ProductModelCreate};
use leptos::prelude::*;

use super::{Accessor, IntoForm};

#[derive(Debug, Copy, Clone, Default)]
pub struct ProductCreateAccessor {
    pub name: RwSignal<String>,
    pub description: RwSignal<String>,
    pub featured: RwSignal<bool>,
    pub category: RwSignal<String>,
    pub base_price: RwSignal<String>,
    pub base_discount: RwSignal<String>,
    pub base_images: RwSignal<Vec<String>>,
    pub slug: RwSignal<String>,
}

impl TryFrom<ProductCreateAccessor> for ProductModelCreate {
    type Error = ();

    fn try_from(value: ProductCreateAccessor) -> Result<Self, Self::Error> {
        let base_price = value.base_price.get().parse().map_err(|_| ())?;
        let base_discount = value.base_discount.get().parse().map_err(|_| ())?;

        Ok(ProductModelCreate {
            name: value.name.get(),
            description: value.description.get(),
            featured: value.featured.get(),
            category: value.category.get(),
            base_price: base_price,
            base_discount: base_discount,
            base_images: value.base_images.get(),
            slug: value.slug.get(),
        })
    }
}

impl Accessor for ProductModel {
    type CreateAccessor = ProductCreateAccessor;
    type UpdateAccessor = ();
}

impl IntoForm for ProductModel {
    fn build_create_form(acc: Self::CreateAccessor, outlet: AnyView) -> AnyView {
        view! {
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
                    <input type="checkbox" bind:value=acc.featured />
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
                <label> Slug
                    <input
                        type="text"
                        pattern=r"^[\p{L}\p{N}]+(?:-[\p{L}\p{N}]+)*$"
                        bind:value=acc.slug
                        required
                    />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }
}
