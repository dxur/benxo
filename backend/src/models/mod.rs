// pub mod order;
pub mod product;
// pub mod settings;
// pub mod theme;
// pub mod user;

use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

use crate::{db::DB, AppState};

pub trait Model: common::models::Model {
    const COLLECTION_NAME: &'static str;
    const UNIQUE_INDICES: &'static [&'static str];

    type ModelInDb: Send + Sync + Serialize + for<'a> Deserialize<'a>;

    fn fetch(body: Self::ModelFetch) -> Document;
    fn create(body: Self::ModelCreate) -> Self::ModelInDb;
    fn update(body: &Self::ModelUpdate) -> Result<(Document, Document), ()>;
    fn delete(body: &Self::ModelDelete) -> Document;
    fn publish(body: Self::ModelInDb) -> Self::ModelPublic;

    async fn on_create(_: &AppState, _: &Self::ModelInDb) {}
    async fn on_update(_: &AppState, _: &Self::ModelUpdate, _: &Self::ModelInDb) {}
    async fn on_delete(_: &AppState, _: Self::ModelDelete, _: &Self::ModelInDb) {}

    async fn init_coll_in_db(db: &DB) -> Result<(), String> {
        if Self::UNIQUE_INDICES.len() == 0 {
            return Ok(());
        }

        let mut keys_doc = doc! {};

        for key in Self::UNIQUE_INDICES {
            keys_doc.insert(key.to_string(), 1);
        }

        db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
            .create_index(
                IndexModel::builder()
                    .keys(keys_doc)
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .map_or_else(|e| Err(e.to_string()), |_| Ok(()))
    }

    async fn create_in_db(db: &DB, body: Self::ModelCreate) -> Result<Option<Self::ModelInDb>, ()> {
        let model = Self::create(body);
        match db
            .collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
            .insert_one(&model)
            .await
        {
            Ok(_) => Ok(Some(model)),
            Err(_) => Ok(None),
        }
    }

    async fn get_all_in_db(db: &DB, limit: usize, offset: usize) -> Option<Vec<Self::ModelInDb>> {
        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset as u64)
            .build();

        Some(
            db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
                .find(doc! {})
                .with_options(find_options)
                .await
                .ok()?
                .try_collect()
                .await
                .ok()?,
        )
    }

    async fn get_one_in_db(db: &DB, body: Self::ModelFetch) -> Result<Option<Self::ModelInDb>, ()> {
        db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
            .find_one(Self::fetch(body))
            .await
            .map_err(|_| ())
    }

    async fn update_in_db(
        db: &DB,
        body: &Self::ModelUpdate,
    ) -> Result<Option<Self::ModelInDb>, ()> {
        let res = Self::update(body)?;

        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
            .find_one_and_update(res.0, doc! {"$set": res.1})
            .with_options(options)
            .await
            .map_err(|_| ())
    }

    async fn delete_in_db(
        db: &DB,
        body: &Self::ModelDelete,
    ) -> Result<Option<Self::ModelInDb>, ()> {
        db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
            .find_one_and_delete(Self::delete(body))
            .await
            .map_err(|_| ())
    }
}

pub trait ModelFilter: Model + common::models::ModelFilter {
    fn filter(_: Self::ModelFilter) -> Document;

    async fn get_some_in_db(
        db: &DB,
        body: Self::ModelFilter,
        limit: usize,
        offset: usize,
    ) -> Option<Vec<Self::ModelInDb>> {
        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset as u64)
            .build();

        Some(
            db.collection::<Self::ModelInDb>(Self::COLLECTION_NAME)
                .find(Self::filter(body))
                .with_options(find_options)
                .await
                .ok()?
                .try_collect()
                .await
                .ok()?,
        )
    }
}
