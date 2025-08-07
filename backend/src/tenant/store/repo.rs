use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use futures::stream::TryStreamExt;
use mongodb::{options::FindOptions, Client, Collection, Database};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait StoreRepo: Send + Sync {
    async fn create(&self, business_id: ObjectId, store: StoreRecord) -> ApiResult<StoreRecord>;
    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<StoreRecord>>;
    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        store: StoreRecord,
    ) -> ApiResult<StoreRecord>;
    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()>;
    async fn list(
        &self,
        business_id: ObjectId,
        filter: StoreFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<StoreRecord>, u64)>;
}

pub struct MongoStoreRepo {
    client: Client,
}

impl MongoStoreRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn get_collection(&self, business_id: ObjectId) -> Collection<StoreRecord> {
        self.client
            .database(&format!("biz-{}", business_id.to_hex()))
            .collection("stores")
    }

    fn build_filter_query(&self, filter: &StoreFilter) -> bson::Document {
        let mut query = doc! {};

        if let Some(ref status) = filter.status {
            query.insert("status", to_bson(status).unwrap());
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
                        "description": {
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
impl StoreRepo for MongoStoreRepo {
    async fn create(&self, business_id: ObjectId, store: StoreRecord) -> ApiResult<StoreRecord> {
        let collection = self.get_collection(business_id);

        collection.insert_one(&store).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ApiError::conflict("store", "Store with this slug already exists")
            } else {
                ApiError::internal(format!("Failed to create store: {}", e))
            }
        })?;

        Ok(store)
    }

    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<StoreRecord>> {
        let collection = self.get_collection(business_id);

        let store = collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(store)
    }

    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        mut store: StoreRecord,
    ) -> ApiResult<StoreRecord> {
        let collection = self.get_collection(business_id);

        store.updated_at = DateTime::now();

        let result = collection
            .replace_one(doc! { "_id": id }, &store)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update store: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("store", "Store not found"));
        }

        Ok(store)
    }

    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()> {
        let collection = self.get_collection(business_id);

        let result = collection
            .update_one(
                doc! { "_id": id },
                doc! {
                    "$set": {
                        "status": to_bson(&StoreStatus::Deleted).unwrap(),
                        "updated_at": DateTime::now()
                    }
                },
            )
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete store: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("store", "Store not found"));
        }

        Ok(())
    }

    async fn list(
        &self,
        business_id: ObjectId,
        filter: StoreFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<StoreRecord>, u64)> {
        let collection = self.get_collection(business_id);
        let query = self.build_filter_query(&filter);

        let total = collection
            .count_documents(query.clone())
            .await
            .map_err(|e| ApiError::internal(format!("Failed to count stores: {}", e)))?;

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

        let mut stores = Vec::new();
        while let Some(store) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            stores.push(store);
        }

        Ok((stores, total))
    }
}

#[async_trait]
pub trait StoreRegRepo: Send + Sync {
    async fn create(&self, store: StoreRegRecord) -> ApiResult<StoreRegRecord>;
    async fn find_by_store(
        &self,
        business_id: ObjectId,
        store_id: ObjectId,
    ) -> ApiResult<Option<StoreRegRecord>>;
    async fn find_by_slug(&self, slug: &str) -> ApiResult<Option<StoreRegRecord>>;
    async fn find_by_domain(&self, domain: &str) -> ApiResult<Option<StoreRegRecord>>;
    async fn update(
        &self,
        business_id: ObjectId,
        store_id: ObjectId,
        store: StoreRegRecord,
    ) -> ApiResult<StoreRegRecord>;
    async fn delete(&self, business_id: ObjectId, store_id: ObjectId) -> ApiResult<()>;
}

pub struct MongoStoreRegRepo {
    collection: Collection<StoreRegRecord>,
}

impl MongoStoreRegRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("stores_registry"),
        }
    }

    fn build_filter_query(&self, filter: &StoreRegFilter) -> bson::Document {
        let mut query = doc! {};
        query.insert("business_id", to_bson(filter.business_id.as_ref()).unwrap());

        if let Some(ref store_id) = filter.store_id {
            query.insert("store_id", to_bson(store_id.as_ref()).unwrap());
        }

        query
    }
}

#[async_trait]
impl StoreRegRepo for MongoStoreRegRepo {
    async fn create(&self, store_reg: StoreRegRecord) -> ApiResult<StoreRegRecord> {
        self.collection.insert_one(&store_reg).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ApiError::conflict("store_reg", "Store with this name already exists")
            } else {
                ApiError::internal(format!("Failed to create store_reg: {}", e))
            }
        })?;

        Ok(store_reg)
    }

    async fn find_by_store(
        &self,
        business_id: ObjectId,
        store_id: ObjectId,
    ) -> ApiResult<Option<StoreRegRecord>> {
        let store_reg = self
            .collection
            .find_one(doc! { "business_id": business_id, "store_id": store_id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(store_reg)
    }

    async fn find_by_slug(&self, slug: &str) -> ApiResult<Option<StoreRegRecord>> {
        let store_reg = self
            .collection
            .find_one(doc! { "slug": slug })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(store_reg)
    }

    async fn find_by_domain(&self, domain: &str) -> ApiResult<Option<StoreRegRecord>> {
        let store_reg = self
            .collection
            .find_one(doc! { "domain": domain })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(store_reg)
    }

    async fn update(
        &self,
        business_id: ObjectId,
        store_id: ObjectId,
        mut store_reg: StoreRegRecord,
    ) -> ApiResult<StoreRegRecord> {
        store_reg.updated_at = DateTime::now();

        let result = self
            .collection
            .replace_one(
                doc! { "business_id": business_id, "store_id": store_id },
                &store_reg,
            )
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update store: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("store_reg", "Store not found"));
        }

        Ok(store_reg)
    }

    async fn delete(&self, business_id: ObjectId, store_id: ObjectId) -> ApiResult<()> {
        let result = self
            .collection
            .delete_one(doc! { "business_id": business_id, "store_id": store_id })
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete store: {}", e)))?;

        if result.deleted_count == 0 {
            return Err(ApiError::not_found("store_reg", "Store not found"));
        }

        Ok(())
    }
}
