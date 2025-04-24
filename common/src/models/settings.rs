use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsModelUpdate {
    store_name: Option<String>,
    store_domain: Option<String>,
    active_theme: Option<ObjectId>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsModelPublic {
    store_name: String,
    store_domain: String,
}

pub struct SettingsModel;
impl Model for SettingsModel {
    type ModelFetch = ();
    type ModelCreate = ();
    type ModelUpdate = SettingsModelUpdate;
    type ModelDelete = ();
    type ModelPublic = SettingsModelPublic;
}
