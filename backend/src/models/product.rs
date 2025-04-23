pub use common::models::product::ProductModel;
use common::models::product::{ProductModelFetch, ProductVarModel};
use common::utils::validators::{non_negative, non_negative_option};
use field::*;
use mongodb::bson::to_document;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::TransactionOptions;
use serde::{Deserialize, Serialize};

use super::{Model, ModelFilter};
use crate::db::DB;
use crate::events::Event;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductModelInDb {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    #[serde(deserialize_with = "non_negative")]
    pub base_price: f32,
    #[serde(deserialize_with = "non_negative")]
    pub base_discount: f32,
    pub default_var: Option<ObjectId>,
    pub base_images: Vec<String>,
    pub slug: String,
}

impl Model for common::models::product::ProductModel {
    const COLLECTION_NAME: &'static str = "products";
    const UNIQUE_INDICES: &'static [&'static str] = &[field!(slug @ ProductModelInDb)];

    type ModelInDb = ProductModelInDb;

    fn fetch(body: Self::ModelFetch) -> Document {
        doc! {field!(_id @ ProductModelInDb): body.id}
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        Self::ModelInDb {
            _id: ObjectId::new(),
            name: body.name,
            description: body.description,
            featured: body.featured,
            category: body.category,
            base_price: body.base_price,
            base_discount: body.base_discount,
            default_var: None,
            base_images: body.base_images,
            slug: body.slug,
        }
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        Ok((
            doc! {field!(_id @ ProductModelInDb): body.id},
            to_document(body).map_err(|_| ())?,
        ))
    }

    fn delete(body: &Self::ModelDelete) -> Document {
        doc! {field!(_id @ ProductModelInDb): body.id}
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        Self::ModelPublic {
            id: body._id,
            name: body.name,
            description: body.description,
            featured: body.featured,
            category: body.category,
            base_price: body.base_price,
            base_discount: body.base_discount,
            base_images: body.base_images,
            slug: body.slug,
        }
    }

    async fn on_create(state: &AppState, body: &Self::ModelInDb) {
        state
            .event_bus
            .emit(Event::ProductCreated(body.clone()))
            .await;
    }

    async fn on_update(state: &AppState, _: &Self::ModelUpdate, value: &Self::ModelInDb) {
        state
            .event_bus
            .emit(Event::ProductUpdated(value.clone()))
            .await;
    }

    async fn on_delete(state: &AppState, _: Self::ModelDelete, value: &Self::ModelInDb) {
        state
            .event_bus
            .emit(Event::ProductDeleted(value.clone()))
            .await;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarModelInDb {
    pub sku: String,
    pub product_id: ObjectId,
    pub name: String,
    pub description: String,
    #[serde(deserialize_with = "non_negative_option")]
    pub price: Option<f32>,
    #[serde(deserialize_with = "non_negative_option")]
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: Vec<String>,
}

impl Model for ProductVarModel {
    const COLLECTION_NAME: &'static str = "variants";
    const UNIQUE_INDICES: &'static [&'static str] = &[field!(sku @ ProductVarModelInDb)];

    type ModelInDb = ProductVarModelInDb;

    fn fetch(body: Self::ModelFetch) -> Document {
        doc! {field!(sku @ ProductVarModelInDb): body.sku}
    }

    fn create(body: Self::ModelCreate) -> Self::ModelInDb {
        Self::ModelInDb {
            sku: body.sku,
            product_id: body.product_id,
            name: body.name,
            description: body.description,
            price: body.price,
            discount: body.discount,
            stocks: body.stocks,
            images: body.images,
            attrs: body.attrs,
        }
    }

    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()> {
        Ok((
            doc! {field!(sku @ ProductVarModelInDb): &body.sku},
            to_document(body).map_err(|_| ())?,
        ))
    }

    fn delete(body: &Self::ModelDelete) -> Document {
        doc! {field!(sku @ ProductVarModelInDb): &body.sku}
    }

    fn publish(body: Self::ModelInDb) -> Self::ModelPublic {
        Self::ModelPublic {
            sku: body.sku,
            product_id: body.product_id,
            name: body.name,
            description: body.description,
            price: body.price,
            discount: body.discount,
            stocks: body.stocks,
            images: body.images,
            attrs: body.attrs,
        }
    }

    async fn create_in_db(db: &DB, body: Self::ModelCreate) -> Result<Option<Self::ModelInDb>, ()> {
        let mut session = db.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let product_exists = db
            .collection::<<ProductModel as Model>::ModelInDb>(ProductModel::COLLECTION_NAME)
            .find_one(ProductModel::fetch(ProductModelFetch {
                id: body.product_id,
            }))
            .session(&mut session)
            .await
            .map_err(|_| ())?
            .is_some();

        if !product_exists {
            session.abort_transaction().await.ok();
            return Err(());
        }

        let model = Self::create(body);
        match db
            .collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
            .insert_one(&model)
            .session(&mut session)
            .await
        {
            Ok(_) => session
                .commit_transaction()
                .await
                .map_or_else(|_| Ok(None), |_| Ok(Some(model))),
            Err(_) => {
                session.abort_transaction().await.ok();
                Ok(None)
            }
        }
    }

    async fn on_create(state: &AppState, body: &Self::ModelInDb) {
        state
            .event_bus
            .emit(Event::ProductVarUpdated((
                body.sku.clone(),
                body.product_id,
            )))
            .await;
    }

    async fn on_update(state: &AppState, _: &Self::ModelUpdate, value: &Self::ModelInDb) {
        state
            .event_bus
            .emit(Event::ProductVarUpdated((
                value.sku.clone(),
                value.product_id,
            )))
            .await;
    }

    async fn on_delete(state: &AppState, _: Self::ModelDelete, value: &Self::ModelInDb) {
        state
            .event_bus
            .emit(Event::ProductVarDeleted((
                value.sku.clone(),
                value.product_id,
            )))
            .await;
    }
}

impl ModelFilter for ProductVarModel {
    fn filter(body: Self::ModelFilter) -> Document {
        doc! {field!(product_id @ ProductVarModelInDb): body.product_id}
    }
}
