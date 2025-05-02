use std::{collections::HashSet, sync::Arc};

use backend::models::{product::*, ObjectId};
use leptos::prelude::*;

use super::{Accessor, IntoForm};

#[derive(Debug, Clone, Default)]
pub struct ProductCreateAccessor {
    pub name: RwSignal<String>,
    pub description: RwSignal<String>,
    pub featured: RwSignal<bool>,
    pub category: RwSignal<String>,
    pub base_price: RwSignal<String>,
    pub base_discount: RwSignal<String>,
    pub base_images: RwSignal<Vec<String>>,
    pub attributes: RwSignal<HashSet<String>>,
    pub slug: RwSignal<String>,
}

impl TryFrom<&ProductCreateAccessor> for ProductCreate {
    type Error = ();

    fn try_from(value: &ProductCreateAccessor) -> Result<Self, Self::Error> {
        let base_price = value.base_price.get().parse().map_err(|_| ())?;
        let base_discount = value.base_discount.get().parse().map_err(|_| ())?;

        Ok(ProductCreate {
            name: value.name.get(),
            description: value.description.get(),
            featured: value.featured.get(),
            category: value.category.get(),
            base_price: base_price,
            base_discount: base_discount,
            base_images: value.base_images.get(),
            slug: value.slug.get(),
            attributes: value.attributes.get(),
        })
    }
}

#[derive(Debug, Clone)]
pub struct ProductUpdateAccessor {
    pub id: ObjectId,
    pub origin: Arc<(bool, HashSet<String>)>,
    pub name: RwSignal<String>,
    pub description: RwSignal<String>,
    pub featured: RwSignal<bool>,
    pub category: RwSignal<String>,
    pub base_price: RwSignal<String>,
    pub base_discount: RwSignal<String>,
    pub base_images: RwSignal<Vec<String>>,
    pub attributes: RwSignal<HashSet<String>>,
    pub slug: RwSignal<String>,
}

impl TryFrom<ProductUpdateAccessor> for ProductUpdate {
    type Error = ();

    fn try_from(value: ProductUpdateAccessor) -> Result<Self, Self::Error> {
        let base_price: Option<f32> = match value.base_price.get().trim() {
            "" => None,
            s => Some(s.parse().map_err(|_| ())?),
        };
        
        let base_discount: Option<f32> = match value.base_discount.get().trim() {
            "" => None,
            s => Some(s.parse().map_err(|_| ())?),
        };

        log::debug!("base_price: {:?}, base_discount: {:?}", base_price, base_discount);
        Ok(ProductUpdate {
            id: value.id,
            body: ProductUpdateBody {
                name: Some(value.name.get()).filter(|s| !s.is_empty()),
                description: Some(value.description.get()).filter(|s| !s.is_empty()),
                featured: if value.featured.get() != value.origin.0 { Some(value.featured.get()) } else { None },
                category: Some(value.category.get()).filter(|s| !s.is_empty()),
                base_price: base_price,
                base_discount: base_discount,
                base_images: Some(value.base_images.get()).filter(|s| !s.is_empty()),
                attributes: Some(value.attributes.get()).filter(|s| s != &value.origin.1),
                slug: Some(value.slug.get()).filter(|s| !s.is_empty()),
            }
        })
    }
}

impl From<&ProductPublic> for ProductUpdateAccessor {
    fn from(value: &ProductPublic) -> Self {
        Self {
            id: value.id,
            origin: Arc::new((value.featured, value.attributes.clone())),
            name: RwSignal::default(),
            description: RwSignal::default(),
            featured: RwSignal::new(value.featured),
            category: RwSignal::default(),
            base_price: RwSignal::default(),
            base_discount: RwSignal::default(),
            base_images: RwSignal::default(),
            attributes: RwSignal::new(value.attributes.clone()),
            slug: RwSignal::default(),
        }
    }
}

impl Accessor for Product {
    type CreateAccessor = ProductCreateAccessor;
    type UpdateAccessor = ProductUpdateAccessor;
}

impl IntoForm<ProductPublic> for Product {
    fn build_create_form(acc: Self::CreateAccessor, outlet: AnyView) -> AnyView {
        let new_attr = RwSignal::<String>::default();
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
                            on:click= move |_| {
                                if !new_attr.get_untracked().is_empty() {
                                    acc.attributes.update(|attrs| {attrs.insert(new_attr.get_untracked().trim().to_string());});
                                    new_attr.set("".to_string());
                                }
                            }
                        > Add </button>
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
                                    on:click= move |_| {
                                        acc.attributes.update(|attrs| {attrs.remove(&attr);});
                                        new_attr.set(attr.clone());
                                    }
                                >"×"</button>
                            </li>
                        </For>
                    </ul>
                </label>
            </fieldset>
            // TODO: base_images
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

    fn build_update_form(
        val: ProductPublic,
        acc: Self::UpdateAccessor,
        outlet: AnyView,
    ) -> AnyView {
        let new_attr = RwSignal::<String>::default();
        view! {
            <fieldset>
                <label> Name
                    <input type="text" placeholder=val.name bind:value=acc.name />
                </label>
            </fieldset>
            <fieldset>
                <label> Description
                    <input type="text" placeholder=val.description bind:value=acc.description />
                </label>
            </fieldset>
            <fieldset>
                <label> Featured
                    <input type="checkbox" bind:checked=acc.featured />
                </label>
            </fieldset>
            <fieldset>
                <label> Category
                    <input type="text" placeholder=val.category bind:value=acc.category />
                </label>
            </fieldset>
            <fieldset>
                <label> Base price
                    <input type="number" placeholder=val.base_price step=".01" bind:value=acc.base_price />
                </label>
            </fieldset>
            <fieldset>
                <label> Base discount
                    <input type="number" placeholder=val.base_discount step=".01" bind:value=acc.base_discount />
                </label>
            </fieldset>
            <fieldset>
                <label> Attributes
                    <div>
                        <input type="text" bind:value=new_attr />
                        <button
                            type="button"
                            on:click= move |_| {
                                if !new_attr.get_untracked().is_empty() {
                                    acc.attributes.update(|attrs| {attrs.insert(new_attr.get_untracked());});
                                    new_attr.set("".to_string());
                                }
                            }
                        > Add </button>
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
                                    on:click= move |_| {
                                        acc.attributes.update(|attrs| {attrs.remove(&attr);});
                                        new_attr.set(attr.clone());
                                    }
                                >"×"</button>
                            </li>
                        </For>
                    </ul>
                </label>
            </fieldset>
            <fieldset>
                <label> Slug
                    <input
                        type="text"
                        pattern=r"^[\p{L}\p{N}]+(?:-[\p{L}\p{N}]+)*$"
                        placeholder=val.slug
                        bind:value=acc.slug
                        
                    />
                </label>
            </fieldset>
            {outlet}
        }
        .into_any()
    }
}
