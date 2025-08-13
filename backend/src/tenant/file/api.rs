use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use ts_rs::TS;

use super::domain::*;
use crate::{
    types::{id::Id, name::Name},
    utils::serde_helpers::JsonOption,
};

#[derive(Debug, Deserialize, TS)]
#[ts(export, optional_fields)]
pub struct FileCreate {
    pub key: String,
    pub name: Name,
    pub mime_type: Option<String>,
    pub size: Option<u64>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export, optional_fields)]
#[from_owned(FileRecord)]
pub struct FileDto {
    #[from(@._id.into())]
    pub id: Id,
    pub key: String,
    pub name: Name,
    pub mime_type: Cow<'static, str>,
    pub size: Option<u64>,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export, optional_fields)]
pub struct FileListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub mime_type: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, optional_fields)]
pub struct FileListResponse {
    pub files: Vec<FileDto>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, optional_fields)]
pub struct PresignedUrlResponse {
    pub url: String,
    pub fields: HashMap<String, String>,
    pub dynamic_fields: HashMap<String, String>,
    pub expiration: DateTime<Utc>,
}
