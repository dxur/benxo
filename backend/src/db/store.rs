use field::field;
use macros::model_in_db;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, ModelInDb};
use crate::models::store::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreInDb {
    pub id: String,
    pub business_id: ObjectId,
    pub name: String,
}

impl Into<StorePublic> for StoreInDb {
    fn into(self) -> StorePublic {
        StorePublic {
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<Result<Document>> for &StoreUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<FindInDb<String>> for &StoreDelete {
    fn into(self) -> FindInDb<String> {
        FindInDb {
            _id: self.id.clone(),
        }
    }
}

impl Into<FindInDb<String>> for &StoreUpdate {
    fn into(self) -> FindInDb<String> {
        FindInDb {
            _id: self.id.clone(),
        }
    }
}

impl From<ByBusinessId<StoreCreate>> for StoreInDb {
    fn from(value: ByBusinessId<StoreCreate>) -> Self {
        let ByBusinessId { business_id, body } = value;
        StoreInDb {
            id: body.id,
            business_id,
            name: body.name,
        }
    }
}

impl Into<FindInDb<String>> for &StoreFetch {
    fn into(self) -> FindInDb<String> {
        FindInDb {
            _id: self.id.clone(),
        }
    }
}

#[model_in_db(
    find=FindInDb<String>,
    fetch=StoreFetch,
    list=ByBusinessId<Void>,
    create=ByBusinessId<StoreCreate>,
    update=StoreUpdate,
    delete=StoreDelete,
)]
impl ModelInDb for Store {
    const COLLECTION_NAME: &'static str = "stores";

    type InDb = StoreInDb;
}
