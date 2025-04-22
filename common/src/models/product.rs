use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use crate::utils::validators::*;

use super::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModelFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModelCreate {
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    #[serde(deserialize_with = "non_negative")]
    pub base_price: f32,
    #[serde(deserialize_with = "non_negative")]
    pub base_discount: f32,
    pub base_images: Vec<String>,
    pub slug: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModelUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
    pub description: Option<String>,
    pub featured: Option<bool>,
    pub category: Option<String>,
    #[serde(deserialize_with = "non_negative_option")]
    pub base_price: Option<f32>,
    #[serde(deserialize_with = "non_negative_option")]
    pub base_discount: Option<f32>,
    pub base_images: Option<Vec<String>>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModelDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    #[serde(deserialize_with = "non_negative")]
    pub base_price: f32,
    #[serde(deserialize_with = "non_negative")]
    pub base_discount: f32,
    pub base_images: Vec<String>,
    pub slug: String,
}

pub struct ProductModel;
impl Model for ProductModel {
    type ModelFetch = ProductModelFetch;
    type ModelCreate = ProductModelCreate;
    type ModelUpdate = ProductModelUpdate;
    type ModelDelete = ProductModelDelete;
    type ModelPublic = ProductModelPublic;
}
