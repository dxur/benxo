use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use indexmap::{IndexMap, IndexSet};
use macros::Model;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use ts_rs::TS;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
#[serde(rename_all = "snake_case")]
pub enum ProductFetch {
    Id(ObjectId),
    Slug(String),
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProductCreate {
    pub name: String,
    pub category: String,
    pub slug: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
pub struct ProductUpdateBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub featured: Option<bool>,
    pub category: Option<String>,
    pub base_price: Option<f32>,
    pub base_compare_price: Option<f32>,
    pub base_images: Option<Vec<String>>,
    pub options: Option<IndexMap<String, IndexSet<String>>>,
    pub variants: Option<Vec<ProductVariant>>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct ProductUpdate {
    pub id: ObjectId,
    pub body: ProductUpdateBody,
}

impl ProductUpdateBody {
    pub fn is_none(&self) -> bool {
        self.name.is_none()
            && self.description.is_none()
            && self.featured.is_none()
            && self.category.is_none()
            && self.base_price.is_none()
            && self.base_compare_price.is_none()
            && self.base_images.is_none()
            && self.options.is_none()
            && self.variants.is_none()
            && self.slug.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProductDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, TS)]
pub struct ProductVariant {
    pub sku: String,
    pub price: Option<f32>,
    pub compare_price: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub options: IndexMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct ProductPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    pub base_price: f32,
    pub base_compare_price: f32,
    pub base_images: Vec<String>,
    pub options: IndexMap<String, IndexSet<String>>,
    pub variants: Vec<ProductVariant>,
    pub slug: String,
}
