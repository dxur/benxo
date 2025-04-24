use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelCreate {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
}

pub struct ThemeModel;
impl Model for ThemeModel {
    type ModelFetch = ThemeModelFetch;
    type ModelCreate = ThemeModelCreate;
    type ModelUpdate = ThemeModelUpdate;
    type ModelDelete = ThemeModelDelete;
    type ModelPublic = ThemeModelPublic;
}
