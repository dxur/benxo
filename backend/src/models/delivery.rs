use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryCreate {
    pub id: ObjectId,
    pub name: String,
    pub tax: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
    pub tax: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryPublic {
    pub id: ObjectId,
    pub name: String,
    pub tax: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDelete {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
}

pub struct Delivery;
impl Model for Delivery {
    type Public = DeliveryPublic;
}
impl Fetchable for Delivery {
    type Fetch = DeliveryFetch;
}
impl Creatable for Delivery {
    type Create = DeliveryCreate;
}
impl Updatable for Delivery {
    type Update = DeliveryUpdate;
}
impl Deletable for Delivery {
    type Delete = DeliveryDelete;
}
