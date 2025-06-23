use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, FetchableInDb, FindableInDb, ModelInDb};
use crate::models::channel::*;
use crate::register_model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ChannelVariantInDb {
    Store { domain: String },
    Api { token: String },
    Custom,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelInDb {
    pub _id: ObjectId,
    pub business_id: ObjectId,
    pub name: String,
    pub channel: ChannelVariantInDb,
}

impl Into<ChannelVariantInDb> for ChannelVariant {
    fn into(self) -> ChannelVariantInDb {
        match self {
            ChannelVariant::Store { domain } => ChannelVariantInDb::Store { domain },
            ChannelVariant::Api { token } => ChannelVariantInDb::Api { token },
            ChannelVariant::Custom => ChannelVariantInDb::Custom,
        }
    }
}

impl Into<ChannelInDb> for ByBusinessId<ChannelCreate> {
    fn into(self) -> ChannelInDb {
        let ByBusinessId { business_id, body } = self;

        ChannelInDb {
            _id: ObjectId::new(),
            business_id,
            name: body.name,
            channel: body.channel.into(),
        }
    }
}

impl Into<ChannelPublic> for ChannelInDb {
    fn into(self) -> ChannelPublic {
        ChannelPublic {
            id: self._id,
            name: self.name,
        }
    }
}

impl Into<FindInDb> for &ChannelFetch {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}
impl Into<FindInDb> for &ChannelDelete {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<FindInDb> for &ChannelUpdate {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<Result<Document>> for &ChannelUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

register_model!(Channel);
impl ModelInDb for Channel {
    const COLLECTION_NAME: &'static str = "channels";

    type InDb = ChannelInDb;
}

impl FindableInDb for Channel {
    type FindInDb = ByBusinessId<FindInDb>;
}

impl FetchableInDb for Channel {
    type FetchInDb = ByBusinessId<ChannelFetch>;
}

impl CreatableInDb for Channel {
    type CreateInDb = ByBusinessId<ChannelCreate>;
}

impl UpdatableInDb for Channel {
    type UpdateInDb = ByBusinessId<ChannelUpdate>;
}

impl DeletableInDb for Channel {
    type DeleteInDb = ByBusinessId<ChannelDelete>;
}
