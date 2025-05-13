use bson::oid::ObjectId;
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

#[derive(Debug, Serialize, Deserialize)]
pub struct UserFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub email: String,
    pub password: String,
    pub permissions: UserPermissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateBody {
    pub name: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,
    pub permissions: Option<UserPermissions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdate {
    pub id: ObjectId,
    pub body: UserUpdateBody,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserPublic {
    pub id: ObjectId,
    pub name: String,
    pub email: String,
    pub permissions: UserPermissions,
}

pub struct User;
impl Model for User {
    type Public = UserPublic;
}
impl Fetchable for User {
    type Fetch = UserFetch;
}
impl Creatable for User {
    type Create = UserCreate;
}
impl Updatable for User {
    type Update = UserUpdate;
}
impl Deletable for User {
    type Delete = UserDelete;
}
