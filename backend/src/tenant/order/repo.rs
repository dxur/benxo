use async_trait::async_trait;
use bson::{doc, oid::ObjectId, to_bson, DateTime};
use futures::stream::TryStreamExt;
use mongodb::{options::FindOptions, Client, Collection};

use super::domain::*;
use crate::utils::error::{ApiError, ApiResult};

#[async_trait]
pub trait OrderRepo: Send + Sync {
    async fn create(&self, business_id: ObjectId, order: OrderRecord) -> ApiResult<OrderRecord>;
    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<OrderRecord>>;
    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        order: OrderRecord,
    ) -> ApiResult<OrderRecord>;
    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()>;
    async fn list(
        &self,
        business_id: ObjectId,
        filter: OrderFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<OrderRecord>, u64)>;
    async fn bulk_update_status(
        &self,
        business_id: ObjectId,
        order_ids: Vec<ObjectId>,
        status: OrderStatus,
        note: Option<String>,
        created_by: Option<String>,
    ) -> ApiResult<u64>;
    async fn get_customer_orders(
        &self,
        business_id: ObjectId,
        customer_email: &str,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<OrderRecord>, u64)>;
}

pub struct MongoOrderRepo {
    client: Client,
}

impl MongoOrderRepo {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    fn get_collection(&self, business_id: ObjectId) -> Collection<OrderRecord> {
        self.client
            .database(&format!("biz_{}", business_id.to_hex()))
            .collection("orders")
    }

    fn build_filter_query(&self, filter: &OrderFilter) -> bson::Document {
        let mut query = doc! {};

        if let Some(ref status) = filter.status {
            query.insert("status", to_bson(status).unwrap());
        }

        if let Some(ref payment_status) = filter.payment_status {
            query.insert("payment_status", to_bson(payment_status).unwrap());
        }

        if let Some(ref customer_email) = filter.customer_email {
            query.insert("customer_email", customer_email);
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

        // TODO: add date from to

        query
    }
}

#[async_trait]
impl OrderRepo for MongoOrderRepo {
    async fn create(&self, business_id: ObjectId, order: OrderRecord) -> ApiResult<OrderRecord> {
        todo!()
    }

    async fn find_by_id(
        &self,
        business_id: ObjectId,
        id: ObjectId,
    ) -> ApiResult<Option<OrderRecord>> {
        let collection = self.get_collection(business_id);

        let order = collection
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| ApiError::internal(format!("Database query failed: {}", e)))?;

        Ok(order)
    }

    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        mut order: OrderRecord,
    ) -> ApiResult<OrderRecord> {
        let collection = self.get_collection(business_id);

        order.updated_at = DateTime::now();

        let result = collection
            .replace_one(doc! { "_id": id }, &order)
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update order: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("order", "order not found"));
        }

        Ok(order)
    }

    async fn delete(&self, business_id: ObjectId, id: ObjectId) -> ApiResult<()> {
        let collection = self.get_collection(business_id);

        let result = collection
            .update_one(
                doc! { "_id": id },
                doc! {
                    "$set": {
                        "status": to_bson(&OrderStatus::Deleted).unwrap(),
                        "updated_at": DateTime::now()
                    }
                },
            )
            .await
            .map_err(|e| ApiError::internal(format!("Failed to delete order: {}", e)))?;

        if result.matched_count == 0 {
            return Err(ApiError::not_found("order", "Order not found"));
        }

        Ok(())
    }

    async fn list(
        &self,
        business_id: ObjectId,
        filter: OrderFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<OrderRecord>, u64)> {
        let collection = self.get_collection(business_id);
        let query = self.build_filter_query(&filter);

        let total = collection
            .count_documents(query.clone())
            .await
            .map_err(|e| ApiError::internal(format!("Failed to count orders: {}", e)))?;

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

        let mut orders = Vec::new();
        while let Some(order) = cursor
            .try_next()
            .await
            .map_err(|e| ApiError::internal(format!("Failed to read cursor: {}", e)))?
        {
            orders.push(order);
        }

        Ok((orders, total))
    }

    async fn bulk_update_status(
        &self,
        business_id: ObjectId,
        order_ids: Vec<ObjectId>,
        status: OrderStatus,
        note: Option<String>,
        created_by: Option<String>,
    ) -> ApiResult<u64> {
        let collection = self.get_collection(business_id);
        let result = collection
            .update_many(
                doc! { "_id": order_ids },
                doc! {
                    "$set": {
                        "status": to_bson(&status).unwrap(),
                        "updated_at": DateTime::now()
                    }
                },
            )
            .await
            .map_err(|e| ApiError::internal(format!("Failed to update orders: {}", e)))?;

        Ok(result.matched_count)
    }

    async fn get_customer_orders(
        &self,
        business_id: ObjectId,
        customer_email: &str,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<OrderRecord>, u64)> {
        todo!()
    }
}
