use crate::models::settings::*;
use field::*;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::Model;
use crate::{db::Db, events::Event};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsInDb {
    pub store_id: ObjectId,
    pub store_name: String,
    pub store_domain: String,
    pub active_theme: ObjectId,
}

impl Into<SettingsPublic> for SettingsInDb {
    fn into(self) -> SettingsPublic {
        todo!()
    }
}

impl ModelInDb for Settings {
    const COLLECTION_NAME: &'static str = "settings";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb = SettingsInDb;
}
