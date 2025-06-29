// pub mod channel;
pub mod domain;
pub mod file;
pub mod order;
pub mod product;
pub mod settings;
pub mod store;
pub mod user;

use bson::oid::ObjectId;
use bson::to_document;
use futures::TryStreamExt;
use linkme::distributed_slice;
use mongodb::bson::{doc, Document};
use mongodb::options::{FindOneAndUpdateOptions, FindOptions, IndexOptions, ReturnDocument};
use mongodb::IndexModel;
use mongodb::{options::ClientOptions, Client, Database};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;

use crate::models::ById;
use crate::utils::error::*;
use crate::utils::types::{HaveContext, RefInto, Result};
use crate::WithDb;

pub struct Db(Database);

impl Db {
    pub async fn new(uri: &str, name: &str) -> Self {
        let client_options = ClientOptions::parse(uri).await.unwrap();
        let db = Db(Client::with_options(client_options).unwrap().database(name));
        init_models(&db).await;
        db
    }
}

impl Deref for Db {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

pub trait __ModelRegisteredByMacro {}

pub type ModelInitFn = for<'a> fn(&'a Db) -> Pin<Box<dyn Future<Output = Result<()>> + Send + 'a>>;

#[distributed_slice]
pub static MODELS_INIT: [ModelInitFn];

pub async fn init_models(db: &Db) {
    for f in MODELS_INIT {
        f(&db).await.unwrap();
    }
}

pub trait ModelInDb: __ModelRegisteredByMacro {
    const COLLECTION_NAME: &'static str;
    const UNIQUE_INDICES: &'static [(&'static [&'static str], bool)] = &[];

    type InDb: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;

    async fn init_coll(db: &Db) -> Result<()> {
        if Self::UNIQUE_INDICES.len() == 0 {
            return Ok(());
        }
        for (index, unique) in Self::UNIQUE_INDICES {
            let mut keys_doc = doc! {};

            for key in *index {
                keys_doc.insert(key.to_string(), 1);
            }

            db.collection::<Self::InDb>(Self::COLLECTION_NAME)
                .create_index(
                    IndexModel::builder()
                        .keys(keys_doc)
                        .options(IndexOptions::builder().unique(*unique).build())
                        .build(),
                )
                .await
                .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))?;
        }
        Ok(())
    }
}

pub trait IntoFilter {
    fn into_filter(&self) -> Result<Document>;
}

#[derive(Debug, Serialize)]
pub struct FindInDb<T = ObjectId>
where
    T: Serialize,
{
    pub _id: T,
}

impl<T: Serialize + Clone> Into<FindInDb<T>> for &ById<T> {
    fn into(self) -> FindInDb<T> {
        FindInDb {
            _id: self.id.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct Void;

impl<T> From<&T> for Void {
    fn from(_: &T) -> Self {
        Void
    }
}

#[derive(Debug, Serialize)]
pub struct ByBusinessId<T> {
    pub business_id: ObjectId,
    #[serde(skip)]
    pub body: T,
}

impl<T: IntoFilter> IntoFilter for ByBusinessId<T> {
    fn into_filter(&self) -> Result<Document> {
        let d = self.body.into_filter()?;
        let mut res = to_document(self).map_err(|e| Error { msg: e.to_string() })?;
        res.extend(d);
        Ok(res)
    }
}

impl IntoFilter for ByBusinessId<Void> {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl<T> Into<Result<Document>> for &ByBusinessId<T>
where
    T: RefInto<Result<Document>>,
{
    fn into(self) -> Result<Document> {
        self.body.ref_into()
    }
}

impl<T> From<HaveContext<T, ObjectId>> for ByBusinessId<T> {
    fn from(value: HaveContext<T, ObjectId>) -> Self {
        let HaveContext(body, business_id) = value;
        Self { business_id, body }
    }
}

impl<T, U> Into<ByBusinessId<U>> for &ByBusinessId<T>
where
    T: RefInto<U>,
{
    fn into(self) -> ByBusinessId<U> {
        ByBusinessId {
            business_id: self.business_id.clone(),
            body: self.body.ref_into(),
        }
    }
}

impl<T: Serialize> IntoFilter for FindInDb<T> {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

pub trait FindableInDb: ModelInDb {
    type FindInDb: Debug + Serialize + IntoFilter;
}

pub trait CreatableInDb: ModelInDb {
    type CreateInDb: Debug + Send + Sync + Serialize + Into<Self::InDb>;

    async fn on_create(_: &impl WithDb, _: &Self::InDb) {}

    async fn create(db: &Db, body: Self::CreateInDb) -> Result<Self::InDb> {
        let model: Self::InDb = body.into();
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

pub trait FetchableInDb: FindableInDb {
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

pub trait UpdatableInDb: FindableInDb {
    type UpdateInDb: Send + Debug + Sync + RefInto<Self::FindInDb> + RefInto<Result<Document>>;

    async fn on_update(_: &impl WithDb, _: &Self::UpdateInDb, _: &Self::InDb) {}

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

pub trait DeletableInDb: FindableInDb {
    type DeleteInDb: Debug + Send + Sync + RefInto<Self::FindInDb>;

    async fn on_delete(_: &impl WithDb, _: &Self::DeleteInDb, _: &Self::InDb) {}

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

pub trait FilterableInDb: ModelInDb {
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
