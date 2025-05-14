pub mod order;
pub mod product;
pub mod settings;
// pub mod theme;
pub mod user;

use bson::oid::ObjectId;
use bson::to_document;
use futures::TryStreamExt;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::IndexModel;
use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::ops::Deref;

use crate::models::*;
use crate::utils::error::*;
use crate::utils::types::{HaveContext, IntoInner, RefInto, Result};
use crate::AppState;

pub const DEFAULT_UUID: ObjectId = ObjectId::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

pub struct Db(Database);

impl Db {
    pub async fn new(uri: &str, name: &str) -> Self {
        let client_options = ClientOptions::parse(uri).await.unwrap();
        Db(Client::with_options(client_options).unwrap().database(name))
    }
}

impl Deref for Db {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait ModelInDb: Model {
    const COLLECTION_NAME: &'static str;
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a> + Into<Self::Public>;

    async fn init_coll(db: &Db) -> Result<()> {
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
}

pub trait IntoFilter {
    fn into_filter(&self) -> Result<Document>;
}

#[derive(Debug, Serialize)]
pub struct FindInDb {
    pub _id: ObjectId,
}

#[derive(Debug, Serialize)]
pub struct ByStoreId<T> {
    pub store_id: String,
    #[serde(skip)]
    pub body: T,
}

impl IntoFilter for ByStoreId<FindInDb> {
    fn into_filter(&self) -> Result<Document> {
        let d = to_document(&self.body).map_err(|e| Error { msg: e.to_string() })?;
        let mut res = to_document(self).map_err(|e| Error { msg: e.to_string() })?;
        res.extend(d);
        Ok(res)
    }
}

impl IntoFilter for ByStoreId<Void> {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl<T> Into<Result<Document>> for &ByStoreId<T>
where
    T: RefInto<Result<Document>>,
{
    fn into(self) -> Result<Document> {
        self.body.ref_into()
    }
}

impl<T> From<HaveContext<T, String>> for ByStoreId<T> {
    fn from(value: HaveContext<T, String>) -> Self {
        let HaveContext(body, store_id) = value;
        Self { store_id, body }
    }
}

impl<T, U> Into<ByStoreId<U>> for &ByStoreId<T>
where
    T: RefInto<U>,
{
    fn into(self) -> ByStoreId<U> {
        ByStoreId {
            store_id: self.store_id.clone(),
            body: self.body.ref_into(),
        }
    }
}

impl IntoFilter for FindInDb {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

pub trait FindableInDb: ModelInDb {
    type FindInDb: Debug + Serialize + IntoFilter;
}

pub trait CreatableInDb: ModelInDb + Creatable {
    type CreateInDb: Debug + Send + Sync + Serialize + IntoInner<Self::InDb>;

    async fn on_create(_: &AppState, _: &Self::InDb) {}

    async fn create(db: &Db, body: Self::CreateInDb) -> Result<Self::InDb> {
        let model: Self::InDb = body.into_inner();
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

pub trait ListableInDb: ModelInDb {
    type ListInDb: Debug + Send + Sync + IntoFilter;

    async fn get_all(
        db: &Db,
        body: Self::ListInDb,
        limit: usize,
        offset: usize,
    ) -> Result<(usize, Vec<Self::InDb>)> {
        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset as u64)
            .build();

        let coll = db.collection::<Self::InDb>(Self::COLLECTION_NAME);
        let filter = body.into_filter()?;

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

pub trait FetchableInDb: FindableInDb + Fetchable {
    type FetchInDb: Debug + Send + Sync + RefInto<Self::FindInDb>;

    async fn get_one(db: &Db, body: Self::FetchInDb) -> Result<Option<Self::InDb>> {
        db.collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one(body.ref_into().into_filter().map_err(|e| {
                tracing::debug!(
                    "Failed to map into document {}: {}",
                    Self::COLLECTION_NAME,
                    e
                );
            })?)
            .await
            .map_err(|e| {
                tracing::debug!("Failed to find {}: {}", Self::COLLECTION_NAME, e);
                ().into()
            })
    }
}

pub trait UpdatableInDb: FindableInDb + Updatable {
    type UpdateInDb: Send + Debug + Sync + RefInto<Self::FindInDb> + RefInto<Result<Document>>;

    async fn on_update(_: &AppState, _: &Self::UpdateInDb, _: &Self::InDb) {}

    async fn update(
        db: &Db,
        body: Self::UpdateInDb,
    ) -> Result<Option<(Self::UpdateInDb, Self::InDb)>> {
        let options = FindOneAndUpdateOptions::builder()
            .return_document(ReturnDocument::After)
            .build();

        let filter: Self::FindInDb = body.ref_into();

        let res = db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one_and_update(
                filter.into_filter().map_err(|e| {
                    tracing::debug!("Failed to map into document: {}", Self::COLLECTION_NAME);
                    e
                })?,
                doc! {"$set": RefInto::<Result<Document>>::ref_into(&body).map_err(|e| {
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
            Some(v) => Some((body, v)),
            None => None,
        })
    }
}

pub trait DeletableInDb: FindableInDb + Deletable {
    type DeleteInDb: Debug + Send + Sync + RefInto<Self::FindInDb>;

    async fn on_delete(_: &AppState, _: &Self::DeleteInDb, _: &Self::InDb) {}

    async fn delete(
        db: &Db,
        body: Self::DeleteInDb,
    ) -> Result<Option<(Self::DeleteInDb, Self::InDb)>> {
        let res = db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .find_one_and_delete(body.ref_into().into_filter().map_err(|e| {
                tracing::debug!(
                    "Failed to map into document {}: {}",
                    Self::COLLECTION_NAME,
                    e
                );
                e
            })?)
            .await
            .map_err(|e| {
                tracing::debug!("Failed to find {}: {}", Self::COLLECTION_NAME, e);
                Error { msg: e.to_string() }
            })?;
        Ok(res.map(|v| (body, v)))
    }
}

pub trait FilterableInDb: ModelInDb + Filterable {
    type FilterInDb: Debug + Send + Sync + Serialize + RefInto<Result<Document>>;

    async fn get_some(
        db: &Db,
        body: Self::FilterInDb,
        limit: usize,
        offset: usize,
    ) -> Result<(usize, Vec<Self::InDb>)> {
        let find_options = FindOptions::builder()
            .limit(limit as i64)
            .skip(offset as u64)
            .build();

        let filter: Document = body.ref_into().map_err(|e| {
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
