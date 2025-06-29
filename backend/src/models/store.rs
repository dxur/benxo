use hex_color::HexColor;
use macros::Model;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreFetch {
    pub store_id: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreCreate {
    pub store_id: String,
    pub name: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, TS)]
pub struct StoreUpdateBody {
    pub store_id: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    #[ts(as = "Option<String>")]
    pub primary_color: Option<HexColor>,
    #[ts(as = "Option<String>")]
    pub secondary_color: Option<HexColor>,
    #[ts(as = "Option<String>")]
    pub background_color: Option<HexColor>,
    pub head: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreUpdate {
    pub store_id: String,
    pub body: StoreUpdateBody,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreDelete {
    pub store_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct StorePublic {
    pub store_id: String,
    pub name: String,
    pub description: String,
    #[ts(as = "String")]
    pub primary_color: HexColor,
    #[ts(as = "String")]
    pub secondary_color: HexColor,
    #[ts(as = "String")]
    pub background_color: HexColor,
    pub head: String,
}

#[derive(Model)]
#[model(public=StorePublic, fetch=StoreFetch, create=StoreCreate, update=StoreUpdate, delete=StoreDelete)]
pub struct Store;
