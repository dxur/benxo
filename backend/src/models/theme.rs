use field::*;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelFetch {
    id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelCreate {
    name: String,
    path: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelUpdate {
    #[serde(skip_serializing)]
    id: ObjectId,
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelDelete {
    id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ThemeModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeModelInDb {
    pub _id: ObjectId,
    pub name: String,
    pub path: String,
}

pub struct ThemeModel;

impl Model for ThemeModel {
    const COLLECTION_NAME: &'static str = "templates";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelFetch = ThemeModelFetch;
    type ModelFilter = ();
    type ModelCreate = ThemeModelCreate;
    type ModelUpdate = ThemeModelUpdate;
    type ModelDelete = ThemeModelDelete;
    type ModelPublic = ThemeModelPublic;
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
