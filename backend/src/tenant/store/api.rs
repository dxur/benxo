use std::collections::HashMap;

use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use indexmap::{IndexMap, IndexSet};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::domain::*;
use crate::types::{id::Id, name::Name};
use crate::utils::serde_helpers::JsonOption;
use crate::utils::types::CowStr;

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

    pub logo: Option<String>,
    pub logo_alt: Option<String>,
    pub favicon: Option<String>,

    pub menu_items: Vec<MenuItem>,
    pub featured_collections: IndexSet<String>,
    pub social_links: Vec<SocialLink>,

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

    pub logo: Option<String>,
    pub logo_alt: Option<String>,
    pub favicon: Option<String>,

    pub menu_items: Vec<MenuItem>,
    pub featured_collections: IndexSet<String>,
    pub social_links: Vec<SocialLink>,

    pub homepage_template: CowStr,
    pub product_page_template: CowStr,
    pub collection_page_template: CowStr,
    pub cart_page_template: CowStr,
    pub shop_page_template: CowStr,
    pub not_found_page_template: CowStr,
    pub custom_pages: IndexMap<String, CowStr>,
    pub snippets: IndexMap<String, CowStr>,

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

#[macros::json_option_serde]
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

    pub logo: JsonOption<String>,
    pub logo_alt: JsonOption<String>,
    pub favicon: JsonOption<String>,

    pub menu_items: JsonOption<Vec<MenuItem>>,
    pub featured_collections: JsonOption<IndexSet<String>>,
    pub social_links: JsonOption<Vec<SocialLink>>,

    pub homepage_template: JsonOption<CowStr>,
    pub product_page_template: JsonOption<CowStr>,
    pub collection_page_template: JsonOption<CowStr>,
    pub cart_page_template: JsonOption<CowStr>,
    pub shop_page_template: JsonOption<CowStr>,
    pub not_found_page_template: JsonOption<CowStr>,
    pub custom_pages: JsonOption<IndexMap<String, CowStr>>,
    pub snippets: JsonOption<IndexMap<String, CowStr>>,

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

#[macros::json_option_serde]
#[derive(Debug, Deserialize, TS)]
#[ts(export, bound = "")]
pub struct StoreRegUpdate {
    pub slug: String,
    pub domain: JsonOption<String>,
}
