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
#[ghosts(Deleted: Self::Archived)]
#[ts(export)]
pub enum ProductStatusDto {
    Draft,
    Active,
    InActive,
    Archived,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ProductVariantCreate {
    pub sku: String,
    #[ts(as = "String")]
    pub price: BigDecimal,
    #[ts(as = "Option<String>")]
    pub compare_at: Option<BigDecimal>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub options: IndexMap<String, String>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export)]
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
    pub options: IndexMap<String, IndexSet<String>>,
    pub variants: Vec<ProductVariant>,
    pub slug: String,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct ProductListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<ProductStatusDto>,
    pub category: Option<String>,
    pub featured: Option<bool>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct ProductUpdate {
    pub title: JsonOption<Name>,
    pub description: JsonOption<String>,
    pub category: JsonOption<String>,
    pub images: JsonOption<Vec<String>>,
    pub featured: JsonOption<bool>,
    pub status: JsonOption<ProductStatusDto>,
    pub options: JsonOption<IndexMap<String, IndexSet<String>>>,
    pub variants: JsonOption<Vec<ProductVariant>>,
    pub slug: JsonOption<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct ProductListResponse {
    pub products: Vec<ProductDto>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}
