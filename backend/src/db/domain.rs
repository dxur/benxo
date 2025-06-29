use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, ModelInDb};
use crate::models::domain::*;
use crate::register_model;

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

pub struct Domain;

register_model!(Domain);
impl ModelInDb for Domain {
    const COLLECTION_NAME: &'static str = "domains";
    type InDb = DomainInDb;
}

impl FindableInDb for Domain {
    type FindInDb = FindInDb<String>;
}

impl FetchableInDb for Domain {
    type FetchInDb = DomainFetch;
}

impl ListableInDb for Domain {
    type ListInDb = ByBusinessId<DomainList>;
}

impl CreatableInDb for Domain {
    type CreateInDb = ByBusinessId<DomainCreate>;
}

impl DeletableInDb for Domain {
    type DeleteInDb = ByBusinessId<DomainDelete>;
}
