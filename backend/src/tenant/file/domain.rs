use std::borrow::Cow;

use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::name::Name;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FileRecord {
    pub _id: ObjectId,
    pub key: String,
    pub name: Name,
    pub mime_type: Cow<'static, str>,
    pub size: Option<u64>,
    pub metadata: std::collections::HashMap<String, String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for FileRecord {
    fn default() -> Self {
        let now = DateTime::now();

        Self {
            _id: Default::default(),
            key: Default::default(),
            name: Default::default(),
            mime_type: Cow::from("application/octet-stream"),
            size: None,
            metadata: Default::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FileFilter {
    pub mime_type: Option<String>,
    pub search: Option<String>,
}
