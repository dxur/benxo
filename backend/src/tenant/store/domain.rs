use std::collections::HashMap;

use bigdecimal::BigDecimal;
use bson::{oid::ObjectId, DateTime, Decimal128};
use indexmap::{indexmap, IndexMap, IndexSet};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::id::Id;
use crate::types::name::Name;
use crate::utils::types::CowStr;

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
pub struct MenuItem {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct FeaturedCollection {
    pub label: String,
    pub img: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct FooterItem {
    pub label: String,
    pub url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct FooterList {
    pub title: String,
    pub items: Vec<FooterItem>,
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

    pub logo: Option<String>,
    pub logo_alt: Option<String>,
    pub favicon: Option<String>,

    pub menu_items: Vec<MenuItem>,
    pub featured_collections: Vec<FeaturedCollection>,
    pub social_links: Vec<SocialLink>,
    pub footer_lists: Vec<FooterList>,

    // templates
    pub homepage_template: CowStr,
    pub product_page_template: CowStr,
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
        logo: Option<String>,
        logo_alt: Option<String>,
        favicon: Option<String>,
        menu_items: Vec<MenuItem>,
        featured_collections: Vec<FeaturedCollection>,
        social_links: Vec<SocialLink>,
        footer_lists: Vec<FooterList>,
        google_analytics_id: Option<String>,
        gtm_container_id: Option<String>,
        tracking_pixels: Vec<TrackingPixel>,
        meta_title: Option<String>,
        meta_description: Option<String>,
        meta_keywords: Option<String>,
        custom_key_values: HashMap<String, String>,
    ) -> Self {
        let now = DateTime::now();

        // Default templates
        let homepage_template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/index.liquid"
        ))
        .into();
        let product_page_template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/product.liquid"
        ))
        .into();
        let cart_page_template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/cart.liquid"
        ))
        .into();
        let shop_page_template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/shop.liquid"
        ))
        .into();
        let not_found_page_template = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../templates/404.liquid"
        ))
        .into();
        let custom_pages = Default::default();
        let snippets = indexmap! {
            "style.liquid".into() => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../templates/style.liquid")).into(),
            "header.liquid".into() => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../templates/header.liquid")).into(),
            "footer.liquid".into() => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../templates/footer.liquid")).into(),
            "product-card.liquid".into() => include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../templates/product-card.liquid")).into(),
        };

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

            logo,
            logo_alt,
            favicon,

            menu_items,
            featured_collections,
            social_links,
            footer_lists,

            homepage_template,
            product_page_template,
            cart_page_template,
            shop_page_template,
            not_found_page_template,
            custom_pages,
            snippets,

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
