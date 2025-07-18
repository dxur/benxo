use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use mongodb::{options::FindOptions, Collection, Database};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait BusinessRepo: Send + Sync {
    async fn create(&self, business: BusinessRecord) -> ApiResult<BusinessRecord>;
    async fn find_by_id(&self, id: ObjectId) -> ApiResult<Option<BusinessRecord>>;
    async fn find_by_name(&self, name: &str) -> ApiResult<Option<BusinessRecord>>;
    async fn update(&self, id: ObjectId, business: BusinessRecord) -> ApiResult<BusinessRecord>;
    async fn delete(&self, id: ObjectId) -> ApiResult<()>;
    async fn list(
        &self,
        filter: BusinessFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<BusinessRecord>, u64)>;
    async fn find_by_owner(&self, owner_id: ObjectId) -> ApiResult<Vec<BusinessRecord>>;
}
