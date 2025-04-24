use common::models::theme::ThemeModel;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeModelInDb {
    pub _id: ObjectId,
    pub name: String,
    pub path: String,
}

impl Model for ThemeModel {
    const COLLECTION_NAME: &'static str = "themes";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelInDb = ThemeModelInDb;

    fn fetch(body: Self::ModelFetch) -> Document {
        doc! {field!(_id @ ThemeModelInDb): body.id}
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        Self::ModelInDb {
            _id: ObjectId::new(),
            name: body.name,
            path: body.path,
        }
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        Ok((
            doc! {field!(_id @ ThemeModelInDb): body.id},
            to_document(body).map_err(|_| ())?,
        ))
    }

    fn delete(body: &Self::ModelDelete) -> Document {
        doc! {field!(_id @ ThemeModelInDb): body.id}
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        Self::ModelPublic {
            id: body._id,
            name: body.name,
        }
    }
}
