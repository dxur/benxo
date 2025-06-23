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
    pub domain: String,
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

impl Into<StoreFetch> for &StoreDelete {
    fn into(self) -> StoreFetch {
        StoreFetch::Id(self.id.clone())
    }
}

impl Into<StoreFetch> for &StoreUpdate {
    fn into(self) -> StoreFetch {
        StoreFetch::Id(self.id.clone())
    }
}

impl From<ByBusinessId<StoreCreate>> for StoreInDb {
    fn from(value: ByBusinessId<StoreCreate>) -> Self {
        let ByBusinessId { business_id, body } = value;
        StoreInDb {
            id: body.id,
            domain: body.domain,
            business_id,
            name: body.name,
        }
    }
}

impl IntoFilter for StoreFetch {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<StoreFetch> for &StoreFetch {
    fn into(self) -> StoreFetch {
        (*self).clone()
    }
}

#[model_in_db(
    find=StoreFetch,
    fetch=StoreFetch,
    list=ByBusinessId<Void>,
    create=ByBusinessId<StoreCreate>,
    update=StoreUpdate,
    delete=StoreDelete,
)]
impl ModelInDb for Store {
    const COLLECTION_NAME: &'static str = "stores";
    const UNIQUE_INDICES: &'static [(&'static [&'static str], bool)] = &[
        (&[field!(id@ StoreInDb)], true),
        (&[field!(domain @ StoreInDb)], true),
    ];

    type InDb = StoreInDb;
}
