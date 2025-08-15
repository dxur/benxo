use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use futures::stream::TryStreamExt;
use mongodb::{options::FindOptions, Client, Collection};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait ProductRepo: Send + Sync {
    async fn create(
        &self,
        business_id: ObjectId,
        product: ProductRecord,
    ) -> ApiResult<ProductRecord>;
    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<ProductRecord>>;
    async fn find_by_slug(
        &self,
        business_id: ObjectId,
        slug: &str,
    ) -> ApiResult<Option<ProductRecord>>;
    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        product: ProductRecord,
    ) -> ApiResult<ProductRecord>;
    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()>;
    async fn list(
        &self,
        business_id: ObjectId,
        filter: ProductFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<ProductRecord>, u64)>;
}

pub struct MongoProductRepo {
    client: Client,
}

impl MongoProductRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn get_collection(&self, business_id: ObjectId) -> Collection<ProductRecord> {
        self.client
            .database(&format!("biz-{}", business_id.to_hex()))
            .collection("products")
    }

    fn build_filter_query(&self, filter: &ProductFilter) -> bson::Document {
        let mut query = doc! {};

        if let Some(ref status) = filter.status {
            query.insert("status", to_bson(status).unwrap());
        }

        if let Some(ref category) = filter.category {
            query.insert("category", category);
        }

        if let Some(featured) = filter.featured {
            query.insert("featured", featured);
        }

        if let Some(ref search) = filter.search {
            query.insert(
                "$or",
                vec![
                    doc! {
                        "title": {
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
impl ProductRepo for MongoProductRepo {
    async fn create(
        &self,
        business_id: ObjectId,
        product: ProductRecord,
    ) -> ApiResult<ProductRecord> {
        let collection = self.get_collection(business_id);

        collection.insert_one(&product).await.map_err(|e| {
            if e.to_string().contains("duplicate key") {
                ApiError::conflict("product", "Product with this slug already exists")
            } else {
                ApiError::internal(format!("Failed to create product: {}", e))
            }
        })?;

        Ok(product)
    }

    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<ProductRecord>> {
        let collection = self.get_collection(business_id);

        let product = collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(product)
    }

    async fn find_by_slug(
        &self,
        business_id: ObjectId,
        slug: &str,
    ) -> ApiResult<Option<ProductRecord>> {
        let collection = self.get_collection(business_id);

        let product = collection
            .find_one(doc! { "slug": slug })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(product)
    }

    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        mut product: ProductRecord,
    ) -> ApiResult<ProductRecord> {
        let collection = self.get_collection(business_id);

        product.updated_at = DateTime::now();

        let result = collection
            .replace_one(doc! { "_id": id }, &product)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update product: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("product", "Product not found"));
        }

        Ok(product)
    }

    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()> {
        let collection = self.get_collection(business_id);

        let result = collection
            .delete_one(
                doc! { "_id": id },
            )
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete product: {}", e)))?;

        if result.deleted_count == 0 {
            return Err(ApiError::not_found("product", "Product not found"));
        }

        Ok(())
    }

    async fn list(
        &self,
        business_id: ObjectId,
        filter: ProductFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<ProductRecord>, u64)> {
        let collection = self.get_collection(business_id);
        let query = self.build_filter_query(&filter);

        let total = collection
            .count_documents(query.clone())
            .await
            .map_err(|e| ApiError::internal(format!("Failed to count products: {}", e)))?;

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

        let mut products = Vec::new();
        while let Some(product) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            products.push(product);
        }

        Ok((products, total))
    }
}
