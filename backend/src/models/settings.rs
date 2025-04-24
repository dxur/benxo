use common::models::settings::SettingsModel;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;
use crate::{
    db::{DB, DEFAULT_UUID},
    events::Event,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsModelInDb {
    pub _id: ObjectId, // defaulted to zeros
    pub store_name: String,
    pub store_domain: String,
    pub active_theme: ObjectId,
}

impl Model for SettingsModel {
    const COLLECTION_NAME: &'static str = "settings";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelInDb = SettingsModelInDb;

    fn fetch(body: Self::ModelFetch) -> Document {
        todo!()
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        todo!()
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        todo!()
    }

    fn delete(body: &Self::ModelDelete) -> Document {
        todo!()
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        todo!()
    }
}
