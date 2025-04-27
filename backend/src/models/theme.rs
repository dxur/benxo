use common::models::theme::*;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeInDb {
    pub _id: ObjectId,
    pub name: String,
    pub path: String,
}

impl Into<ThemePublic> for ThemeInDb {
    fn into(self) -> ThemePublic {
        todo!()
    }
}

impl Model for Theme {
    const COLLECTION_NAME: &'static str = "themes";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb = ThemeInDb;
}
