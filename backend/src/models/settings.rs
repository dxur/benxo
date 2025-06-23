use bson::oid::ObjectId;
use macros::Model;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsUpdate {
    pub name: Option<String>,
    pub domain: Option<String>,
    pub active_theme: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct SettingsPublic {
    pub name: String,
    pub description: String,
    pub phone: String,
    pub email: String,
    pub domain: String,
}

#[derive(Model)]
#[model(public=SettingsPublic, fetch=Void, update=SettingsUpdate)]
pub struct Settings;
