use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeCreate {
    pub name: String,
    pub path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeUpdate {
    pub id: ObjectId,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemePublic {
    pub id: ObjectId,
    pub name: String,
}

pub struct Theme;
impl Model for Theme {
    type Public = ThemePublic;
}
impl Fetchable for Theme {
    type Fetch = ThemeFetch;
}
impl Creatable for Theme {
    type Create = ThemeCreate;
}
impl Updatable for Theme {
    type Update = ThemeUpdate;
}
impl Deletable for Theme {
    type Delete = ThemeDelete;
}
