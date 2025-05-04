use bson::oid::ObjectId;
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use strum::{EnumString, VariantNames, Display};

use super::*;

#[derive(Debug, Serialize, Deserialize, EnumString, Display, VariantNames, PartialEq, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DeliveryType {
    #[default]
    StopDesk,
    Domicil,
    Other,
}

#[derive(Debug, Serialize, Deserialize, EnumString, Display, VariantNames, PartialEq, Clone, Copy, Default)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    #[default]
    Pending,
    Confirmed,
    Rejected,
    Delivered,
    Done,
    Returned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub quantity: u32,
    pub price: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCreate {
    pub full_name: String,
    pub phone: String,
    pub email: String,
    pub province: String,
    pub address: String,
    pub delivery: DeliveryType,
    pub note: String,
    pub items: IndexMap<String, CartItem>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderUpdateBody {
    pub status: Option<OrderStatus>,
    pub full_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub province: Option<String>,
    pub address: Option<String>,
    pub delivery: Option<DeliveryType>,
    pub note: Option<String>,
    pub items: Option<IndexMap<String, CartItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub id: ObjectId,
    pub body: OrderUpdateBody,
}

impl OrderUpdateBody {
    pub fn is_none(&self) -> bool {
        self.status.is_none()
            && self.full_name.is_none()
            && self.phone.is_none()
            && self.email.is_none()
            && self.province.is_none()
            && self.address.is_none()
            && self.delivery.is_none()
            && self.note.is_none()
            && self.items.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderPublic {
    pub id: ObjectId,
    pub status: OrderStatus,
    pub full_name: String,
    pub phone: String,
    pub email: String,
    pub province: String,
    pub address: String,
    pub delivery: DeliveryType,
    pub note: String,
    pub items: IndexMap<String, CartItem>,
}

pub struct Order;
impl Model for Order {
    type Public = OrderPublic;
}
impl Fetchable for Order {
    type Fetch = OrderFetch;
}
impl Creatable for Order {
    type Create = OrderCreate;
}
impl Updatable for Order {
    type Update = OrderUpdate;
}
impl Deletable for Order {
    type Delete = OrderDelete;
}
