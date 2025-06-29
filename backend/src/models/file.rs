use bson::oid::ObjectId;
use bson::DateTime;
use macros::Model;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct FileEntryPublic {
    pub id: String,
    pub name: String,
    pub url: String,
    pub mime_type: String,
    pub size: usize,
    #[ts(as = "String")]
    pub created_at: DateTime,
}
