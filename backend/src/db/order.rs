use bson::to_bson;
use field::*;
use indexmap::IndexMap;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use mongodb::options::TransactionOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::product::ProductVarInDb;
use super::*;
use super::{Error, FetchableInDb, FindableInDb, ModelInDb};
use crate::models::order::*;
use crate::models::product::ProductVar;

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInDb {
    pub _id: ObjectId,
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

impl From<OrderCreate> for OrderInDb {
    fn from(value: OrderCreate) -> Self {
        OrderInDb {
            _id: ObjectId::new(),
            status: OrderStatus::Pending,
            full_name: value.full_name,
            phone: value.phone,
            email: value.email,
            province: value.province,
            address: value.address,
            delivery: value.delivery,
            note: value.note,
            items: value.items,
            history: vec![OrderHistoryEntry {
                status: OrderStatus::Pending,
            }],
        }
    }
}

impl ModelInDb for Order {
    const COLLECTION_NAME: &'static str = "orders";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb = OrderInDb;
}

impl FindableInDb for Order {
    type FindInDb = FindInDb;
}

impl FetchableInDb for Order {
    type FetchInDb = OrderFetch;
}

impl CreatableInDb for Order {
    type CreateInDb = OrderCreate;

    async fn create(db: &Db, body: Self::Create) -> Result<Self::InDb> {
        if body.items.len() == 0 {
            return Err(Error {
                msg: "Order must have at least one item".to_string(),
            });
        }

        let mut session = db.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let products =
            db.collection::<<ProductVar as ModelInDb>::InDb>(ProductVar::COLLECTION_NAME);
        let orders = db.collection::<Self::InDb>(Self::COLLECTION_NAME);

        let mut products_qnt = HashMap::<String, u32>::new();

        for (sku, item) in &body.items {
            products_qnt
                .entry(sku.clone())
                .and_modify(|q| *q += item.quantity)
                .or_insert(item.quantity);
        }

        for (sku, qnt) in &products_qnt {
            let filter = doc! {
                field!(sku @ ProductVarInDb): sku,
                field!(stocks @ ProductVarInDb): { "$gte": qnt }
            };

            let product = products
                .find_one(filter)
                .session(&mut session)
                .await
                .map_err(|_| ())?;

            if product.is_none() {
                session.abort_transaction().await.map_err(|_| ())?;
                return Err(Error {
                    msg: "Not enough stocks".to_string(),
                });
            }
        }

        for (sku, item) in &body.items {
            let filter = doc! { field!(sku @ ProductVarInDb): &sku };
            let update =
                doc! { "$inc": { field!(stocks @ ProductVarInDb): -(item.quantity as i32) } };

            products
                .update_one(filter, update)
                .session(&mut session)
                .await
                .map_err(|_| ())?;
        }

        let order: Self::InDb = body.into();
        orders
            .insert_one(&order)
            .session(&mut session)
            .await
            .map_err(|_| ())?;

        session.commit_transaction().await.map_err(|_| ())?;

        Ok(order)
    }
}

impl UpdatableInDb for Order {
    type UpdateInDb = OrderUpdate;

    async fn update(db: &Db, body: Self::Update) -> Result<Option<(Self::UpdateInDb, Self::InDb)>> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let update = Self::UpdateInDb::from(body);
        let filter: Self::FindInDb = (&update).ref_into();

        let history = if let Some(status) = update.body.status {
            doc! {
                field! { history @ OrderInDb }: to_bson(&OrderHistoryEntry { status }).map_err(|e| Error { msg: format!("Failed to map status into document {}: {}", Self::COLLECTION_NAME, e) })?
            }
        } else {
            doc! {}
        };

        let res = db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one_and_update(
                Into::<Result<Document>>::into(&filter).map_err(|e| {
                    tracing::debug!("Failed to map into document: {}", Self::COLLECTION_NAME);
                    e
                })?,
                doc! {
                    "$set": RefInto::<Result<Document>>::ref_into(&update).map_err(|e| {
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
            Some(v) => Some((update, v)),
            None => None,
        })
    }
}

impl DeletableInDb for Order {
    type DeleteInDb = OrderDelete;

    // async fn delete(_: &Db, _: Self::Delete) -> Result<Option<(Self::DeleteInDb, Self::InDb)>> {
    //     todo!("Not implemented")
    // }
}
