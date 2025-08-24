use bigdecimal::BigDecimal;
use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::name::Name;

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    #[default]
    Pending,
    Confirmed,
    Processing,
    Shipped,
    Delivered,
    Cancelled,
    Refunded,
    Archived,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentStatus {
    #[default]
    Pending,
    Paid,
    Failed,
    Refunded,
    PartiallyRefunded,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
pub struct OrderItem {
    pub variant_sku: String,
    pub product_title: String,
    pub quantity: u32,
    #[ts(as = "String")]
    pub unit_price: BigDecimal,
    #[ts(as = "String")]
    pub total_price: BigDecimal,
}

#[derive(Debug, Clone, Deserialize, Serialize, TS)]
#[ts(export, bound = "")]
pub struct ShippingAddress {
    pub full_name: String,
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub postal_code: String,
    pub country: String,
    pub phone: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Source {
    Store(ObjectId),
    User(ObjectId),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderHistory {
    pub status: OrderStatus,
    pub note: Option<String>,
    pub created_by: Option<Source>,
    pub created_at: DateTime,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct OrderRecord {
    pub _id: ObjectId,
    pub customer_email: Option<String>,
    pub customer_name: String,
    pub customer_phone: String,
    pub items: Vec<OrderItem>,
    pub shipping_address: ShippingAddress,
    pub billing_address: Option<ShippingAddress>,
    pub status: OrderStatus,
    // pub payment_status: PaymentStatus,
    pub subtotal: BigDecimal,
    pub shipping_cost: BigDecimal,
    pub tax_amount: BigDecimal,
    pub total_amount: BigDecimal,
    pub currency: String,
    pub notes: Option<String>,
    pub tracking_number: Option<String>,
    pub history: Vec<OrderHistory>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Default for OrderRecord {
    fn default() -> Self {
        let now = DateTime::now();

        Self {
            _id: Default::default(),
            customer_email: Default::default(),
            customer_name: Default::default(),
            customer_phone: Default::default(),
            items: Default::default(),
            shipping_address: ShippingAddress {
                full_name: Default::default(),
                address_line_1: Default::default(),
                address_line_2: Default::default(),
                city: Default::default(),
                state: Default::default(),
                postal_code: Default::default(),
                country: Default::default(),
                phone: Default::default(),
            },
            billing_address: Default::default(),
            status: Default::default(),
            // payment_status: Default::default(),
            subtotal: BigDecimal::from(0),
            shipping_cost: BigDecimal::from(0),
            tax_amount: BigDecimal::from(0),
            total_amount: BigDecimal::from(0),
            currency: "USD".to_string(),
            notes: Default::default(),
            tracking_number: Default::default(),
            history: Default::default(),
            created_at: now,
            updated_at: now,
        }
    }
}

impl OrderRecord {
    pub fn is_completed(&self) -> bool {
        matches!(self.status, OrderStatus::Delivered)
    }

    pub fn can_be_cancelled(&self) -> bool {
        matches!(self.status, OrderStatus::Pending | OrderStatus::Confirmed)
    }

    pub fn add_history_entry(
        &mut self,
        status: OrderStatus,
        note: Option<String>,
        created_by: Option<Source>,
    ) {
        self.history.push(OrderHistory {
            status: status.clone(),
            note,
            created_by,
            created_at: DateTime::now(),
        });
        self.status = status;
        self.updated_at = DateTime::now();
    }

    pub fn calculate_totals(&mut self) {
        self.subtotal = self.items.iter().map(|item| &item.total_price).sum();

        self.total_amount = &self.subtotal + &self.shipping_cost + &self.tax_amount;
    }
}

#[derive(Debug, Clone, Default)]
pub struct OrderFilter {
    pub status: Option<OrderStatus>,
    // pub payment_status: Option<PaymentStatus>,
    pub customer_email: Option<String>,
    pub search: Option<String>,
    pub date_from: Option<DateTime>,
    pub date_to: Option<DateTime>,
}
