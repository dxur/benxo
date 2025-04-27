use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::*;
use crate::validators::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryCreate {
    pub id: ObjectId,
    pub name: String,
    #[serde(deserialize_with = "non_negative")]
    pub tax: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
    #[serde(deserialize_with = "non_negative_option")]
    pub tax: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryPublic {
    pub id: ObjectId,
    pub name: String,
    #[serde(deserialize_with = "non_negative")]
    pub tax: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryDelete {
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
