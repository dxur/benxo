use bigdecimal::BigDecimal;
use bson::{oid::ObjectId, DateTime, Decimal128};
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::name::Name;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    #[default]
    Draft,
    Active,
    InActive,
    Archived,
    Deleted,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export)]
pub struct ProductVariant {
    pub sku: String,
    #[ts(as = "String")]
    pub price: BigDecimal,
    #[ts(as = "Option<String>")]
    pub compare_at: Option<BigDecimal>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub options: IndexMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductRecord {
    pub _id: ObjectId,
    pub title: Name,
    pub description: String,
    pub status: ProductStatus,
    pub featured: bool,
    pub category: String,
    pub images: Vec<String>,
    pub options: IndexMap<String, IndexSet<String>>,
    pub variants: Vec<ProductVariant>,
    pub slug: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for ProductRecord {
    fn default() -> Self {
        let now = DateTime::now();

        Self {
            _id: Default::default(),
            title: Default::default(),
            description: Default::default(),
            status: Default::default(),
            featured: Default::default(),
            category: Default::default(),
            images: Default::default(),
            options: Default::default(),
            variants: Default::default(),
            slug: Default::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl ProductRecord {
    pub fn is_active(&self) -> bool {
        matches!(self.status, ProductStatus::Active)
    }

    pub fn get_variant_by_sku(&self, sku: &str) -> Option<&ProductVariant> {
        self.variants.iter().find(|v| v.sku == sku)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ProductFilter {
    pub status: Option<ProductStatus>,
    pub category: Option<String>,
    pub featured: Option<bool>,
    pub search: Option<String>,
}
