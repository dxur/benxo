use bigdecimal::BigDecimal;
use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::domain::*;
use crate::{types::id::Id, utils::serde_helpers::JsonOption};

#[derive(Debug, Clone, Deserialize, Serialize, o2o, TS)]
#[serde(rename_all = "snake_case")]
#[map_owned(OrderStatus)]
#[ghosts(Deleted: Self::Archived)]
#[ts(export)]
pub enum OrderStatusView {
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
    Archived,
}

#[derive(Debug, Clone, Deserialize, Serialize, o2o, TS)]
#[serde(rename_all = "snake_case")]
#[map_owned(PaymentStatus)]
#[ts(export)]
pub enum PaymentStatusView {
    Pending,
    Paid,
    Failed,
    Refunded,
    PartiallyRefunded,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct OrderItemCreate {
    pub product_id: Id,
    pub variant_sku: String,
    pub quantity: u32,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct OrderCreate {
    pub customer_email: String,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub items: Vec<OrderItemCreate>,
    pub shipping_address: ShippingAddress,
    pub billing_address: Option<ShippingAddress>,
    #[ts(as = "String")]
    pub shipping_cost: BigDecimal,
    #[ts(as = "String")]
    pub tax_amount: BigDecimal,
    pub currency: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export)]
#[from_owned(OrderHistory)]
pub struct OrderHistoryView {
    #[from(~.into())]
    pub status: OrderStatusView,
    pub note: Option<String>,
    #[from(~.map(From::from))]
    pub created_by: Option<Id>,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export)]
#[from_owned(OrderRecord)]
pub struct OrderView {
    #[from(@._id.into())]
    pub id: Id,
    pub customer_email: String,
    pub customer_name: String,
    pub customer_phone: Option<String>,
    pub items: Vec<OrderItem>,
    pub shipping_address: ShippingAddress,
    pub billing_address: Option<ShippingAddress>,
    #[from(~.into())]
    pub status: OrderStatusView,
    #[from(~.into())]
    pub payment_status: PaymentStatusView,
    #[ts(as = "String")]
    pub subtotal: BigDecimal,
    #[ts(as = "String")]
    pub shipping_cost: BigDecimal,
    #[ts(as = "String")]
    pub tax_amount: BigDecimal,
    #[ts(as = "String")]
    pub total_amount: BigDecimal,
    pub currency: String,
    pub notes: Option<String>,
    pub tracking_number: Option<String>,
    #[from(~.into_iter().map(Into::into).collect())]
    pub history: Vec<OrderHistoryView>,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct OrderListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<OrderStatusView>,
    pub payment_status: Option<PaymentStatusView>,
    pub customer_email: Option<String>,
    pub search: Option<String>,
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct OrderUpdate {
    pub status: JsonOption<OrderStatusView>,
    pub payment_status: JsonOption<PaymentStatusView>,
    pub tracking_number: JsonOption<String>,
    pub notes: JsonOption<String>,
    pub shipping_address: JsonOption<ShippingAddress>,
    pub billing_address: JsonOption<Option<ShippingAddress>>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct OrderStatusUpdate {
    pub status: OrderStatusView,
    pub note: Option<String>,
    pub created_by: Option<Id>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct OrderListResponse {
    pub orders: Vec<OrderView>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct OrderAnalytics {
    pub total_orders: u64,
    #[ts(as = "String")]
    pub total_revenue: BigDecimal,
    pub pending_orders: u64,
    pub completed_orders: u64,
    pub cancelled_orders: u64,
    #[ts(as = "String")]
    pub average_order_value: BigDecimal,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct BulkOrderStatusUpdate {
    pub order_ids: Vec<Id>,
    pub status: OrderStatusView,
    pub note: Option<String>,
    pub created_by: Option<String>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export)]
pub struct BulkUpdateResponse {
    pub updated_count: u64,
    pub failed_ids: Vec<Id>,
}

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub date_from: Option<DateTime<Utc>>,
    pub date_to: Option<DateTime<Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
}
