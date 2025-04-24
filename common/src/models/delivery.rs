use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::Model;
use crate::utils::validators::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryModelFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryModelCreate {
    pub id: ObjectId,
    pub name: String,
    #[serde(deserialize_with = "non_negative")]
    pub tax: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryModelUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
    #[serde(deserialize_with = "non_negative_option")]
    pub tax: Option<f32>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryModelPublic {
    pub id: ObjectId,
    pub name: String,
    #[serde(deserialize_with = "non_negative")]
    pub tax: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeliveryModelDelete {
    pub id: ObjectId,
}

pub struct DeliveryModel;
impl Model for DeliveryModel {
    type ModelFetch = DeliveryModelFetch;
    type ModelCreate = DeliveryModelCreate;
    type ModelUpdate = DeliveryModelUpdate;
    type ModelDelete = DeliveryModelDelete;
    type ModelPublic = DeliveryModelPublic;
}
