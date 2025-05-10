use backend::api::{ApiRoutes, Routes};
use backend::models::product::*;
use backend::models::{ObjectId, Page, Pagination};
use indexmap::{IndexMap, IndexSet};
use leptos::{html::*, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use slotmap::{DefaultKey, SlotMap};
use std::str::FromStr;

use crate::notifications::*;
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
    pub options: RwSignal<IndexMap<String, String>>,
    pub price: RwSignal<String>,
    pub compare_price: RwSignal<String>,
    pub availability: RwSignal<String>,
    pub included: RwSignal<bool>,
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

        Effect::watch(
            move || state.fields.options.get(),
            move |options, _, _| {
                let options = options.iter().collect::<Vec<_>>();

                state.fields.variants.update(|variants| {
                    variants.retain(|_, variant| {
                        let variant_options = variant.options.get();
                        if variant_options.len() != options.len() {
                            return false;
                        }

                        for (_, option_entry) in &options {
                            match variant_options.get(&option_entry.name.get()) {
                                Some(value) => {
                                    if !option_entry.values.get().contains(value) {
                                        return false;
                                    }
                                }
                                None => return false,
                            }
                        }
                        true
                    });

                    let mut combinations = vec![IndexMap::new()];
                    for (_, option_entry) in &options {
                        let values = option_entry.values.get();
                        let mut new_combinations = Vec::new();

                        for existing_combo in combinations {
                            for value in values.iter() {
                                let mut new_combo = existing_combo.clone();
                                new_combo.insert(option_entry.name.get(), value.clone());
                                new_combinations.push(new_combo);
                            }
                        }

                        combinations = new_combinations;
                    }

                    let existing_combinations: Vec<IndexMap<String, String>> =
                        variants.iter().map(|(_, v)| v.options.get()).collect();

                    for combination in combinations {
                        if !existing_combinations.contains(&combination) {
                            let new_variant = VariantEntry {
                                sku: RwSignal::new(String::new()),
                                options: RwSignal::new(combination),
                                price: Default::default(),
                                compare_price: Default::default(),
                                availability: RwSignal::new(String::new()),
                                included: RwSignal::new(false),
                            };
                            variants.insert(new_variant);
                        }
                    }
                });
            },
            false,
        );
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
        // self.update_variants();
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

    pub fn done_editing_variant(self, key: DefaultKey) {
        // TODO: validate the variant
        self.fields.variants.update(|v| {
            let entry = v.get_mut(key).unwrap();
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
        let mut variants: SlotMap<DefaultKey, VariantEntry> = Default::default();
        for ProductVariant {
            sku,
            options,
            price,
            compare_price,
            stocks,
            ..
        } in product.variants
        {
            let entry = VariantEntry {
                sku: RwSignal::new(sku),
                options: RwSignal::new(options),
                included: RwSignal::new(true),
                price: RwSignal::new(price.map_or(Default::default(), |v| v.to_string())),
                compare_price: RwSignal::new(
                    compare_price.map_or(Default::default(), |v| v.to_string()),
                ),
                availability: RwSignal::new(stocks.to_string()),
            };
            variants.insert(entry);
        }
        self.fields.variants.set(variants);
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
        let variants = self.try_get_variants()?;

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
                variants: Some(variants).filter(|v| *v != product.variants),
                slug: Some(self.fields.slug.get_untracked()).filter(|v| *v != product.slug),
            };
            log::debug!("update model: {:?}", body);
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
        for (_, entry) in self.fields.options.get_untracked() {
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

    fn try_get_variants(&self) -> Result<Vec<ProductVariant>> {
        let mut variants = Vec::new();
        for (_, entry) in self.fields.variants.get_untracked() {
            if !entry.included.get_untracked() {
                continue;
            }
            let sku = entry.sku.get_untracked();
            let price_string = entry.price.get_untracked();
            let price_str = price_string.trim();
            let price = (!price_str.is_empty())
                .then_some(
                    price_str
                        .parse()
                        .map_err(|_| "failed to parse price".to_string()),
                )
                .transpose()?;
            let compare_price_string = entry.compare_price.get_untracked();
            let compare_price_str = compare_price_string.trim();
            let compare_price = (!compare_price_str.is_empty())
                .then_some(
                    compare_price_str
                        .parse()
                        .map_err(|_| "failed to parse compare-at price".to_string()),
                )
                .transpose()?;
            let stocks = entry
                .availability
                .get_untracked()
                .parse()
                .map_err(|_| "failed to parse availability".to_string())?;
            let options = entry.options.get_untracked();
            if sku.is_empty() {
                Err("Variant sku can't be empty".to_string())?;
            }
            variants.push(ProductVariant {
                sku,
                price,
                compare_price,
                stocks,
                options,
                images: Default::default(),
            });
        }
        Ok(variants)
    }
}
