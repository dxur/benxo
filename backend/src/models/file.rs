use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::*;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct FileEntrySummary {
    pub name: String,
    pub preview: Option<String>,
    pub url: String,
    pub size: Option<usize>,
    #[ts(as = "String")]
    pub updated_at: Option<i64>,
}

#[derive(Debug, Serialize, Clone, TS)]
#[ts(export)]
pub struct FileUploadAccess {
    pub url: String,
}

#[derive(Debug, Deserialize, Clone, TS)]
#[ts(export)]
pub struct FileUploadRequest {
    pub name: String,
    pub mime: String,
    pub size: usize,
}
