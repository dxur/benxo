use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
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
pub struct OrderModelFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelCreate {
    pub full_name: String,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelUpdate {
    #[serde(skip_serializing)]
    pub id: ObjectId,
    pub status: Option<OrderStatus>,
    pub full_name: Option<String>,
    pub items: Option<Vec<CartItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub full_name: String,
    pub items: Vec<CartItem>,
}

pub struct OrderModel;
impl Model for OrderModel {
    type ModelFetch = OrderModelFetch;
    type ModelCreate = OrderModelCreate;
    type ModelUpdate = OrderModelUpdate;
    type ModelDelete = OrderModelDelete;
    type ModelPublic = OrderModelPublic;
}
