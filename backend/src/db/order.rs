use bson::{to_bson, DateTime};
use field::*;
use indexmap::IndexMap;
use macros::Model;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;
use super::{Error, FetchableInDb, FindableInDb, ModelInDb};
use crate::models::order::*;
use crate::register_model;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInDb {
    pub _id: ObjectId,
    pub business_id: ObjectId,
    pub status: OrderStatus,
    pub full_name: String,
    pub phone: String,
    pub email: String,
    pub province: String,
    pub address: String,
    pub delivery: DeliveryType,
    pub note: String,
    pub items: IndexMap<String, CartItem>,
    pub history: Vec<OrderHistoryEntry>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Into<OrderPublic> for OrderInDb {
    fn into(self) -> OrderPublic {
        OrderPublic {
            id: self._id,
            status: self.status,
            full_name: self.full_name,
            phone: self.phone,
            email: self.email,
            province: self.province,
            address: self.address,
            delivery: self.delivery,
            note: self.note,
            items: self.items,
            history: self.history,
        }
    }
}

impl Into<Result<Document>> for &OrderUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<FindInDb> for &OrderFetch {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<FindInDb> for &OrderDelete {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<FindInDb> for &OrderUpdate {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl From<ByBusinessId<OrderCreate>> for OrderInDb {
    fn from(value: ByBusinessId<OrderCreate>) -> Self {
        let ByBusinessId { business_id, body } = value;
        OrderInDb {
            _id: ObjectId::new(),
            business_id,
            status: OrderStatus::Pending,
            full_name: body.full_name,
            phone: body.phone,
            email: body.email,
            province: body.province,
            address: body.address,
            delivery: body.delivery,
            note: body.note,
            items: body.items,
            history: vec![OrderHistoryEntry {
                status: OrderStatus::Pending,
                time: DateTime::now().timestamp_millis(),
            }],
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

#[derive(Model)]
#[model(
    find=ByBusinessId<FindInDb>,
    fetch=ByBusinessId<OrderFetch>,
    list=ByBusinessId<Void>,
    create=ByBusinessId<OrderCreate>,
    delete=ByBusinessId<OrderDelete>,
)]
pub struct Order;

register_model!(Order);
impl ModelInDb for Order {
    const COLLECTION_NAME: &'static str = "orders";
    const UNIQUE_INDICES: &'static [(&'static [&'static str], bool)] = &[
        (&[field!(business_id @ OrderInDb)], false),
        (
            &[field!(business_id @ OrderInDb), field!(_id @ OrderInDb)],
            false,
        ),
    ];

    type InDb = OrderInDb;
}

impl UpdatableInDb for Order {
    type UpdateInDb = ByBusinessId<OrderUpdate>;

    async fn update(
        db: &Db,
        body: Self::UpdateInDb,
    ) -> Result<Option<(Self::UpdateInDb, Self::InDb)>> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let filter: Self::FindInDb = body.ref_into();

        let history = if let Some(status) = body.body.body.status {
            let time = DateTime::now().timestamp_millis();
            doc! {
                field! { history @ OrderInDb }: to_bson(&OrderHistoryEntry { status, time }).map_err(|e| Error { msg: format!("Failed to map status into document {}: {}", Self::COLLECTION_NAME, e) })?
            }
        } else {
            doc! {}
        };

        let res = db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one_and_update(
                filter.into_filter().map_err(|e| {
                    tracing::debug!("Failed to map into document: {}", Self::COLLECTION_NAME);
                    e
                })?,
                doc! {
                    "$set": RefInto::<Result<Document>>::ref_into(&body).map_err(|e| {
                        tracing::debug!("Failed to map into document {}: {}", Self::COLLECTION_NAME, e);
                        e
                    })?,
                    "$push": history
                },
            )
            .with_options(options)
            .await
            .map_err(|e| {
                tracing::debug!("Failed to find {}: {}", Self::COLLECTION_NAME, e);
                Error { msg: e.to_string() }
            })?;
        Ok(match res {
            Some(v) => Some((body, v)),
            None => None,
        })
    }
}
