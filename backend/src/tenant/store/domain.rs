use bigdecimal::BigDecimal;
use bson::{oid::ObjectId, DateTime, Decimal128};
use indexmap::{IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::id::Id;
use crate::types::name::Name;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum StoreStatus {
    Active,
    #[default]
    Inactive,
    Archived,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoreRecord {
    pub _id: ObjectId,
    pub name: Name,
    pub description: String,
    pub status: StoreStatus,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl StoreRecord {
    pub fn new(name: Name, description: String, status: StoreStatus) -> Self {
        let now = DateTime::now();
    
        Self {
            _id: Default::default(),
            name,
            description,
            status,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_active(&self) -> bool {
        matches!(self.status, StoreStatus::Active)
    }
}

#[derive(Debug, Clone, Default)]
pub struct StoreFilter {
    pub status: Option<StoreStatus>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoreRegRecord {
    pub slug: String,
    pub domain: Option<String>,
    pub business_id: ObjectId,
    pub store_id: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl StoreRegRecord {
    pub fn new(
        business_id: ObjectId,
        store_id: ObjectId,
        slug: String,
        domain: Option<String>,
    ) -> Self {
        let now = DateTime::now();
        Self {
            slug,
            domain,
            business_id,
            store_id,
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StoreRegFilter {
    pub business_id: Id,
    pub store_id: Option<Id>,
}
