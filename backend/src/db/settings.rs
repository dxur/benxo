use bson::DateTime;
use field::*;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::*;
use crate::{models::settings::*, register_model};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SettingsInDb {
    pub business_id: ObjectId,
    pub name: String,
    pub description: String,
    pub phone: String,
    pub email: String,
    pub domain: String,
    pub active_theme: ObjectId,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<SettingsPublic> for SettingsInDb {
    fn into(self) -> SettingsPublic {
        SettingsPublic {
            name: self.name,
            description: self.description,
            phone: self.phone,
            email: self.email,
            domain: self.domain,
        }
    }
}

impl Into<Result<Document>> for &SettingsUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self).map_err(|e| Error { msg: e.to_string() })
    }
}

register_model!(Settings);
impl ModelInDb for Settings {
    const COLLECTION_NAME: &'static str = "settings";

    type InDb = SettingsInDb;

    async fn init_coll(db: &Db) -> Result<()> {
        let coll = db.collection::<Self::InDb>(Self::COLLECTION_NAME);
        coll.create_index(
            IndexModel::builder()
                .keys(doc! {
                   field!(business_id @ SettingsInDb): 1,
                })
                .build(),
        )
        .await
        .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))?;

        coll.create_index(
            IndexModel::builder()
                .keys(doc! {
                    field!(domain @ SettingsInDb): 1,
                })
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .partial_filter_expression(doc! {
                            field!(domain @ SettingsInDb): { "$gt": "" }
                        })
                        .build(),
                )
                .build(),
        )
        .await
        .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))
    }
}

impl FindableInDb for Settings {
    type FindInDb = ByBusinessId<Void>;
}

impl FetchableInDb for Settings {
    type FetchInDb = ByBusinessId<Void>;
}

impl UpdatableInDb for Settings {
    type UpdateInDb = ByBusinessId<SettingsUpdate>;
}
