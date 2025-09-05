use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use indexmap::{IndexMap, IndexSet};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::domain::*;
use crate::{
    types::{id::Id, name::Name},
    utils::serde_helpers::JsonOption,
};

#[derive(Debug, Clone, Deserialize, Serialize, o2o, TS)]
#[serde(rename_all = "snake_case")]
#[map_owned(ProductStatus)]
#[ts(export)]
pub enum ProductStatusDto {
    Active,
    Inactive,
    Archived,
}

#[derive(Debug, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export, bound = "")]
pub struct ProductCreateDto {
    pub title: Name,
    pub description: String,
    pub status: ProductStatusDto,
    pub featured: bool,
    pub category: String,
    pub images: Vec<String>,
    pub variants: Vec<ProductVariant>,
    pub slug: String,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export, bound = "")]
#[from_owned(ProductRecord)]
pub struct ProductDto {
    #[from(@._id.into())]
    pub id: Id,
    pub title: Name,
    pub description: String,
    #[from(~.into())]
    pub status: ProductStatusDto,
    pub featured: bool,
    pub category: String,
    pub images: Vec<String>,
    pub variants: Vec<ProductVariant>,
    pub slug: String,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, optional_fields)]
pub struct ProductListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<ProductStatusDto>,
    pub category: Option<String>,
    pub featured: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct ProductUpdate {
    pub title: JsonOption<Name>,
    pub description: JsonOption<String>,
    pub category: JsonOption<String>,
    pub images: JsonOption<Vec<String>>,
    pub featured: JsonOption<bool>,
    pub status: JsonOption<ProductStatusDto>,
    pub variants: JsonOption<Vec<ProductVariant>>,
    pub slug: JsonOption<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct ProductListResponse {
    pub products: Vec<ProductDto>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}
