use std::collections::{HashMap, HashSet};

use crate::models::product::*;
pub use crate::models::product::{Product, ProductVar};
use field::*;
use mongodb::bson::to_document;
use mongodb::bson::{doc, oid::ObjectId, Document};
use mongodb::options::TransactionOptions;
use serde::{Deserialize, Serialize};

use super::*;
use crate::events::Event;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductInDb {
    pub _id: ObjectId,
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    pub base_price: f32,
    pub base_discount: f32,
    pub base_images: Vec<String>,
    pub attributes: HashSet<String>,
    pub slug: String,
}

impl Into<ProductPublic> for ProductInDb {
    fn into(self) -> ProductPublic {
        ProductPublic {
            id: self._id,
            name: self.name,
            description: self.description,
            featured: self.featured,
            category: self.category,
            base_price: self.base_price,
            base_discount: self.base_discount,
            base_images: self.base_images,
            attributes: self.attributes,
            slug: self.slug,
        }
    }
}

impl Into<ProductInDb> for ProductCreate {
    fn into(self) -> ProductInDb {
        ProductInDb {
            _id: ObjectId::new(),
            name: self.name,
            description: self.description,
            featured: self.featured,
            category: self.category,
            base_price: self.base_price,
            base_discount: self.base_discount,
            base_images: self.base_images,
            attributes: self.attributes,
            slug: self.slug,
        }
    }
}

impl Into<FindInDb> for &ProductUpdate {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<FindInDb> for &ProductDelete {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<FindInDb> for &ProductFetch {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<Result<Document>> for &ProductUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl ModelInDb for Product {
    const COLLECTION_NAME: &'static str = "products";

    type InDb = ProductInDb;

    async fn init_coll(db: &Db) -> Result<()> {
        let keys_doc = doc! {
            field!(slug @ ProductInDb): 1,
        };

        let partial_filter_expression = doc! {
            field!(slug @ ProductInDb): { "$gt": "" }
        };
        
        db.collection::<Self::InDb>(Self::COLLECTION_NAME)
            .create_index(
                IndexModel::builder()
                    .keys(keys_doc)
                    .options(
                        IndexOptions::builder()
                        .unique(true)
                        .partial_filter_expression(partial_filter_expression)
                        .build())
                    .build(),
            )
            .await
            .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))
    }
}

impl FindableInDb for Product {
    type FindInDb = FindInDb;
}

impl FetchableInDb for Product {
    type FetchInDb = ProductFetch;
}

impl CreatableInDb for Product {
    type CreateInDb = ProductCreate;
    async fn on_create(state: &AppState, body: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductCreated(body.clone()))
            .await;
    }
}

impl UpdatableInDb for Product {
    type UpdateInDb = ProductUpdate;

    async fn on_update(state: &AppState, _: &Self::UpdateInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductUpdated(value.clone()))
            .await;
    }
}

impl DeletableInDb for Product {
    type DeleteInDb = ProductDelete;

    async fn on_delete(state: &AppState, _: &Self::DeleteInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductDeleted(value.clone()))
            .await;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductVarInDb {
    pub sku: String,
    pub product_id: ObjectId,
    pub name: String,
    pub description: String,
    pub price: Option<f32>,
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: HashMap<String, String>,
}

impl Into<ProductVarPublic> for ProductVarInDb {
    fn into(self) -> ProductVarPublic {
        ProductVarPublic {
            sku: self.sku,
            product_id: self.product_id,
            name: self.name,
            description: self.description,
            price: self.price,
            discount: self.discount,
            stocks: self.stocks,
            images: self.images,
            attrs: self.attrs,
        }
    }
}

impl Into<Result<Document>> for &ProductVarFetch {
    fn into(self) -> Result<Document> {
        to_document(&self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<ProductVarInDb> for ProductVarCreate {
    fn into(self) -> ProductVarInDb {
        ProductVarInDb {
            sku: self.sku,
            product_id: self.product_id,
            name: self.name,
            description: self.description,
            price: self.price,
            discount: self.discount,
            stocks: self.stocks,
            images: self.images,
            attrs: self.attrs,
        }
    }
}

impl Into<ProductVarFetch> for &ProductVarFetch {
    fn into(self) -> ProductVarFetch {
        ProductVarFetch {
            sku: self.sku.clone(),
        }
    }
}

impl Into<ProductVarFetch> for &ProductVarUpdate {
    fn into(self) -> ProductVarFetch {
        ProductVarFetch {
            sku: self.sku.clone(),
        }
    }
}

impl Into<Result<Document>> for &ProductVarUpdate {
    fn into(self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<ProductVarFetch> for &ProductVarDelete {
    fn into(self) -> ProductVarFetch {
        ProductVarFetch {
            sku: self.sku.clone(),
        }
    }
}

impl Into<Result<Document>> for &ProductVarFilter {
    fn into(self) -> Result<Document> {
        to_document(&self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl ModelInDb for ProductVar {
    const COLLECTION_NAME: &'static str = "variants";
    const UNIQUE_INDICES: &'static [&'static str] = &[field!(sku @ ProductVarInDb)];

    type InDb = ProductVarInDb;
}

impl FindableInDb for ProductVar {
    type FindInDb = ProductVarFetch;
}

impl FetchableInDb for ProductVar {
    type FetchInDb = ProductVarFetch;
}

impl CreatableInDb for ProductVar {
    type CreateInDb = ProductVarCreate;

    async fn create(db: &Db, body: Self::Create) -> Result<Self::InDb> {
        let mut session = db.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let filter: Result<Document> = FindInDb {
            _id: body.product_id,
        }
        .ref_into();

        let product_exists = db
            .collection::<<Product as ModelInDb>::InDb>(Product::COLLECTION_NAME)
            .find_one(filter?)
            .session(&mut session)
            .await
            .map_err(|_| ())?
            .is_some();

        if !product_exists {
            session.abort_transaction().await.ok();
            return Err(Error {
                msg: "Product does not exist".to_string(),
            });
        }

        // TODO: check attributes

        let model: Self::InDb = body.into();
        match db
            .collection::<Self::InDb>(Self::COLLECTION_NAME)
            .insert_one(&model)
            .session(&mut session)
            .await
        {
            Ok(_) => session
                .commit_transaction()
                .await
                .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(model)),
            Err(e) => {
                session.abort_transaction().await.ok();
                Err(Error { msg: e.to_string() })
            }
        }
    }

    async fn on_create(state: &AppState, body: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductVarUpdated((
                body.sku.clone(),
                body.product_id,
            )))
            .await;
    }
}

impl UpdatableInDb for ProductVar {
    type UpdateInDb = ProductVarUpdate;

    async fn on_update(state: &AppState, _: &Self::UpdateInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductVarUpdated((
                value.sku.clone(),
                value.product_id,
            )))
            .await;
    }
}

impl DeletableInDb for ProductVar {
    type DeleteInDb = ProductVarDelete;
    async fn on_delete(state: &AppState, _: &Self::DeleteInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductVarDeleted((
                value.sku.clone(),
                value.product_id,
            )))
            .await;
    }
}

impl FilterableInDb for ProductVar {
    type FilterInDb = ProductVarFilter;
}
