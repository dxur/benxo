use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use futures::stream::TryStreamExt;
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
    async fn find_businesses_by_member_email(&self, email: &str) -> ApiResult<Vec<BusinessRecord>>;
    async fn find_by_invitation_token(&self, token: &str) -> ApiResult<Option<BusinessRecord>>;
}

pub struct MongoBusinessRepo {
    collection: Collection<BusinessRecord>,
}

impl MongoBusinessRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("businesses"), // Fixed: was "users"
        }
    }

    fn build_filter_query(&self, filter: &BusinessFilter) -> bson::Document {
        let mut query = doc! {};

        if let Some(owner_id) = filter.owner_id {
            query.insert("owner_id", owner_id);
        }

        if let Some(ref plan_type) = filter.plan_type {
            query.insert("plan_type", to_bson(plan_type).unwrap());
        }

        if let Some(ref status) = filter.status {
            query.insert("status", to_bson(status).unwrap());
        }

        if let Some(ref name) = filter.name {
            query.insert(
                "name",
                doc! {
                    "$regex": name,
                    "$options": "i"
                },
            );
        }

        if let Some(ref member_email) = filter.member_email {
            query.insert("members.email", member_email);
        }

        if filter.created_after.is_some() || filter.created_before.is_some() {
            let mut date_query = doc! {};

            if let Some(after) = filter.created_after {
                date_query.insert("$gte", after);
            }

            if let Some(before) = filter.created_before {
                date_query.insert("$lte", before);
            }

            query.insert("created_at", date_query);
        }

        query
    }
}

#[async_trait]
impl BusinessRepo for MongoBusinessRepo {
    async fn create(&self, business: BusinessRecord) -> ApiResult<BusinessRecord> {
        self.collection.insert_one(&business).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ApiError::conflict("business", "Business with this name already exists")
            } else {
                ApiError::internal(format!("Failed to create business: {}", e))
            }
        })?;

        Ok(business)
    }

    async fn find_by_id(&self, id: ObjectId) -> ApiResult<Option<BusinessRecord>> {
        let business = self
            .collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(business)
    }

    async fn find_by_name(&self, name: &str) -> ApiResult<Option<BusinessRecord>> {
        let business = self
            .collection
            .find_one(doc! { "name": name })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(business)
    }

    async fn update(
        &self,
        id: ObjectId,
        mut business: BusinessRecord,
    ) -> ApiResult<BusinessRecord> {
        business.updated_at = DateTime::now();

        let result = self
            .collection
            .replace_one(doc! { "_id": id }, &business)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update business: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("business", "Business not found"));
        }

        Ok(business)
    }

    async fn delete(&self, id: ObjectId) -> ApiResult<()> {
        // Soft delete by updating status
        let result = self
            .collection
            .update_one(
                doc! { "_id": id },
                doc! {
                    "$set": {
                        "status": to_bson(&BusinessStatus::Deleted).unwrap(),
                        "updated_at": DateTime::now()
                    }
                },
            )
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete business: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("business", "Business not found"));
        }

        Ok(())
    }

    async fn list(
        &self,
        filter: BusinessFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<BusinessRecord>, u64)> {
        let query = self.build_filter_query(&filter);

        // Count total documents
        let total = self
            .collection
            .count_documents(query.clone())
            .await
            .map_err(|e| ApiError::internal(format!("Failed to count businesses: {}", e)))?;

        // Calculate skip value
        let skip = ((page.max(1) - 1) * limit) as u64;

        // Find documents with pagination
        let find_options = FindOptions::builder()
            .skip(skip)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();

        let mut cursor = self
            .collection
            .find(query)
            .with_options(find_options)
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        let mut businesses = Vec::new();
        while let Some(business) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            businesses.push(business);
        }

        Ok((businesses, total))
    }

    async fn find_by_owner(&self, owner_id: ObjectId) -> ApiResult<Vec<BusinessRecord>> {
        let mut cursor = self
            .collection
            .find(doc! {
                "owner_id": owner_id,
                "status": { "$ne": to_bson(&BusinessStatus::Deleted).unwrap() }
            })
            .with_options(
                FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .build(),
            )
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        let mut businesses = Vec::new();
        while let Some(business) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            businesses.push(business);
        }

        Ok(businesses)
    }

    async fn find_businesses_by_member_email(&self, email: &str) -> ApiResult<Vec<BusinessRecord>> {
        let mut cursor = self
            .collection
            .find(doc! {
                "members.email": email,
                "status": { "$ne": to_bson(&BusinessStatus::Deleted).unwrap() }
            })
            .with_options(
                FindOptions::builder()
                    .sort(doc! { "created_at": -1 })
                    .build(),
            )
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        let mut businesses = Vec::new();
        while let Some(business) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            businesses.push(business);
        }

        Ok(businesses)
    }

    async fn find_by_invitation_token(&self, token: &str) -> ApiResult<Option<BusinessRecord>> {
        let business = self
            .collection
            .find_one(doc! {
                "members.invitation_token": token,
                "status": { "$ne": to_bson(&BusinessStatus::Deleted).unwrap() }
            })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(business)
    }
}
