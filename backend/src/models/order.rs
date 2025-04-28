use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Rejected,
    Delivered,
    Done,
    Returned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub product_sku: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCreate {
    pub full_name: String,
    pub items: Vec<CartItem>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct OrderUpdateBody {
    pub status: Option<OrderStatus>,
    pub full_name: Option<String>,
    pub items: Option<Vec<CartItem>>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct OrderUpdate {
    pub id: ObjectId,
    pub body: OrderUpdateBody
}

impl OrderUpdateBody {
    pub fn is_none(&self) -> bool {
        self.full_name.is_none()
            && self.status.is_none()
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
    pub items: Vec<CartItem>,
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
