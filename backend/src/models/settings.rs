use bson::oid::ObjectId;
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

pub struct Settings;
impl Model for Settings {
    type Public = SettingsPublic;
}

impl Fetchable for Settings {
    type Fetch = Void;
}

impl Updatable for Settings {
    type Update = SettingsUpdate;
}
