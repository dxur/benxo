use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use mongodb::{options::FindOptions, Collection, Database};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait UserRepo: Send + Sync {
    async fn create(&self, user: UserRecord) -> ApiResult<UserRecord>;
    async fn find_by_id(&self, id: ObjectId) -> ApiResult<Option<UserRecord>>;
    async fn find_by_email(&self, email: &str) -> ApiResult<Option<UserRecord>>;
    async fn find_by_username(&self, username: &str) -> ApiResult<Option<UserRecord>>;
    async fn find_by_email_verification_token(&self, token: &str) -> ApiResult<Option<UserRecord>>;
    async fn find_by_password_reset_token(&self, token: &str) -> ApiResult<Option<UserRecord>>;
    async fn update(&self, id: ObjectId, user: UserRecord) -> ApiResult<UserRecord>;
    async fn delete(&self, id: ObjectId) -> ApiResult<()>;
    async fn list(
        &self,
        filter: UserFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<UserRecord>, u64)>;
    async fn count(&self, filter: UserFilter) -> ApiResult<u64>;
    async fn increment_login_attempts(&self, id: ObjectId) -> ApiResult<()>;
    async fn reset_login_attempts(&self, id: ObjectId) -> ApiResult<()>;
    async fn lock_user(&self, id: ObjectId, until: DateTime) -> ApiResult<()>;
}

pub struct MongoUserRepo {
    collection: Collection<UserRecord>,
}

impl MongoUserRepo {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection("users"),
        }
    }

    fn build_filter_doc(&self, filter: &UserFilter) -> bson::Document {
        let mut doc = bson::Document::new();
        if let Some(ref email) = filter.email {
            doc.insert("email", email);
        }

        if let Some(ref username) = filter.username {
            doc.insert("username", username);
        }

        if let Some(ref status) = filter.status {
            doc.insert("status", to_bson(&status).unwrap());
        }

        if let Some(created_after) = filter.created_after {
            doc.insert("created_at", doc! { "$gte": created_after });
        }

        if let Some(created_before) = filter.created_before {
            doc.insert("created_at", doc! { "$lte": created_before });
        }

        doc
    }
}

#[async_trait]
impl UserRepo for MongoUserRepo {
    async fn create(&self, user: UserRecord) -> ApiResult<UserRecord> {
        self.collection
            .insert_one(&user)
            .await
            .map_err(|e| match *e.kind {
                mongodb::error::ErrorKind::Write(mongodb::error::WriteFailure::WriteError(w))
                    if w.code == 11000 =>
                {
                    ApiError::conflict("user", "Email or Username already exists")
                }
                _ => ApiError::database(e.to_string()),
            })?;

        Ok(user)
    }

    async fn find_by_id(&self, id: ObjectId) -> ApiResult<Option<UserRecord>> {
        let filter = doc! { "_id": id };
        self.collection
            .find_one(filter)
            .await
            .map_err(|e| ApiError::database(e.to_string()))
    }

    async fn find_by_email(&self, email: &str) -> ApiResult<Option<UserRecord>> {
        let filter = doc! { "email": email};
        self.collection
            .find_one(filter)
            .await
            .map_err(|e| ApiError::database(e.to_string()))
    }

    async fn find_by_username(&self, username: &str) -> ApiResult<Option<UserRecord>> {
        let filter = doc! { "username": username };
        self.collection
            .find_one(filter)
            .await
            .map_err(|e| ApiError::database(e.to_string()))
    }

    async fn find_by_email_verification_token(&self, token: &str) -> ApiResult<Option<UserRecord>> {
        let filter = doc! { "email_verification_token": token };
        self.collection
            .find_one(filter)
            .await
            .map_err(|e| ApiError::database(e.to_string()))
    }

    async fn find_by_password_reset_token(&self, token: &str) -> ApiResult<Option<UserRecord>> {
        let filter = doc! {
            "password_reset_token": token,
            "password_reset_expires": { "$gt": DateTime::now() },
        };
        self.collection
            .find_one(filter)
            .await
            .map_err(|e| ApiError::database(e.to_string()))
    }

    async fn update(&self, id: ObjectId, mut user: UserRecord) -> ApiResult<UserRecord> {
        user.updated_at = DateTime::now();

        let filter = doc! { "_id": id };
        let update = doc! { "$set": bson::to_document(&user).unwrap() };

        self.collection
            .update_one(filter, update)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        Ok(user)
    }

    async fn delete(&self, id: ObjectId) -> ApiResult<()> {
        let filter = doc! { "_id": id };
        let result = self
            .collection
            .delete_one(filter)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        if result.deleted_count == 0 {
            return Err(ApiError::not_found("user", id.to_hex()));
        }

        Ok(())
    }

    async fn list(
        &self,
        filter: UserFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<UserRecord>, u64)> {
        let filter_doc = self.build_filter_doc(&filter);

        let skip = (page - 1) * limit;
        let options = FindOptions::builder()
            .skip(skip as u64)
            .limit(limit as i64)
            .sort(doc! { "created_at": -1 })
            .build();

        let mut cursor = self
            .collection
            .find(filter_doc.clone())
            .with_options(options)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        let mut users = Vec::new();
        while cursor
            .advance()
            .await
            .map_err(|e| ApiError::database(e.to_string()))?
        {
            users.push(
                cursor
                    .deserialize_current()
                    .map_err(|e| ApiError::database(e.to_string()))?,
            );
        }

        let total = self
            .collection
            .count_documents(filter_doc)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        Ok((users, total))
    }

    async fn count(&self, filter: UserFilter) -> ApiResult<u64> {
        let filter_doc = self.build_filter_doc(&filter);
        self.collection
            .count_documents(filter_doc)
            .await
            .map_err(|e| ApiError::database(e.to_string()))
    }

    async fn increment_login_attempts(&self, id: ObjectId) -> ApiResult<()> {
        let filter = doc! { "_id": id };
        let update = doc! { "$inc": { "login_attempts": 1 } };

        self.collection
            .update_one(filter, update)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        Ok(())
    }

    async fn reset_login_attempts(&self, id: ObjectId) -> ApiResult<()> {
        let filter = doc! { "_id": id };
        let update = doc! {
            "$set": { "login_attempts": 0 },
            "$unset": { "locked_until": "" }
        };

        self.collection
            .update_one(filter, update)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        Ok(())
    }

    async fn lock_user(&self, id: ObjectId, until: DateTime) -> ApiResult<()> {
        let filter = doc! { "_id": id };
        let update = doc! { "$set": { "locked_until": until } };

        self.collection
            .update_one(filter, update)
            .await
            .map_err(|e| ApiError::database(e.to_string()))?;

        Ok(())
    }
}
