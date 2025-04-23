use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use crate::utils::validators::*;

use super::{Model, ModelFilter};

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

impl ProductModelUpdate {
    pub fn is_none(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.featured.is_none()
            && self.category.is_none()
            && self.base_price.is_none()
            && self.base_discount.is_none()
            && self.base_images.is_none()
            && self.slug.is_none()
    }
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVarModelFetch {
    pub sku: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarModelCreate {
    pub sku: String,
    pub product_id: ObjectId,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "non_negative_option")]
    pub price: Option<f32>,
    #[serde(deserialize_with = "non_negative_option")]
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarModelUpdate {
    pub sku: String,
    pub name: Option<String>,
    pub description: Option<String>,
    #[serde(deserialize_with = "non_negative_option")]
    pub price: Option<f32>,
    #[serde(deserialize_with = "non_negative_option")]
    pub discount: Option<f32>,
    pub stocks: Option<usize>,
    pub images: Option<Vec<String>>,
    pub attrs: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarModelPublic {
    pub sku: String,
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub product_id: ObjectId,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "non_negative_option")]
    pub price: Option<f32>,
    #[serde(deserialize_with = "non_negative_option")]
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVarModelDelete {
    pub sku: String,
}

pub struct ProductVarModel;
impl Model for ProductVarModel {
    type ModelFetch = ProductVarModelFetch;
    type ModelCreate = ProductVarModelCreate;
    type ModelUpdate = ProductVarModelUpdate;
    type ModelDelete = ProductVarModelDelete;
    type ModelPublic = ProductVarModelPublic;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVarModelFilter {
    pub product_id: ObjectId,
}

impl ModelFilter for ProductVarModel {
    type ModelFilter = ProductVarModelFilter;
}
