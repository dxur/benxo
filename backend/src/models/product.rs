use bson::oid::ObjectId;
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductCreate {
    pub name: String,
    pub category: String,
    pub slug: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
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
            && self.slug.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVariant {
    pub sku: String,
    pub price: Option<f32>,
    pub compare_price: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub options: IndexMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductPublic {
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

pub struct Product;
impl Model for Product {
    type Public = ProductPublic;
}
impl Fetchable for Product {
    type Fetch = ProductFetch;
}
impl Creatable for Product {
    type Create = ProductCreate;
}
impl Updatable for Product {
    type Update = ProductUpdate;
}
impl Deletable for Product {
    type Delete = ProductDelete;
}
