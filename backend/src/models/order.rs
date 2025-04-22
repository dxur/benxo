use field::*;
use mongodb::bson::serde_helpers::serialize_object_id_as_hex_string;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use mongodb::options::TransactionOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::product::ProductVarModelInDb;

use super::product::ProductVarModel;
use super::Model;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum OrderStatus {
    Pending,
    Confirmed,
    UnConfirmed,
    Delivered,
    Done,
    Returned,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CartItem {
    pub product_sku: String,
    pub quantity: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelFetch {
    id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelCreate {
    full_name: String,
    items: Vec<CartItem>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelUpdate {
    #[serde(skip_serializing)]
    id: ObjectId,
    status: Option<OrderStatus>,
    full_name: Option<String>,
    items: Option<Vec<CartItem>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelDelete {
    id: ObjectId,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderModelPublic {
    #[serde(serialize_with = "serialize_object_id_as_hex_string")]
    id: ObjectId,
    full_name: String,
    items: Vec<CartItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrderModelInDb {
    pub _id: ObjectId,
    pub status: OrderStatus,
    pub full_name: String,
    pub items: Vec<CartItem>,
}

pub struct OrderModel;

impl Model for OrderModel {
    const COLLECTION_NAME: &'static str = "orders";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type ModelFetch = OrderModelFetch;
    type ModelFilter = ();
    type ModelCreate = OrderModelCreate;
    type ModelUpdate = OrderModelUpdate;
    type ModelDelete = OrderModelDelete;
    type ModelPublic = OrderModelPublic;
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
        let mut session = db.0.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let products = db
            .0
            .collection::<<ProductVarModel as Model>::ModelInDb>(ProductVarModel::COLLECTION_NAME);
        let orders = db.0.collection::<Self::ModelInDb>(Self::COLLECTION_NAME);

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
                field!(quantity @ ProductVarModelInDb): { "$gte": qnt }
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
            let update = doc! { "$inc": { field!(quantity @ ProductVarModelInDb): -(item.quantity as i32) } };

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
}
