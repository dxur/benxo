use crate::models::theme::*;
use bson::DateTime;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeInDb {
    pub _id: ObjectId,
    pub name: String,
    pub path: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<ThemePublic> for ThemeInDb {
    fn into(self) -> ThemePublic {
        todo!()
    }
}

register_model!(Theme);
impl ModelInDb for Theme {
    const COLLECTION_NAME: &'static str = "themes";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb = ThemeInDb;
}
