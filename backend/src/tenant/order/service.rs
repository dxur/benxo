use bigdecimal::BigDecimal;
use bson::DateTime;
use chrono::Utc;

use super::api::*;
use super::domain::*;
use super::repo::OrderRepo;
use crate::platform::business::api::BusinessSession;
use crate::tenant::product::domain::ProductVariant;
use crate::tenant::product::repo::ProductRepo;
use crate::tenant::product::service::ProductService;
use crate::types::id::Id;
use crate::utils::error::{ApiError, ApiResult};

pub struct OrderService<R: OrderRepo> {
    repo: R,
}

impl<R: OrderRepo> OrderService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn create_order<P: ProductRepo>(
        &self,
        product_service: &ProductService<P>,
        business: BusinessSession,
        create_req: OrderCreate,
    ) -> ApiResult<OrderDto> {
        let mut order_items = Vec::new();
        let mut subtotal = BigDecimal::from(0);

        // TODO: wrap inside a transaction
        for item_req in create_req.items {
            let product = product_service
                .get_product(business.clone(), item_req.product_id)
                .await?;

            let variant: &ProductVariant = product
                .variants
                .iter()
                .find(|v| v.sku == item_req.variant_sku)
                .ok_or_else(|| {
                    ApiError::forbidden(
                        "order",
                        format!("Variant with SKU '{}' not found", item_req.variant_sku),
                    )
                })?;

            if variant.stocks < item_req.quantity as usize {
                return Err(ApiError::forbidden(
                    "order",
                    format!(
                        "Insufficient stock for variant '{}'. Available: {}, Requested: {}",
                        item_req.variant_sku, variant.stocks, item_req.quantity
                    ),
                ));
            }

            let unit_price = variant.price.clone();
            let total_price = &unit_price * BigDecimal::from(item_req.quantity);
            subtotal += &total_price;

            order_items.push(OrderItem {
                variant_sku: item_req.variant_sku,
                product_title: product.title.to_string(),
                quantity: item_req.quantity,
                unit_price,
                total_price,
            });
        }

        // Create order record
        let mut order = OrderRecord {
            customer_email: create_req.customer_email,
            customer_name: create_req.customer_name,
            customer_phone: create_req.customer_phone,
            items: order_items,
            shipping_address: create_req.shipping_address,
            billing_address: create_req.billing_address,
            subtotal,
            shipping_cost: create_req.shipping_cost,
            tax_amount: create_req.tax_amount,
            currency: create_req.currency.unwrap_or_else(|| "USD".to_string()),
            notes: create_req.notes,
            ..Default::default()
        };

        order.add_history_entry(
            OrderStatus::Pending,
            None,
            Some(Source::User(business.user_id.into())),
        );

        order.calculate_totals();

        self.repo
            .create(business.business_id.into(), order)
            .await
            .map(Into::into)
    }

    pub async fn get_order(&self, business: BusinessSession, order_id: Id) -> ApiResult<OrderDto> {
        let id = order_id.into_inner();
        self.repo
            .find_by_id(business.business_id.into_inner(), id)
            .await?
            .ok_or_else(|| ApiError::not_found("order", id.to_hex()))
            .map(Into::into)
    }

    pub async fn update_order(
        &self,
        business: BusinessSession,
        order_id: Id,
        update_req: OrderUpdate,
    ) -> ApiResult<OrderDto> {
        let id = order_id.into_inner();
        let business_id = business.business_id.into_inner();

        let mut order = self
            .repo
            .find_by_id(business_id, id)
            .await?
            .ok_or_else(|| ApiError::not_found("order", id.to_hex()))?;

        // Apply updates
        update_req.status.map(|v| {
            order.add_history_entry(v.into(), Some("Status updated".to_string()), None);
        });
        // update_req
        //     .payment_status
        //     .map(|v| order.payment_status = v.into());
        update_req
            .tracking_number
            .map(|v| order.tracking_number = Some(v));
        update_req.notes.map(|v| order.notes = Some(v));
        update_req
            .shipping_address
            .map(|v| order.shipping_address = v);
        update_req
            .billing_address
            .map(|v| order.billing_address = v);

        self.repo
            .update(business_id, id, order)
            .await
            .map(Into::into)
    }

    pub async fn update_order_status(
        &self,
        business: BusinessSession,
        order_id: Id,
        status_update: OrderStatusUpdate,
    ) -> ApiResult<OrderDto> {
        let id = order_id.into_inner();
        let business_id = business.business_id.into_inner();

        let mut order = self
            .repo
            .find_by_id(business_id, id)
            .await?
            .ok_or_else(|| ApiError::not_found("order", id.to_hex()))?;

        order.add_history_entry(
            status_update.status.into(),
            status_update.note,
            Some(Source::User(business.user_id.into())),
        );

        self.repo
            .update(business_id, id, order)
            .await
            .map(Into::into)
    }

    pub async fn list_orders(
        &self,
        business: BusinessSession,
        query: OrderListQuery,
    ) -> ApiResult<OrderListResponse> {
        let OrderListQuery {
            page,
            limit,
            status,
            // payment_status,
            customer_email,
            search,
            date_from,
            date_to,
        } = query;

        let filter = OrderFilter {
            status: status.map(Into::into),
            // payment_status: payment_status.map(Into::into),
            customer_email,
            search,
            date_from: date_from.map(DateTime::from_chrono),
            date_to: date_to.map(DateTime::from_chrono),
        };

        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let (orders, total) = self
            .repo
            .list(business.business_id.into_inner(), filter, page, limit)
            .await?;

        let views: Vec<_> = orders.into_iter().map(Into::into).collect();
        Ok(OrderListResponse {
            orders: views,
            total,
            page,
            limit,
        })
    }

    pub async fn bulk_update_order_status(
        &self,
        business: BusinessSession,
        bulk_update: BulkOrderStatusUpdate,
    ) -> ApiResult<BulkUpdateResponse> {
        let order_ids: Vec<_> = bulk_update
            .order_ids
            .into_iter()
            .map(|id| id.into_inner())
            .collect();

        let updated_count = self
            .repo
            .bulk_update_status(
                business.business_id.into_inner(),
                order_ids,
                bulk_update.status.into(),
                bulk_update.note,
                bulk_update.created_by,
            )
            .await?;

        Ok(BulkUpdateResponse {
            updated_count,
            failed_ids: vec![], // In a real implementation, you'd track failures
        })
    }

    pub async fn get_analytics(
        &self,
        business: BusinessSession,
        date_from: Option<chrono::DateTime<Utc>>,
        date_to: Option<chrono::DateTime<Utc>>,
    ) -> ApiResult<OrderAnalytics> {
        self.repo
            .get_analytics(
                business.business_id.into_inner(),
                date_from.map(DateTime::from_chrono),
                date_to.map(DateTime::from_chrono),
            )
            .await
    }

    pub async fn get_customer_orders(
        &self,
        business: BusinessSession,
        customer_email: String,
        page: Option<u32>,
        limit: Option<u32>,
    ) -> ApiResult<OrderListResponse> {
        let page = page.unwrap_or(1);
        let limit = limit.unwrap_or(10);

        let (orders, total) = self
            .repo
            .get_customer_orders(
                business.business_id.into_inner(),
                &customer_email,
                page,
                limit,
            )
            .await?;

        let views: Vec<_> = orders.into_iter().map(Into::into).collect();
        Ok(OrderListResponse {
            orders: views,
            total,
            page,
            limit,
        })
    }
}
