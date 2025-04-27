pub mod order;
pub mod product;
// pub mod settings;
// pub mod theme;
pub mod user;

use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::IndexModel;
use serde::{Deserialize, Serialize};

use crate::utils::error::*;
use crate::utils::types::{RefInto, Result};
use crate::{db::DB, AppState};

pub trait Model: common::models::Model {
    const COLLECTION_NAME: &'static str;
    const UNIQUE_INDICES: &'static [&'static str];

    type InDb: Send + Sync + Serialize + for<'a> Deserialize<'a> + Into<Self::Public>;

    async fn init_coll(db: &DB) -> Result<()> {
        if Self::UNIQUE_INDICES.len() == 0 {
            return Ok(());
        }

        let mut keys_doc = doc! {};

        for key in Self::UNIQUE_INDICES {
            keys_doc.insert(key.to_string(), 1);
        }

        db.collection::<Self::InDb>(Self::COLLECTION_NAME)
            .create_index(
                IndexModel::builder()
                    .keys(keys_doc)
                    .options(IndexOptions::builder().unique(true).build())
                    .build(),
            )
            .await
            .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))
    }

    async fn get_all(db: &DB, limit: usize, offset: usize) -> Result<(usize, Vec<Self::InDb>)> {
        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset as u64)
            .build();

        let coll = db.collection::<Self::InDb>(Self::COLLECTION_NAME);

        let total = coll.count_documents(doc! {}).await.map_err(|e| {
            tracing::debug!("Failed to count {}: {}", Self::COLLECTION_NAME, e);
            ()
        })? as usize;

        Ok((
            total,
            db.collection::<Self::InDb>(Self::COLLECTION_NAME)
                .find(doc! {})
                .with_options(find_options)
                .await
                .map_err(|e| {
                    tracing::debug!("Failed to find {}: {}", Self::COLLECTION_NAME, e);
                    ()
                })?
                .try_collect()
                .await
                .map_err(|e| {
                    tracing::debug!("Failed to collect {}: {}", Self::COLLECTION_NAME, e);
                    ()
                })?,
        ))
    }
}

pub trait Findable: Model {
    type FindInDb: Serialize + RefInto<Result<Document>>;
}

pub trait Creatable: Model + common::models::Creatable {
    type CreateInDb: Send + Sync + Serialize + From<Self::Create> + Into<Self::InDb>;

    async fn on_create(_: &AppState, _: &Self::InDb) {}

    async fn create(db: &DB, body: Self::Create) -> Result<Self::InDb> {
        let model: Self::InDb = Self::CreateInDb::from(body).into();
        match db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .insert_one(&model)
            .await
        {
            Ok(_) => Ok(model),
            Err(e) => {
                tracing::debug!("Failed to create {}: {}", Self::COLLECTION_NAME, e);
                Err(().into())
            }
        }
    }
}

pub trait Fetchable: Findable + common::models::Fetchable {
    type FetchInDb: Send + Sync + Serialize + From<Self::Fetch> + RefInto<Self::FindInDb>;

    async fn get_one(db: &DB, body: Self::Fetch) -> Result<Option<Self::InDb>> {
        db.collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one(
                Self::FetchInDb::from(body.into())
                    .ref_into()
                    .ref_into()
                    .map_err(|e| {
                        tracing::debug!(
                            "Failed to map into document {}: {}",
                            Self::COLLECTION_NAME,
                            e
                        );
                    })?,
            )
            .await
            .map_err(|e| {
                tracing::debug!("Failed to find {}: {}", Self::COLLECTION_NAME, e);
                ().into()
            })
    }
}

pub trait Updatable: Findable + common::models::Updatable {
    type UpdateInDb: Send
        + Sync
        + Serialize
        + From<Self::Update>
        + RefInto<Self::FindInDb>
        + RefInto<Result<Document>>;

    async fn on_update(_: &AppState, _: &Self::UpdateInDb, _: &Self::InDb) {}

    async fn update(db: &DB, body: Self::Update) -> Result<Option<(Self::UpdateInDb, Self::InDb)>> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let update = Self::UpdateInDb::from(body);
        let filter: Self::FindInDb = (&update).ref_into();

        let res = db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one_and_update(
                filter.ref_into().map_err(|e| {
                    tracing::debug!("Failed to map into document: {}", Self::COLLECTION_NAME);
                    e
                })?,
                doc! {"$set": RefInto::<Result<Document>>::ref_into(&update).map_err(|e| {
                    tracing::debug!("Failed to map into document {}: {}", Self::COLLECTION_NAME, e);
                    e
                })?},
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

pub trait Deletable: Findable + common::models::Deletable {
    type DeleteInDb: Send + Sync + From<Self::Delete> + RefInto<Self::FindInDb>;

    async fn on_delete(_: &AppState, _: &Self::DeleteInDb, _: &Self::InDb) {}

    async fn delete(db: &DB, body: Self::Delete) -> Result<Option<(Self::DeleteInDb, Self::InDb)>> {
        let model = Self::DeleteInDb::from(body.into());
        let res = db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one_and_delete(model.ref_into().ref_into().map_err(|e| {
                tracing::debug!(
                    "Failed to map into document {}: {}",
                    Self::COLLECTION_NAME,
                    e
                );
                e
            })?)
            .await
            .map_err(|e| Error { msg: e.to_string() })?;
        Ok(res.map(|v| (model, v)))
    }
}

pub trait Filterable: Model + common::models::Filterable {
    type FilterInDb: Send + Sync + Serialize + From<Self::Filter> + RefInto<Result<Document>>;

    async fn get_some(
        db: &DB,
        body: Self::Filter,
        limit: usize,
        offset: usize,
    ) -> Result<(usize, Vec<Self::InDb>)> {
        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset as u64)
            .build();

        let filter: Document = Self::FilterInDb::from(body).ref_into().map_err(|e| {
            tracing::debug!(
                "Failed to map into document {}: {}",
                Self::COLLECTION_NAME,
                e
            );
            e
        })?;
        let coll = db.collection::<Self::InDb>(Self::COLLECTION_NAME);

        let total = coll.count_documents(filter.clone()).await.map_err(|e| {
            tracing::debug!("Failed to count {}: {}", Self::COLLECTION_NAME, e);
            ()
        })? as usize;

        Ok((
            total,
            db.collection::<Self::InDb>(Self::COLLECTION_NAME)
                .find(filter)
                .with_options(find_options)
                .await
                .map_err(|e| {
                    tracing::debug!("Failed to find {}: {}", Self::COLLECTION_NAME, e);
                    ()
                })?
                .try_collect()
                .await
                .map_err(|e| {
                    tracing::debug!("Failed to collect {}: {}", Self::COLLECTION_NAME, e);
                    ()
                })?,
        ))
    }
}
