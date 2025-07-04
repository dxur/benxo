use bson::serde_helpers::serialize_bson_datetime_as_rfc3339_string;
use bson::serde_helpers::serialize_object_id_as_hex_string;
use bson::DateTime;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CategoryCreate {
    pub name: String,
    pub slug: String,
}

#[skip_serializing_none]
#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CategoryUpdateBody {
    pub name: Option<String>,
    pub slug: Option<String>,
    pub archived: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct CategoryUpdate {
    pub id: ObjectId,
    pub body: CategoryUpdateBody,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct CategoryPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    pub id: ObjectId,
    pub name: String,
    pub slug: String,
    pub archived: bool,
    #[ts(as = "String")]
    #[serde(serialize_with = "serialize_bson_datetime_as_rfc3339_string")]
    pub created_at: DateTime,
}
