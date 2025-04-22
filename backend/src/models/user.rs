use field::*;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum UserPermissions {
    Admin,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelFetch {
    id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelCreate {
    name: String,
    email: String,
    password: String,
    permissions: UserPermissions,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelUpdate {
    #[serde(skip_serializing)]
    id: ObjectId,
    name: Option<String>,
    email: Option<String>,
    password: Option<String>,
    permissions: Option<UserPermissions>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelDelete {
    id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    name: String,
    email: String,
    permissions: UserPermissions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserModelInDb {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub permissions: UserPermissions,
}

pub struct UserModel;

impl Model for UserModel {
    const COLLECTION_NAME: &'static str = "templates";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelFetch = UserModelFetch;
    type ModelFilter = ();
    type ModelCreate = UserModelCreate;
    type ModelUpdate = UserModelUpdate;
    type ModelDelete = UserModelDelete;
    type ModelPublic = UserModelPublic;
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
