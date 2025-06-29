use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Permission {
    Full,
    Products,
    Orders,
    Users,
    Site,
    Settings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Access {
    Read,
    Write,
    Full,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPermissions(Vec<(Permission, Access)>);

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserFetch {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct UserUpdateBody {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserUpdate {
    pub email: String,
    pub body: UserUpdateBody,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct UserDelete {
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct UserPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub email: String,
}

pub struct User;
