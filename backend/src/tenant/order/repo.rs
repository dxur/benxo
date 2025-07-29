use async_trait::async_trait;
use bson::{doc, oid::ObjectId, DateTime};
use mongodb::Client;

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
    ) -> ApiResult<u32>;

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
        todo!()
    }

    async fn update(
        &self,
        business_id: ObjectId,
        id: ObjectId,
        order: OrderRecord,
    ) -> ApiResult<OrderRecord> {
        todo!()
    }

    async fn list(
        &self,
        business_id: ObjectId,
        filter: OrderFilter,
        page: u32,
        limit: u32,
    ) -> ApiResult<(Vec<OrderRecord>, u64)> {
        todo!()
    }

    async fn bulk_update_status(
        &self,
        business_id: ObjectId,
        order_ids: Vec<ObjectId>,
        status: OrderStatus,
        note: Option<String>,
        created_by: Option<String>,
    ) -> ApiResult<u32> {
        todo!()
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
