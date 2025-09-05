use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use futures::stream::TryStreamExt;
use mongodb::{Collection, Database};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait DomainRepo: Send + Sync {
    async fn find_by_domain(&self, domain: &str) -> ApiResult<Option<DomainRecord>>;
    async fn find_by_business_id(&self, business_id: ObjectId) -> ApiResult<Vec<DomainRecord>>;
    async fn create(&self, domain: DomainRecord) -> ApiResult<DomainRecord>;
    async fn update(&self, domain: DomainRecord) -> ApiResult<DomainRecord>;
    async fn delete_expired(&self) -> ApiResult<u64>;
}

pub struct MongoDomainRepo {
    collection: Collection<DomainRecord>,
}

impl MongoDomainRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("domain_cache"),
        }
    }
}

#[async_trait]
impl DomainRepo for MongoDomainRepo {
    async fn find_by_domain(&self, domain: &str) -> ApiResult<Option<DomainRecord>> {
        let record = self
            .collection
            .find_one(doc! { "domain": domain.to_lowercase() })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(record)
    }

    async fn find_by_business_id(&self, business_id: ObjectId) -> ApiResult<Vec<DomainRecord>> {
        let mut cursor = self
            .collection
            .find(doc! { "business_id": business_id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        let mut domains = Vec::new();
        while let Some(domain) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            domains.push(domain);
        }

        Ok(domains)
    }

    async fn create(&self, domain: DomainRecord) -> ApiResult<DomainRecord> {
        self.collection
            .insert_one(&domain)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to create domain: {}", e)))?;

        Ok(domain)
    }

    async fn update(&self, domain: DomainRecord) -> ApiResult<DomainRecord> {
        self.collection
            .replace_one(doc! { "_id": domain._id }, &domain)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update domain: {}", e)))?;

        Ok(domain)
    }

    async fn delete_expired(&self) -> ApiResult<u64> {
        let now = DateTime::now();
        let result = self
            .collection
            .delete_many(doc! { "expires_at": { "$lte": now } })
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete expired domains: {}", e)))?;

        Ok(result.deleted_count)
    }
}
