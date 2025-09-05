use std::collections::HashMap;

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

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct SocialLink {
    pub platform: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct TrackingPixel {
    pub platform: String,
    pub pixel_id: String,
    pub events: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StoreRecord {
    pub _id: ObjectId,
    pub name: Name,
    pub description: String,
    pub status: StoreStatus,

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

    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl StoreRecord {
    pub fn new(
        name: Name,
        description: String,
        status: StoreStatus,
        category: Option<String>,
        contact_email: Option<String>,
        contact_phone: Option<String>,
        address: Option<String>,
        city: Option<String>,
        zip_code: Option<String>,
        social_links: Vec<SocialLink>,
        selected_theme: Option<String>,
        color_scheme: Option<String>,
        header_style: Option<String>,
        google_analytics_id: Option<String>,
        gtm_container_id: Option<String>,
        tracking_pixels: Vec<TrackingPixel>,
        meta_title: Option<String>,
        meta_description: Option<String>,
        meta_keywords: Option<String>,
        custom_key_values: HashMap<String, String>,
    ) -> Self {
        let now = DateTime::now();

        Self {
            _id: Default::default(),
            name,
            description,
            status,
            category,
            contact_email,
            contact_phone,
            address,
            city,
            zip_code,
            social_links,
            selected_theme,
            color_scheme,
            header_style,
            google_analytics_id,
            gtm_container_id,
            tracking_pixels,
            meta_title,
            meta_description,
            meta_keywords,
            custom_key_values,
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
