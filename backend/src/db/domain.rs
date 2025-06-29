use macros::model_in_db;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, ModelInDb};
use crate::models::domain::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainInDb {
    pub _id: String,
    pub business_id: ObjectId,
    pub store_id: String,
}

impl IntoFilter for DomainList {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<DomainPublic> for DomainInDb {
    fn into(self) -> DomainPublic {
        DomainPublic {
            domain: self._id,
            store_id: self.store_id,
        }
    }
}

impl Into<FindInDb<String>> for &ByBusinessId<DomainDelete> {
    fn into(self) -> FindInDb<String> {
        FindInDb {
            _id: self.body.domain.clone(),
        }
    }
}

impl Into<DomainInDb> for ByBusinessId<DomainCreate> {
    fn into(self) -> DomainInDb {
        let ByBusinessId { business_id, body } = self;
        DomainInDb {
            _id: body.domain,
            business_id,
            store_id: body.store_id,
        }
    }
}

impl Into<FindInDb<String>> for &DomainFetch {
    fn into(self) -> FindInDb<String> {
        FindInDb {
            _id: self.domain.clone(),
        }
    }
}

#[model_in_db(
    find=FindInDb<String>,
    fetch=DomainFetch,
    list=ByBusinessId<DomainList>,
    create=ByBusinessId<DomainCreate>,
    delete=ByBusinessId<DomainDelete>,
)]
impl ModelInDb for Domain {
    const COLLECTION_NAME: &'static str = "domains";

    type InDb = DomainInDb;
}
