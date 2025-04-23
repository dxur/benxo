use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserPermissions {
    Full,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelCreate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub permissions: UserPermissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub permissions: Option<UserPermissions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub permissions: UserPermissions,
}

pub struct UserModel;
impl Model for UserModel {
    type ModelFetch = UserModelFetch;
    type ModelCreate = UserModelCreate;
    type ModelUpdate = UserModelUpdate;
    type ModelDelete = UserModelDelete;
    type ModelPublic = UserModelPublic;
}
