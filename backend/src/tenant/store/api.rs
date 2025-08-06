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
#[map_owned(StoreStatus)]
#[ghosts(Deleted: Self::Archived)]
#[ts(export)]
pub enum StoreStatusDto {
    Draft,
    Active,
    InActive,
    Archived,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export)]
#[from_owned(StoreRecord)]
pub struct StoreDto {
    #[from(@._id.into())]
    pub id: Id,
    pub name: Name,
    pub description: String,
    #[from(~.into())]
    pub status: StoreStatusDto,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct StoreListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<StoreStatusDto>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct StoreUpdate {
    pub name: JsonOption<Name>,
    pub description: JsonOption<String>,
    pub status: JsonOption<StoreStatusDto>,
    pub slug: JsonOption<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct StoreListResponse {
    pub stores: Vec<StoreDto>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export)]
#[from_owned(StoreRegRecord)]
pub struct StoreRegDto {
    #[from(~.into())]
    pub business_id: Id,
    #[from(~.into())]
    pub store_id: Id,
    pub slug: String,
    pub domain: Option<String>,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct StoreRegUpdate {
    pub slug: String,
    pub domain: JsonOption<String>,
}
