use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;
use crate::{
    db::{DB, DEFAULT_UUID},
    events::Event,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct SettingsModelCreate {
    store_name: String,
    store_domain: String,
}

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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsModelInDb {
    pub _id: ObjectId, // defaulted to zeros
    pub store_name: String,
    pub store_domain: String,
    pub active_theme: ObjectId,
}

pub struct SettingsModel;

impl Model for SettingsModel {
    const COLLECTION_NAME: &'static str = "settings";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelFetch = ();
    type ModelFilter = ();
    type ModelCreate = SettingsModelCreate;
    type ModelUpdate = SettingsModelUpdate;
    type ModelDelete = ();
    type ModelPublic = SettingsModelPublic;
    type ModelInDb = SettingsModelInDb;

    fn fetch(_: Self::ModelFetch) -> Document {
        doc! {field!(_id @ SettingsModelInDb): DEFAULT_UUID}
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        Self::ModelInDb {
            _id: DEFAULT_UUID,
            store_name: body.store_name,
            store_domain: body.store_domain,
            active_theme: DEFAULT_UUID,
        }
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        Ok((
            doc! {field!(_id @ SettingsModelInDb): DEFAULT_UUID},
            to_document(body).map_err(|_| ())?,
        ))
    }

    fn delete(_: &Self::ModelDelete) -> Document {
        doc! {field!(_id @ SettingsModelInDb): DEFAULT_UUID}
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        Self::ModelPublic {
            store_name: body.store_name,
            store_domain: body.store_domain,
        }
    }

    async fn init_coll_in_db(db: &DB) -> Result<(), String> {
        SettingsModel::create_in_db(
            db,
            SettingsModelCreate {
                store_name: "Rus".to_string(),
                store_domain: "".to_string(),
            },
        )
        .await
        .map_err(|_| "Something just happened".to_string())
        .map(|_| ())
    }

    async fn on_update(state: &crate::AppState, body: &Self::ModelUpdate, value: &Self::ModelInDb) {
        if body.active_theme.is_some() {
            state
                .event_bus
                .emit(Event::ThemeUpdated(value.active_theme))
                .await;
        }
    }
}
