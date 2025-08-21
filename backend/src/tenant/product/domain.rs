use bigdecimal::BigDecimal;
use bson::{oid::ObjectId, DateTime, Decimal128};
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::name::Name;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ProductStatus {
    Active,
    #[default]
    Inactive,
    Archived,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
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
    pub variants: Vec<ProductVariant>,
    pub slug: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl ProductRecord {
    pub fn new(
        title: Name,
        description: String,
        status: ProductStatus,
        featured: bool,
        category: String,
        images: Vec<String>,
        variants: Vec<ProductVariant>,
        slug: String,
    ) -> Self {
        let now = DateTime::now();

        Self {
            _id: Default::default(),
            title,
            description,
            status,
            featured,
            category,
            images,
            variants,
            slug,
            created_at: now,
            updated_at: now,
        }
    }

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
