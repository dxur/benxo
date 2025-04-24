use common::models::order::*;
use common::models::product::ProductVarModel;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use mongodb::options::TransactionOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::product::ProductVarModelInDb;
use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderModelInDb {
    pub _id: ObjectId,
    pub status: OrderStatus,
    pub full_name: String,
    pub items: Vec<CartItem>,
}

impl Model for OrderModel {
    const COLLECTION_NAME: &'static str = "orders";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelInDb = OrderModelInDb;

    fn fetch(body: Self::ModelFetch) -> Document {
        doc! {field!(_id @ OrderModelInDb): body.id}
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        Self::ModelInDb {
            _id: ObjectId::new(),
            status: OrderStatus::Pending,
            full_name: body.full_name,
            items: body.items,
        }
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        Ok((
            doc! {field!(_id @ OrderModelInDb): body.id},
            to_document(body).map_err(|_| ())?,
        ))
    }

    fn delete(body: &Self::ModelDelete) -> Document {
        doc! {field!(_id @ OrderModelInDb): body.id}
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        Self::ModelPublic {
            id: body._id,
            full_name: body.full_name,
            items: body.items,
        }
    }

    async fn create_in_db(
        db: &crate::db::DB,
        body: Self::ModelCreate,
    ) -> Result<Option<Self::ModelInDb>, ()> {
        let mut session = db.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let products = db
            .collection::<<ProductVarModel as Model>::ModelInDb>(ProductVarModel::COLLECTION_NAME);
        let orders = db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME);

        let mut products_qnt = HashMap::<String, u32>::new();

        for item in &body.items {
            products_qnt
                .entry(item.product_sku.clone())
                .and_modify(|q| *q += item.quantity)
                .or_insert(item.quantity);
        }

        for (sku, qnt) in &products_qnt {
            let filter = doc! {
                field!(sku @ ProductVarModelInDb): sku,
                field!(stocks @ ProductVarModelInDb): { "$gte": qnt }
            };

            let product = products
                .find_one(filter)
                .session(&mut session)
                .await
                .map_err(|_| ())?;

            if product.is_none() {
                session.abort_transaction().await.map_err(|_| ())?;
                return Err(());
            }
        }

        for item in &body.items {
            let filter = doc! { field!(sku @ ProductVarModelInDb): &item.product_sku };
            let update =
                doc! { "$inc": { field!(stocks @ ProductVarModelInDb): -(item.quantity as i32) } };

            products
                .update_one(filter, update)
                .session(&mut session)
                .await
                .map_err(|_| ())?;
        }

        let order = Self::create(body);
        orders
            .insert_one(&order)
            .session(&mut session)
            .await
            .map_err(|_| ())?;

        session.commit_transaction().await.map_err(|_| ())?;

        Ok(Some(order))
    }

    async fn update_in_db(
        _: &crate::db::DB,
        _: &Self::ModelUpdate,
    ) -> Result<Option<Self::ModelInDb>, ()> {
        todo!("recalculate the stocks")
    }

    async fn delete_in_db(
        _: &crate::db::DB,
        _: &Self::ModelDelete,
    ) -> Result<Option<Self::ModelInDb>, ()> {
        todo!("release the stocks")
    }
}
