use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsUpdate {
    store_name: Option<String>,
    store_domain: Option<String>,
    active_theme: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsPublic {
    store_name: String,
    store_domain: String,
}

pub struct Settings;
impl Model for Settings {
    type Public = SettingsPublic;
}
impl Updatable for Settings {
    type Update = SettingsUpdate;
}
