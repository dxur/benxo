use std::collections::HashMap;

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
pub enum StoreStatusDto {
    Active,
    Inactive,
    Archived,
}

// TODO: into StoreRecord
#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, bound = "")]
pub struct StoreCreateDto {
    pub name: Name,
    pub description: String,
    pub status: StoreStatusDto,

    pub category: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,

    pub social_links: Vec<SocialLink>,

    pub selected_theme: Option<String>,
    pub color_scheme: Option<String>,
    pub header_style: Option<String>,

    pub google_analytics_id: Option<String>,
    pub gtm_container_id: Option<String>,

    pub tracking_pixels: Vec<TrackingPixel>,

    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,

    pub custom_key_values: HashMap<String, String>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[from_owned(StoreRecord)]
#[ts(export, bound = "")]
pub struct StoreDto {
    #[from(@._id.into())]
    pub id: Id,
    pub name: Name,
    pub description: String,
    #[from(~.into())]
    pub status: StoreStatusDto,

    pub category: Option<String>,
    pub contact_email: Option<String>,
    pub contact_phone: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub zip_code: Option<String>,

    pub social_links: Vec<SocialLink>,

    pub selected_theme: Option<String>,
    pub color_scheme: Option<String>,
    pub header_style: Option<String>,

    pub google_analytics_id: Option<String>,
    pub gtm_container_id: Option<String>,

    pub tracking_pixels: Vec<TrackingPixel>,

    pub meta_title: Option<String>,
    pub meta_description: Option<String>,
    pub meta_keywords: Option<String>,

    pub custom_key_values: HashMap<String, String>,

    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, optional_fields)]
pub struct StoreListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<StoreStatusDto>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct StoreUpdate {
    pub name: JsonOption<Name>,
    pub description: JsonOption<String>,
    pub status: JsonOption<StoreStatusDto>,
    pub slug: JsonOption<String>,

    pub category: JsonOption<String>,
    pub contact_email: JsonOption<String>,
    pub contact_phone: JsonOption<String>,
    pub address: JsonOption<String>,
    pub city: JsonOption<String>,
    pub zip_code: JsonOption<String>,

    pub social_links: JsonOption<Vec<SocialLink>>,

    pub selected_theme: JsonOption<String>,
    pub color_scheme: JsonOption<String>,
    pub header_style: JsonOption<String>,

    pub google_analytics_id: JsonOption<String>,
    pub gtm_container_id: JsonOption<String>,

    pub tracking_pixels: JsonOption<Vec<TrackingPixel>>,

    pub meta_title: JsonOption<String>,
    pub meta_description: JsonOption<String>,
    pub meta_keywords: JsonOption<String>,

    pub custom_key_values: JsonOption<HashMap<String, String>>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct StoreListResponse {
    pub stores: Vec<StoreDto>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export, bound = "")]
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
#[ts(export, bound = "")]
pub struct StoreRegUpdate {
    pub slug: String,
    pub domain: JsonOption<String>,
}
