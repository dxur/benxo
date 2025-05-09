use backend::api::{ApiRoutes, Routes};
use backend::models::product::*;
use backend::models::{ObjectId, Page, Pagination};
use indexmap::{IndexMap, IndexSet};
use leptos::{html::*, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use slotmap::{DefaultKey, SlotMap};
use std::str::FromStr;

use crate::notifications::{error, success};
use crate::routes::*;
use crate::utils::*;

#[derive(Clone, Copy, Default)]
pub struct CreateFields {
    pub name: RwSignal<String>,
    pub category: RwSignal<String>,
    pub slug: RwSignal<String>,
}

#[derive(Clone, Copy)]
pub struct IndexState {
    pub products: RwSignal<Page<ProductPublic>>,
    pub status: RwSignal<LoadingStatus>,
    pub page: RwSignal<usize>,
    pub total: RwSignal<usize>,
    pub dialog: NodeRef<Dialog>,
    pub fields: CreateFields,
}

impl IndexState {
    pub fn new() -> Self {
        let state = Self {
            products: Default::default(),
            status: Default::default(),
            page: RwSignal::new(1),
            total: Default::default(),
            dialog: Default::default(),
            fields: Default::default(),
        };

        Effect::watch(
            move || state.page.get(),
            move |_, _, _| {
                state.fetch();
            },
            true,
        );
        state
    }

    pub fn fetch(self) {
        self.status.set(LoadingStatus::Loading);
        spawn_local(async move {
            let res = ApiRoutes::get_all_products(Pagination {
                page: Some(self.page.get_untracked()),
                per_page: None,
            })
            .await
            .map_err(|e| e.to_string());
            log::debug!("Fetched products: {:?}", res);
            match res {
                Ok(data) => {
                    self.total.set(data.total_pages());
                    self.products.set(data);
                    self.status.set(LoadingStatus::Ok);
                }
                Err(e) => {
                    self.status.set(LoadingStatus::Err(e));
                }
            }
        });
    }

    pub fn create(self) {
        let res = self.try_create();
        log::debug!("Into product: {:?}", res);
        match res {
            Ok(product) => spawn_local(async move {
                let res = ApiRoutes::create_product(product).await;
                log::debug!("Created product: {:?}", res);
                match res {
                    Ok(product) => {
                        success("Product created");
                        Self::edit(product.id);
                    }
                    Err(e) => error(e),
                }
            }),
            Err(e) => error(e),
        }
    }

    pub fn delete(self, id: ObjectId) {
        spawn_local(async move {
            let res = ApiRoutes::delete_product(ProductDelete { id }).await;
            log::debug!("Deleted product: {:?}", res);
            match res {
                Ok(_) => {
                    success("Product deleted");
                    self.fetch();
                }
                Err(e) => error(e),
            }
        });
    }

    pub fn edit(id: ObjectId) {
        navigate(
            AppRoutes::PRODUCT
                .path()
                .replace(":id", &id.to_hex())
                .as_str(),
            Default::default(),
        );
    }

    pub fn try_create(&self) -> Result<ProductCreate> {
        Ok(ProductCreate {
            name: self.fields.name.get_untracked(),
            category: self.fields.category.get_untracked(),
            slug: self.fields.slug.get_untracked(),
        })
    }
}

#[derive(Clone)]
pub struct OptionEntry {
    pub name: RwSignal<String>,
    pub values: RwSignal<IndexSet<String>>,
    pub editing: RwSignal<bool>,
    pub new_value: RwSignal<String>,
}

#[derive(Clone)]
pub struct VariantEntry {
    pub sku: RwSignal<String>,
    pub options: RwSignal<IndexMap<String, RwSignal<String>>>,
    pub price: RwSignal<String>,
    pub compare_price: RwSignal<String>,
    pub availability: RwSignal<String>,
    pub editing: RwSignal<bool>,
}

#[derive(Clone, Copy, Default)]
pub struct Fields {
    pub name: RwSignal<String>,
    pub description: RwSignal<String>,
    pub featured: RwSignal<bool>,
    pub category: RwSignal<String>,
    pub base_price: RwSignal<String>,
    pub base_compare_price: RwSignal<String>,
    pub base_images: RwSignal<Vec<String>>,
    pub options: RwSignal<SlotMap<DefaultKey, OptionEntry>>,
    pub variants: RwSignal<SlotMap<DefaultKey, VariantEntry>>,
    pub slug: RwSignal<String>,
}

#[derive(Clone, Copy)]
pub struct EditState {
    pub id: Option<ObjectId>,
    pub status: RwSignal<LoadingStatus>,
    pub product: RwSignal<Option<ProductPublic>>,
    pub fields: Fields,
}

impl EditState {
    pub fn new() -> Self {
        let id_opt = use_params_map()
            .read_untracked()
            .get("id")
            .and_then(|s| ObjectId::from_str(s.as_str()).ok());

        let status = if let Some(id) = id_opt {
            log::debug!("ID: {}", id);
            LoadingStatus::Loading
        } else {
            LoadingStatus::Err("Can't Parse ID".to_string())
        };

        let state = Self {
            id: id_opt,
            status: RwSignal::new(status),
            product: Default::default(),
            fields: Default::default(),
        };
        state.fetch();
        state
    }

    pub fn fetch(self) {
        if let Some(id) = self.id {
            spawn_local(async move {
                let res = ApiRoutes::get_one_product(ProductFetch { id }).await;
                log::debug!("Fetched product: {:?}", res);
                match res {
                    Ok(product) => {
                        self.update_fields(product.clone());
                        self.product.set(Some(product));
                        self.status.set(LoadingStatus::Ok);
                    }
                    Err(e) => {
                        self.status.set(LoadingStatus::Err(e.to_string()));
                    }
                }
            });
        }
    }

    pub fn update(self) {
        let res: Result<ProductUpdate> = self.try_update();
        log::debug!("Into product: {:?}", res);
        match res {
            Ok(product) => spawn_local(async move {
                let res = ApiRoutes::update_product(product).await;
                log::debug!("Update product: {:?}", res);
                match res {
                    Ok(product) => {
                        success("Product updated");
                        self.update_fields(product.clone());
                        self.product.set(Some(product));
                    }
                    Err(e) => error(e),
                }
            }),
            Err(e) => error(e),
        }
    }

    pub fn delete(self) {
        if let Some(id) = self.id {
            spawn_local(async move {
                let res = ApiRoutes::delete_product(ProductDelete { id }).await;
                log::debug!("Deleted product: {:?}", res);
                match res {
                    Ok(_) => {
                        success("Product deleted");
                        navigate(AppRoutes::PRODUCTS.path(), Default::default());
                    }
                    Err(e) => error(e),
                }
            });
        } else {
            error("Can't delete product. unknown id");
        }
    }

    pub fn add_new_option(self) {
        let option = OptionEntry {
            name: Default::default(),
            values: Default::default(),
            editing: RwSignal::new(true),
            new_value: Default::default(),
        };
        self.fields.options.update(|v| {
            v.insert(option);
        });
    }

    pub fn remove_option(self, key: DefaultKey) {
        self.fields.options.update(|v| {
            v.remove(key);
        });
    }

    pub fn done_editing_option(self, key: DefaultKey) {
        if self
            .fields
            .options
            .get_untracked()
            .get(key)
            .unwrap()
            .name
            .get_untracked()
            .is_empty()
            || self
                .fields
                .options
                .get_untracked()
                .get(key)
                .unwrap()
                .values
                .get_untracked()
                .is_empty()
        {
            error("Option name and values can't be empty");
            return;
        }
        self.fields.options.update(|v| {
            let entry = v.get_mut(key).unwrap();
            entry.editing.set(false);
        });
    }

    pub fn option_available(self, variant: DefaultKey, option: String, value: String) -> bool {
        for (key, entry) in self.fields.variants.get_untracked() {
            if key == variant {
                continue;
            }
            if let Some(opt) = entry.options.get_untracked().get(&option) {
                if opt.get() == value {
                    return false;
                }
            }
        }
        true
    }

    pub fn bind_option(self, variant: DefaultKey, option: String) -> RwSignal<String> {
        let variants = self.fields.variants.get_untracked();
        let entry = variants.get(variant).unwrap();
        if let Some(opt) = entry.options.get_untracked().get(&option) {
            opt.clone()
        }else {
            let signal = RwSignal::new(Default::default());
            entry.options.update(|v| {
                v.insert(option, signal);
            });
            signal
        }
    }

    pub fn add_new_variant(self) {
        // TODO: check if there is room for another variant
        if false {
            error("Can't add another variant");
        }
        let entry = VariantEntry {
            sku: Default::default(),
            options: Default::default(),
            price: Default::default(),
            compare_price: Default::default(),
            editing: RwSignal::new(true),
            availability: Default::default(),
        };
        self.fields.variants.update(|v| {
            v.insert(entry);
        });
    }

    pub fn remove_variant(self, key: DefaultKey) {
        self.fields.variants.update(|v| {
            v.remove(key);
        });
    }

    pub fn done_editing_variant(self, key: DefaultKey) {
        // TODO: validate the variant
        self.fields.variants.update(|v| {
            let entry = v.get_mut(key).unwrap();
            entry.editing.set(false);
        });
    }

    fn update_fields(&self, product: ProductPublic) {
        self.fields.name.set(product.name);
        self.fields.description.set(product.description);
        self.fields.featured.set(product.featured);
        self.fields.category.set(product.category);
        self.fields.base_price.set(product.base_price.to_string());
        self.fields
            .base_compare_price
            .set(product.base_compare_price.to_string());
        self.fields.base_images.set(product.base_images);
        self.fields.slug.set(product.slug);
        self.fields.variants.update(|v| {
            v.clear();
            for ProductVariant {
                sku,
                options,
                price,
                compare_price,
                stocks,
                images,
            } in product.variants
            {
                let opts = options
                    .into_iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if let Some((opt_name, opt_vals)) = product.options.get_index(i) {
                            (
                                opt_name.clone(),
                                RwSignal::new(
                                    opt_vals
                                        .get_index(v)
                                        .cloned()
                                        .unwrap_or_else(|| "unknown".to_string()),
                                ),
                            )
                        } else {
                            ("unknown".to_string(), RwSignal::new("unknown".to_string()))
                        }
                    })
                    .collect();
                let entry = VariantEntry {
                    sku: RwSignal::new(sku),
                    options: RwSignal::new(opts),
                    editing: RwSignal::new(false),
                    price: RwSignal::new(price.map_or(Default::default(), |v| v.to_string())),
                    compare_price: RwSignal::new(
                        compare_price.map_or(Default::default(), |v| v.to_string()),
                    ),
                    availability: RwSignal::new(stocks.to_string()),
                };
                v.insert(entry);
            }
        });
        self.fields.options.update(|v| {
            v.clear();
            for (name, opts) in product.options {
                let entry = OptionEntry {
                    name: RwSignal::new(name),
                    values: RwSignal::new(opts),
                    editing: RwSignal::new(false),
                    new_value: Default::default(),
                };
                v.insert(entry);
            }
        });
    }

    fn try_update(&self) -> Result<ProductUpdate> {
        let base_price = self
            .fields
            .base_price
            .get_untracked()
            .parse()
            .map_err(|_| "failed to parse base price".to_string())?;
        let base_compare_price = self
            .fields
            .base_compare_price
            .get_untracked()
            .parse()
            .map_err(|_| "failed to parse base discount".to_string())?;
        let options = self.try_get_options()?;

        let name = self.fields.name.get_untracked();
        if name.is_empty() {
            return Err("Product name can't be empty".to_string());
        }

        if let Some(product) = self.product.get_untracked() {
            let body = ProductUpdateBody {
                name: Some(name).filter(|v| *v != product.name),
                description: Some(self.fields.description.get_untracked())
                    .filter(|v| *v != product.description),
                featured: Some(self.fields.featured.get_untracked())
                    .filter(|v| *v != product.featured),
                category: Some(self.fields.category.get_untracked())
                    .filter(|v| *v != product.category),
                base_price: Some(base_price).filter(|v| *v != product.base_price),
                base_compare_price: Some(base_compare_price)
                    .filter(|v| *v != product.base_compare_price),
                base_images: Some(self.fields.base_images.get_untracked())
                    .filter(|v| *v != product.base_images),
                options: Some(options).filter(|v| *v != product.options),
                variants: None,
                slug: Some(self.fields.slug.get_untracked()).filter(|v| *v != product.slug),
            };
            if body.is_none() {
                Err("Nothing Have been Updated".to_string())
            } else {
                Ok(ProductUpdate {
                    id: product.id,
                    body,
                })
            }
        } else {
            Err("Can't read the product".to_string())
        }
    }

    fn try_get_options(&self) -> Result<IndexMap<String, IndexSet<String>>> {
        let mut options = IndexMap::new();
        for (_, entry) in self.fields.options.get_untracked().iter() {
            if entry.editing.get_untracked() {
                Err("Can't update options while editing".to_string())?;
            }
            let name = entry.name.get_untracked();
            let values = entry.values.get_untracked();
            if name.is_empty() && values.is_empty() {
                Err("Option name and values can't be empty".to_string())?;
            }
            options.insert(name, values.into_iter().collect());
        }
        Ok(options)
    }
}
