use common::models::order::*;
use common::models::product::ProductVar;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use mongodb::options::TransactionOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::product::ProductVarInDb;
use super::*;
use super::{Error, Fetchable, Findable, Model};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderInDb {
    pub _id: ObjectId,
    pub status: OrderStatus,
    pub full_name: String,
    pub items: Vec<CartItem>,
}

#[derive(Debug, Serialize)]
pub struct OrderFindInDb(pub OrderFetch);

#[derive(Debug, Serialize)]
pub struct OrderUpdateInDb(pub OrderUpdate);

impl Into<OrderPublic> for OrderInDb {
    fn into(self) -> OrderPublic {
        OrderPublic {
            id: self._id,
            full_name: self.full_name,
            items: self.items,
        }
    }
}

impl From<OrderUpdate> for OrderUpdateInDb {
    fn from(value: OrderUpdate) -> Self {
        OrderUpdateInDb(value)
    }
}

impl Into<Result<Document>> for &OrderFindInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<Result<Document>> for &OrderUpdateInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<OrderFindInDb> for &OrderFetch {
    fn into(self) -> OrderFindInDb {
        OrderFindInDb(OrderFetch { id: self.id })
    }
}

impl Into<OrderFindInDb> for &OrderDelete {
    fn into(self) -> OrderFindInDb {
        OrderFindInDb(OrderFetch { id: self.id })
    }
}

impl Into<OrderFindInDb> for &OrderUpdateInDb {
    fn into(self) -> OrderFindInDb {
        OrderFindInDb(OrderFetch { id: self.0.id })
    }
}

impl From<OrderCreate> for OrderInDb {
    fn from(value: OrderCreate) -> Self {
        OrderInDb {
            _id: ObjectId::new(),
            status: OrderStatus::Pending,
            full_name: value.full_name,
            items: value.items,
        }
    }
}

impl Model for Order {
    const COLLECTION_NAME: &'static str = "orders";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb = OrderInDb;
}

impl Findable for Order {
    type FindInDb = OrderFindInDb;
}

impl Fetchable for Order {
    type FetchInDb = OrderFetch;
}

impl Creatable for Order {
    type CreateInDb = OrderCreate;

    async fn create(db: &DB, body: Self::Create) -> Result<Self::InDb> {
        let mut session = db.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let products = db.collection::<<ProductVar as Model>::InDb>(ProductVar::COLLECTION_NAME);
        let orders = db.collection::<Self::InDb>(Self::COLLECTION_NAME);

        let mut products_qnt = HashMap::<String, u32>::new();

        for item in &body.items {
            products_qnt
                .entry(item.product_sku.clone())
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

        for item in &body.items {
            let filter = doc! { field!(sku @ ProductVarInDb): &item.product_sku };
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

impl Updatable for Order {
    type UpdateInDb = OrderUpdateInDb;

    async fn update(_: &DB, _: Self::Update) -> Result<Option<(Self::UpdateInDb, Self::InDb)>> {
        todo!("Not implemented")
    }
}

impl Deletable for Order {
    type DeleteInDb = OrderDelete;

    async fn delete(_: &DB, _: Self::Delete) -> Result<Option<(Self::DeleteInDb, Self::InDb)>> {
        todo!("Not implemented")
    }
}
