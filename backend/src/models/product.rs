use bson::oid::ObjectId;
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
    pub description: String,
    pub featured: bool,
    pub category: String,
    pub base_price: f32,
    pub base_discount: f32,
    pub base_images: Vec<String>,
    pub slug: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductUpdateBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub featured: Option<bool>,
    pub category: Option<String>,
    pub base_price: Option<f32>,
    pub base_discount: Option<f32>,
    pub base_images: Option<Vec<String>>,
    pub slug: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
            && self.base_discount.is_none()
            && self.base_images.is_none()
            && self.slug.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductPublic {
    pub id: ObjectId,
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    pub base_price: f32,
    pub base_discount: f32,
    pub base_images: Vec<String>,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVarFetch {
    pub sku: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarCreate {
    pub sku: String,
    pub product_id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: Option<f32>,
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: Vec<String>,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarUpdateBody {
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f32>,
    pub discount: Option<f32>,
    pub stocks: Option<usize>,
    pub images: Option<Vec<String>>,
    pub attrs: Option<Vec<String>>,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarUpdate {
    pub sku: String,
    pub body: ProductVarUpdateBody,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarPublic {
    pub sku: String,
    pub product_id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: Option<f32>,
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVarDelete {
    pub sku: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductVarFilter {
    pub product_id: ObjectId,
}

pub struct ProductVar;
impl Model for ProductVar {
    type Public = ProductVarPublic;
}
impl Fetchable for ProductVar {
    type Fetch = ProductVarFetch;
}
impl Creatable for ProductVar {
    type Create = ProductVarCreate;
}
impl Updatable for ProductVar {
    type Update = ProductVarUpdate;
}
impl Deletable for ProductVar {
    type Delete = ProductVarDelete;
}
impl Filterable for ProductVar {
    type Filter = ProductVarFilter;
}
