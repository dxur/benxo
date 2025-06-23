use macros::Model;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreFetch {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreCreate {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct StoreUpdateBody {
    pub id: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreUpdate {
    pub id: String,
    pub body: StoreUpdateBody,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StoreDelete {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct StorePublic {
    pub id: String,
    pub name: String,
}

#[derive(Model)]
#[model(public=StorePublic, fetch=StoreFetch, create=StoreCreate, update=StoreUpdate, delete=StoreDelete)]
pub struct Store;
