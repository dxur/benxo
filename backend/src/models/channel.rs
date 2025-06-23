use bson::oid::ObjectId;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use ts_rs::TS;

use super::*;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelFetch {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ChannelVariant {
    Store { domain: String },
    Api { token: String },
    Custom,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelCreate {
    pub name: String,
    pub channel: ChannelVariant,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
pub enum ChannelVariantUpdateBody {
    Store { domain: Option<String> },
    Api { token: Option<String> },
    Custom,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, Clone, TS)]
pub struct ChannelUpdateBody {
    pub name: Option<String>,
    pub channel: Option<ChannelVariantUpdateBody>,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct ChannelUpdate {
    pub id: ObjectId,
    pub body: ChannelUpdateBody,
}

impl ChannelVariantUpdateBody {
    pub fn is_none(&self) -> bool {
        match self {
            ChannelVariantUpdateBody::Store { domain } => domain.is_none(),
            ChannelVariantUpdateBody::Api { token } => token.is_none(),
            ChannelVariantUpdateBody::Custom => true,
        }
    }
}

impl ChannelUpdateBody {
    pub fn is_none(&self) -> bool {
        self.name.is_none() && self.channel.is_none()
    }
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ChannelDelete {
    pub id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct ChannelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
}

pub struct Channel;
impl Model for Channel {
    type Public = ChannelPublic;
}
impl Fetchable for Channel {
    type Fetch = ChannelFetch;
}
impl Creatable for Channel {
    type Create = ChannelCreate;
}
impl Updatable for Channel {
    type Update = ChannelUpdate;
}
impl Deletable for Channel {
    type Delete = ChannelDelete;
}
