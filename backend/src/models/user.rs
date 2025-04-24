use common::models::user::{UserModel, UserPermissions};
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModelInDb {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub permissions: UserPermissions,
}

impl Model for UserModel {
    const COLLECTION_NAME: &'static str = "users";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelInDb = UserModelInDb;

    fn fetch(body: Self::ModelFetch) -> Document {
        doc! {field!(_id @ UserModelInDb): body.id}
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        Self::ModelInDb {
            _id: ObjectId::new(),
            name: body.name,
            email: body.email,
            password: body.password,
            permissions: body.permissions,
        }
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        Ok((
            doc! {field!(_id @ UserModelInDb): body.id},
            to_document(body).map_err(|_| ())?,
        ))
    }

    fn delete(body: &Self::ModelDelete) -> Document {
        doc! {field!(_id @ UserModelInDb): body.id}
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        Self::ModelPublic {
            id: body._id,
            name: body.name,
            email: body.email,
            permissions: body.permissions,
        }
    }
}
