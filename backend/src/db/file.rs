use bson::DateTime;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::ModelInDb;
use super::*;
use crate::{
    models::{file::*, ById},
    register_model,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct FileEntryInDb {
    pub _id: String,
    pub business_id: ObjectId,
    pub name: String,
    pub url: String,
    pub mime_type: String,
    pub size: usize,
    pub created_at: DateTime,
}

impl Into<FileEntryPublic> for FileEntryInDb {
    fn into(self) -> FileEntryPublic {
        FileEntryPublic {
            id: self._id,
            name: self.name,
            url: self.url,
            mime_type: self.mime_type,
            size: self.size,
            created_at: self.created_at,
        }
    }
}

pub struct FileEntry;

register_model!(FileEntry);
impl ModelInDb for FileEntry {
    const COLLECTION_NAME: &'static str = "file_entries";
    type InDb = FileEntryInDb;
}

impl FindableInDb for FileEntry {
    type FindInDb = ByBusinessId<FindInDb>;
}

impl FetchableInDb for FileEntry {
    type FetchInDb = ByBusinessId<ById>;
}

impl CreatableInDb for FileEntry {
    type CreateInDb = FileEntryInDb;
}

impl ListableInDb for FileEntry {
    type ListInDb = ByBusinessId<Void>;
}

impl DeletableInDb for FileEntry {
    type DeleteInDb = ByBusinessId<ById>;
}
