use backend::api::{ApiRoutes, Routes};
use backend::models::order::*;
use backend::models::{ObjectId, Page, Pagination};
use leptos::{html::*, prelude::*, task::spawn_local};
use leptos_router::hooks::use_params_map;
use slotmap::{DefaultKey, SlotMap};
use std::str::FromStr;

use crate::notifications::{error, success};
use crate::routes::*;
use crate::utils::*;

#[derive(Clone, Copy)]
pub struct IndexState {
    pub orders: RwSignal<Page<OrderPublic>>,
    pub status: RwSignal<LoadingStatus>,
    pub page: RwSignal<usize>,
    pub total: RwSignal<usize>,
    pub dialog: NodeRef<Dialog>,
}

impl IndexState {
    pub fn new() -> Self {
        let state = Self {
            orders: Default::default(),
            status: Default::default(),
            page: RwSignal::new(1),
            total: Default::default(),
            dialog: Default::default(),
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

    pub fn view(id: ObjectId) {
        navigate(
            AppRoutes::ORDER
                .path()
                .replace(":id", &id.to_hex())
                .as_str(),
            Default::default(),
        );
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
}

#[derive(Clone, Copy)]
pub struct EditState {
    pub id: Option<ObjectId>,
    pub status: RwSignal<LoadingStatus>,
    pub order: RwSignal<Option<OrderPublic>>,
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
            order: Default::default(),
            fields: Default::default(),
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
                    let res = ApiRoutes::update_order(OrderUpdate {
                        id,
                        body: update,
                    })
                    .await;
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
                    let res = ApiRoutes::update_order(OrderUpdate {
                        id,
                        body: update,
                    })
                    .await;
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

    pub fn remove_item(self, product_sku: &str) {
        self.fields.items.update(|items| {
            items.retain(|item| item.product_sku != product_sku);
        });
    }

    fn update_fields(&self, order: OrderPublic) {
        self.fields.progress.set(match order.status {
            OrderStatus::Pending => Some(OrderStatus::Confirmed),
            OrderStatus::Confirmed => Some(OrderStatus::Delivered),
            OrderStatus::Delivered => Some(OrderStatus::Done),
            OrderStatus::Done => None,
            OrderStatus::Rejected => Some(OrderStatus::Pending),
            OrderStatus::Returned => Some(OrderStatus::Delivered),
        });

        self.fields.regress.set(match order.status {
            OrderStatus::Pending => Some(OrderStatus::Rejected),
            OrderStatus::Confirmed => Some(OrderStatus::Rejected),
            OrderStatus::Delivered => Some(OrderStatus::Returned),
            OrderStatus::Done => Some(OrderStatus::Returned),
            OrderStatus::Rejected => None,
            OrderStatus::Returned => None,
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
}
