use backend::api::{ApiRoutes, Routes};
use backend::models::order::*;
use backend::models::{ObjectId, Page, Pagination};
use indexmap::IndexMap;
use leptos::{html::*, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use std::str::FromStr;

use crate::notifications::{error, success};
use crate::routes::*;
use crate::utils::*;

#[derive(Clone, Copy, Default)]
pub struct CreateFields {
    pub full_name: RwSignal<String>,
    pub phone: RwSignal<String>,
    pub email: RwSignal<String>,
    pub province: RwSignal<String>,
    pub address: RwSignal<String>,
    pub note: RwSignal<String>,
    pub delivery: RwSignal<DeliveryType>,
}

#[derive(Clone, Copy)]
pub struct IndexState {
    pub orders: RwSignal<Page<OrderPublic>>,
    pub status: RwSignal<LoadingStatus>,
    pub page: RwSignal<usize>,
    pub total: RwSignal<usize>,
    pub dialog: NodeRef<Dialog>,
    pub fields: CreateFields,
}

impl IndexState {
    pub fn new() -> Self {
        let state = Self {
            orders: Default::default(),
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
            let res = ApiRoutes::get_all_orders(Pagination {
                page: Some(self.page.get_untracked()),
                per_page: None,
            })
            .await
            .map_err(|e| e.to_string());
            log::debug!("Fetched orders: {:?}", res);
            match res {
                Ok(data) => {
                    self.total.set(data.total_pages());
                    self.orders.set(data);
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
        log::debug!("Into order: {:?}", res);
        match res {
            Ok(order) => spawn_local(async move {
                let res = ApiRoutes::create_order(order).await;
                log::debug!("Created order: {:?}", res);
                match res {
                    Ok(order) => {
                        success("Product created");
                        Self::edit(order.id);
                    }
                    Err(e) => error(e),
                }
            }),
            Err(e) => error(e),
        }
    }

    pub fn edit(id: ObjectId) {
        navigate(
            AppRoutes::ORDER
                .path()
                .replace(":id", &id.to_hex())
                .as_str(),
            Default::default(),
        );
    }

    fn try_create(&self) -> Result<OrderCreate> {
        Ok(OrderCreate {
            full_name: self.fields.full_name.get_untracked(),
            phone: self.fields.phone.get_untracked(),
            email: self.fields.email.get_untracked(),
            province: self.fields.province.get_untracked(),
            address: self.fields.address.get_untracked(),
            delivery: self.fields.delivery.get_untracked(),
            note: self.fields.note.get_untracked(),
            items: Default::default(),
        })
    }
}

#[derive(Clone)]
pub struct CartItemFields {
    pub product_sku: String,
    pub quantity: RwSignal<String>,
    pub price: RwSignal<String>,
}

#[derive(Clone, Copy, Default)]
pub struct Fields {
    pub full_name: RwSignal<String>,
    pub phone: RwSignal<String>,
    pub email: RwSignal<String>,
    pub province: RwSignal<String>,
    pub address: RwSignal<String>,
    pub note: RwSignal<String>,
    pub delivery: RwSignal<DeliveryType>,
    pub status: RwSignal<OrderStatus>,
    pub progress: RwSignal<Option<OrderStatus>>,
    pub regress: RwSignal<Option<OrderStatus>>,
    pub items: RwSignal<Vec<CartItemFields>>,
    pub history: RwSignal<Vec<OrderHistoryEntry>>,
    pub item_quantity: RwSignal<String>,
}

#[derive(Clone, Copy)]
pub struct EditState {
    pub id: Option<ObjectId>,
    pub status: RwSignal<LoadingStatus>,
    pub order: RwSignal<Option<OrderPublic>>,
    pub fields: Fields,
    pub dialog: NodeRef<Dialog>,
    pub search: RwSignal<String>,
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
            order: Default::default(),
            fields: Default::default(),
            dialog: Default::default(),
            search: Default::default(),
        };
        state.fetch();
        state
    }

    pub fn fetch(self) {
        if let Some(id) = self.id {
            spawn_local(async move {
                let res = ApiRoutes::get_one_order(OrderFetch { id }).await;
                log::debug!("Fetched order: {:?}", res);
                match res {
                    Ok(order) => {
                        self.update_fields(order.clone());
                        self.order.set(Some(order));
                        self.status.set(LoadingStatus::Ok);
                    }
                    Err(e) => {
                        self.status.set(LoadingStatus::Err(e.to_string()));
                    }
                }
            });
        }
    }

    pub fn delete(self) {
        if let Some(id) = self.id {
            spawn_local(async move {
                let res = ApiRoutes::delete_order(OrderDelete { id }).await;
                log::debug!("Deleted order: {:?}", res);
                match res {
                    Ok(_) => {
                        success("Order deleted");
                        navigate(AppRoutes::ORDERS.path(), Default::default());
                    }
                    Err(e) => error(e),
                }
            });
        } else {
            error("Can't delete order. unknown id");
        }
    }

    pub fn progress(self) {
        match (self.id, self.fields.progress.get()) {
            (Some(id), Some(status)) => {
                let update = OrderUpdateBody {
                    status: Some(status),
                    full_name: None,
                    phone: None,
                    email: None,
                    province: None,
                    address: None,
                    delivery: None,
                    note: None,
                    items: None,
                };
                spawn_local(async move {
                    let res = ApiRoutes::update_order(OrderUpdate { id, body: update }).await;
                    log::debug!("Updated order: {:?}", res);
                    match res {
                        Ok(order) => {
                            success("Order updated");
                            self.update_fields(order);
                        }
                        Err(e) => error(e),
                    }
                });
            }
            _ => {
                error("Can't delete order. unknown id");
            }
        }
    }

    pub fn regress(self) {
        match (self.id, self.fields.regress.get()) {
            (Some(id), Some(status)) => {
                let update = OrderUpdateBody {
                    status: Some(status),
                    full_name: None,
                    phone: None,
                    email: None,
                    province: None,
                    address: None,
                    delivery: None,
                    note: None,
                    items: None,
                };
                spawn_local(async move {
                    let res = ApiRoutes::update_order(OrderUpdate { id, body: update }).await;
                    log::debug!("Updated order: {:?}", res);
                    match res {
                        Ok(order) => {
                            success("Order updated");
                            self.update_fields(order);
                        }
                        Err(e) => error(e),
                    }
                });
            }
            _ => {
                error("Can't delete order. unknown id");
            }
        }
    }

    pub fn update(self) {
        let res = self.try_update();
        log::debug!("Into product: {:?}", res);
        match res {
            Ok(order) => spawn_local(async move {
                let res = ApiRoutes::update_order(order).await;
                log::debug!("Update product: {:?}", res);
                match res {
                    Ok(ord) => {
                        success("Product updated");
                        self.update_fields(ord.clone());
                        self.order.set(Some(ord));
                    }
                    Err(e) => error(e),
                }
            }),
            Err(e) => error(e),
        }
    }

    pub fn remove_item(self, product_sku: &str) {
        self.fields.items.update(|items| {
            items.retain(|item| item.product_sku != product_sku);
        });
    }

    pub fn add_new_item(self) {
        self.dialog.get().map(|d| d.show_modal());
    }

    pub fn add_item(self) {
        todo!()
    }

    fn update_fields(&self, order: OrderPublic) {
        self.fields.progress.set(match order.status {
            OrderStatus::Pending => Some(OrderStatus::Confirmed),
            OrderStatus::Confirmed => Some(OrderStatus::Delivered),
            OrderStatus::Delivered => Some(OrderStatus::Done),
            OrderStatus::Done => None,
            OrderStatus::Canceled => Some(OrderStatus::Pending),
            OrderStatus::Returned => Some(OrderStatus::Delivered),
        });

        self.fields.regress.set(match order.status {
            OrderStatus::Pending => Some(OrderStatus::Canceled),
            OrderStatus::Confirmed => Some(OrderStatus::Canceled),
            OrderStatus::Delivered => Some(OrderStatus::Returned),
            OrderStatus::Done => Some(OrderStatus::Canceled),
            OrderStatus::Canceled => None,
            OrderStatus::Returned => Some(OrderStatus::Canceled),
        });

        self.fields.full_name.set(order.full_name);
        self.fields.phone.set(order.phone);
        self.fields.email.set(order.email);
        self.fields.province.set(order.province);
        self.fields.address.set(order.address);
        self.fields.note.set(order.note);
        self.fields.delivery.set(order.delivery);
        self.fields.status.set(order.status);
        self.fields.history.set(order.history);
        self.fields.items.set(
            order
                .items
                .into_iter()
                .map(|(sku, item)| CartItemFields {
                    product_sku: sku,
                    quantity: RwSignal::new(item.quantity.to_string()),
                    price: RwSignal::new(item.price.to_string()),
                })
                .collect(),
        );
    }

    fn try_update(&self) -> Result<OrderUpdate> {
        if let Some(order) = self.order.get_untracked() {
            let items = self
                .fields
                .items
                .get_untracked()
                .into_iter()
                .map(|v| {
                    let quantity = v
                        .price
                        .get_untracked()
                        .parse()
                        .map_err(|_| "failed to parse base price".to_string())?;
                    let price = v
                        .price
                        .get_untracked()
                        .parse()
                        .map_err(|_| "failed to parse base price".to_string())?;
                    Ok((v.product_sku, CartItem { quantity, price }))
                })
                .collect::<Result<_>>()?;

            let body = OrderUpdateBody {
                status: None,
                full_name: Some(self.fields.full_name.get_untracked())
                    .filter(|v| *v != order.full_name),
                phone: Some(self.fields.phone.get_untracked()).filter(|v| *v != order.phone),
                email: Some(self.fields.email.get_untracked()).filter(|v| *v != order.email),
                province: Some(self.fields.province.get_untracked())
                    .filter(|v| *v != order.province),
                address: Some(self.fields.address.get_untracked()).filter(|v| *v != order.address),
                delivery: Some(self.fields.delivery.get_untracked())
                    .filter(|v| *v != order.delivery),
                note: Some(self.fields.note.get_untracked()).filter(|v| *v != order.note),
                items: Some(items).filter(|v| *v != order.items),
            };
            if body.is_none() {
                Err("Nothing Have been Updated".to_string())
            } else {
                Ok(OrderUpdate { id: order.id, body })
            }
        } else {
            Err("Can't read the order".to_string())
        }
    }
}
