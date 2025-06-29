use hex_color::HexColor;
use macros::model_in_db;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, ModelInDb};
use crate::models::store::*;

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
            head: self.head,
        }
    }
}

impl IntoFilter for StoreFindInDb {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<Result<Document>> for &StoreUpdate {
    fn into(self) -> Result<Document> {
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

impl Into<StoreFindInDb> for &ByBusinessId<StoreUpdate> {
    fn into(self) -> StoreFindInDb {
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

#[model_in_db(
    find=StoreFindInDb,
    fetch=StoreFetch,
    list=ByBusinessId<Void>,
    create=ByBusinessId<StoreCreate>,
    update=ByBusinessId<StoreUpdate>,
    delete=ByBusinessId<StoreDelete>,
)]
impl ModelInDb for Store {
    const COLLECTION_NAME: &'static str = "stores";

    type InDb = StoreInDb;
}
