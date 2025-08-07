use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use futures::stream::TryStreamExt;
use mongodb::{options::FindOptions, Client, Collection};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait FileRepo: Send + Sync {
    async fn create(&self, business_id: ObjectId, file: FileRecord) -> ApiResult<FileRecord>;
    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<FileRecord>>;
    async fn find_by_key(&self, business_id: ObjectId, key: &str) -> ApiResult<Option<FileRecord>>;
    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        file: FileRecord,
    ) -> ApiResult<FileRecord>;
    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()>;
    async fn list(
        &self,
        business_id: ObjectId,
        filter: FileFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<FileRecord>, u64)>;
}

pub struct MongoFileRepo {
    client: Client,
}

impl MongoFileRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn get_collection(&self, business_id: ObjectId) -> Collection<FileRecord> {
        self.client
            .database(&format!("biz-{}", business_id.to_hex()))
            .collection("files")
    }

    fn build_filter_query(&self, filter: &FileFilter) -> bson::Document {
        let mut query = doc! {};

        if let Some(ref mime_type) = filter.mime_type {
            query.insert("mime_type", mime_type);
        }

        if let Some(ref search) = filter.search {
            query.insert(
                "$or",
                vec![
                    doc! {
                        "name": {
                            "$regex": search,
                            "$options": "i"
                        }
                    },
                    doc! {
                        "key": {
                            "$regex": search,
                            "$options": "i"
                        }
                    },
                ],
            );
        }

        query
    }
}

#[async_trait]
impl FileRepo for MongoFileRepo {
    async fn create(&self, business_id: ObjectId, file: FileRecord) -> ApiResult<FileRecord> {
        let collection = self.get_collection(business_id);

        collection.insert_one(&file).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ApiError::conflict("file", "File with this key already exists")
            } else {
                ApiError::internal(format!("Failed to create file: {}", e))
            }
        })?;

        Ok(file)
    }

    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<FileRecord>> {
        let collection = self.get_collection(business_id);

        let file = collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(file)
    }

    async fn find_by_key(&self, business_id: ObjectId, key: &str) -> ApiResult<Option<FileRecord>> {
        let collection = self.get_collection(business_id);

        let file = collection
            .find_one(doc! { "key": key })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(file)
    }

    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        mut file: FileRecord,
    ) -> ApiResult<FileRecord> {
        let collection = self.get_collection(business_id);

        file.updated_at = DateTime::now();

        let result = collection
            .replace_one(doc! { "_id": id }, &file)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update file: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("file", "File not found"));
        }

        Ok(file)
    }

    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()> {
        let collection = self.get_collection(business_id);

        let result = collection
            .delete_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete file: {}", e)))?;

        if result.deleted_count == 0 {
            return Err(ApiError::not_found("file", "File not found"));
        }

        Ok(())
    }

    async fn list(
        &self,
        business_id: ObjectId,
        filter: FileFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<FileRecord>, u64)> {
        let collection = self.get_collection(business_id);
        let query = self.build_filter_query(&filter);

        let total = collection
            .count_documents(query.clone())
            .await
            .map_err(|e| ApiError::internal(format!("Failed to count files: {}", e)))?;

        let skip = ((page.max(1) - 1) * limit) as u64;

        let find_options = FindOptions::builder()
            .skip(skip)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();

        let mut cursor = collection
            .find(query)
            .with_options(find_options)
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        let mut files = Vec::new();
        while let Some(file) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            files.push(file);
        }

        Ok((files, total))
    }
}
