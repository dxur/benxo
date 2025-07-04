use hex_color::HexColor;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, ModelInDb};
use crate::models::store::*;
use crate::register_model;

#[derive(Debug, Serialize)]
pub struct StoreFindInDb {
    pub store_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StoreInDb {
    pub _id: ObjectId,
    pub store_id: String,
    pub business_id: ObjectId,
    pub name: String,
    pub description: String,
    pub primary_color: HexColor,
    pub secondary_color: HexColor,
    pub background_color: HexColor,
    pub logo: String,
    pub head: String,
}

impl Into<StorePublic> for StoreInDb {
    fn into(self) -> StorePublic {
        StorePublic {
            store_id: self.store_id,
            name: self.name,
            description: self.description,
            primary_color: self.primary_color,
            secondary_color: self.secondary_color,
            background_color: self.background_color,
            logo: self.logo,
            head: self.head,
        }
    }
}

impl IntoFilter for StoreFindInDb {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl RefInto<Result<Document>> for StoreUpdate {
    fn ref_into(&self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<StoreFindInDb> for &ByBusinessId<StoreDelete> {
    fn into(self) -> StoreFindInDb {
        StoreFindInDb {
            store_id: self.body.store_id.clone(),
        }
    }
}

impl RefInto<StoreFindInDb> for ByBusinessId<StoreUpdate> {
    fn ref_into(&self) -> StoreFindInDb {
        StoreFindInDb {
            store_id: self.body.store_id.clone(),
        }
    }
}

impl From<ByBusinessId<StoreCreate>> for StoreInDb {
    fn from(value: ByBusinessId<StoreCreate>) -> Self {
        let ByBusinessId { business_id, body } = value;
        StoreInDb {
            _id: ObjectId::new(),
            store_id: body.store_id,
            business_id,
            name: body.name,
            description: "Your amazing store".to_owned(),
            primary_color: HexColor::GRAY,
            secondary_color: HexColor::BLACK,
            background_color: HexColor::WHITE,
            logo: "".to_owned(),
            head: "".to_owned(),
        }
    }
}

impl Into<StoreFindInDb> for &StoreFetch {
    fn into(self) -> StoreFindInDb {
        StoreFindInDb {
            store_id: self.store_id.clone(),
        }
    }
}

pub struct Store;

register_model!(Store);
impl ModelInDb for Store {
    const COLLECTION_NAME: &'static str = "stores";

    type InDb = StoreInDb;
}

impl FindableInDb for Store {
    type FindInDb = StoreFindInDb;
}

impl FetchableInDb for Store {
    type FetchInDb = StoreFetch;
}

impl ListableInDb for Store {
    type ListInDb = ByBusinessId<Void>;
}

impl CreatableInDb for Store {
    type CreateInDb = ByBusinessId<StoreCreate>;
}

impl UpdatableInDb for Store {
    type UpdateInDb = ByBusinessId<StoreUpdate>;
}

impl DeletableInDb for Store {
    type DeleteInDb = ByBusinessId<StoreDelete>;
}
