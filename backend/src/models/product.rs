use common::models::product::*;
pub use common::models::product::{Product, ProductVar};
use common::utils::validators::{non_negative, non_negative_option};
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
    #[serde(deserialize_with = "non_negative")]
    pub base_price: f32,
    #[serde(deserialize_with = "non_negative")]
    pub base_discount: f32,
    pub base_images: Vec<String>,
    pub slug: String,
}

#[derive(Debug, Serialize)]
pub struct ProductFind(pub ProductFetch);

#[derive(Debug, Serialize)]
pub struct ProductUpdateInDb(pub ProductUpdate);

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
            slug: self.slug,
        }
    }
}

impl Into<Result<Document>> for &ProductFind {
    fn into(self) -> Result<Document> {
        to_document(&self).map_err(|e| Error { msg: e.to_string() })
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
            slug: self.slug,
        }
    }
}

impl Into<ProductFind> for &ProductUpdateInDb {
    fn into(self) -> ProductFind {
        ProductFind(ProductFetch { id: self.0.id })
    }
}

impl Into<ProductFind> for &ProductDelete {
    fn into(self) -> ProductFind {
        ProductFind(ProductFetch { id: self.id })
    }
}

impl Into<ProductFind> for &ProductFetch {
    fn into(self) -> ProductFind {
        ProductFind(ProductFetch { id: self.id })
    }
}

impl Into<Result<Document>> for &ProductUpdateInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
    }
}

impl From<ProductUpdate> for ProductUpdateInDb {
    fn from(value: ProductUpdate) -> Self {
        ProductUpdateInDb(value)
    }
}

impl Model for common::models::product::Product {
    const COLLECTION_NAME: &'static str = "products";
    const UNIQUE_INDICES: &'static [&'static str] = &[field!(slug @ ProductInDb)];

    type InDb = ProductInDb;
}

impl Findable for Product {
    type FindInDb = ProductFind;
}

impl Fetchable for Product {
    type FetchInDb = ProductFetch;
}

impl Creatable for Product {
    type CreateInDb = ProductCreate;
    async fn on_create(state: &AppState, body: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductCreated(body.clone()))
            .await;
    }
}

impl Updatable for Product {
    type UpdateInDb = ProductUpdateInDb;

    async fn on_update(state: &AppState, _: &Self::UpdateInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductUpdated(value.clone()))
            .await;
    }
}

impl Deletable for Product {
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
    #[serde(deserialize_with = "non_negative_option")]
    pub price: Option<f32>,
    #[serde(deserialize_with = "non_negative_option")]
    pub discount: Option<f32>,
    pub stocks: usize,
    pub images: Vec<String>,
    pub attrs: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct ProductVarFindInDb(pub ProductVarFetch);

#[derive(Debug, Serialize)]
pub struct ProductVarUpdateInDb(pub ProductVarUpdate);

#[derive(Debug, Serialize)]
pub struct ProductVarFilterInDb(pub ProductVarFilter);

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

impl Into<Result<Document>> for &ProductVarFindInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
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

impl Into<ProductVarFindInDb> for &ProductVarUpdateInDb {
    fn into(self) -> ProductVarFindInDb {
        ProductVarFindInDb(ProductVarFetch {
            sku: self.0.sku.clone(),
        })
    }
}

impl Into<Result<Document>> for &ProductVarUpdateInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
    }
}

impl From<ProductVarUpdate> for ProductVarUpdateInDb {
    fn from(value: ProductVarUpdate) -> Self {
        ProductVarUpdateInDb(value)
    }
}

impl Into<ProductVarFindInDb> for &ProductVarFetch {
    fn into(self) -> ProductVarFindInDb {
        ProductVarFindInDb(ProductVarFetch {
            sku: self.sku.clone(),
        })
    }
}

impl Into<ProductVarFindInDb> for &ProductVarUpdate {
    fn into(self) -> ProductVarFindInDb {
        ProductVarFindInDb(ProductVarFetch {
            sku: self.sku.clone(),
        })
    }
}

impl Into<ProductVarFindInDb> for &ProductVarDelete {
    fn into(self) -> ProductVarFindInDb {
        ProductVarFindInDb(ProductVarFetch {
            sku: self.sku.clone(),
        })
    }
}

impl From<ProductVarFilter> for ProductVarFilterInDb {
    fn from(value: ProductVarFilter) -> Self {
        ProductVarFilterInDb(value)
    }
}

impl Into<Result<Document>> for &ProductVarFilterInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Model for ProductVar {
    const COLLECTION_NAME: &'static str = "variants";
    const UNIQUE_INDICES: &'static [&'static str] = &[field!(sku @ ProductVarInDb)];

    type InDb = ProductVarInDb;
}

impl Findable for ProductVar {
    type FindInDb = ProductVarFindInDb;
}

impl Fetchable for ProductVar {
    type FetchInDb = ProductVarFetch;
}

impl Creatable for ProductVar {
    type CreateInDb = ProductVarCreate;

    async fn create(db: &DB, body: Self::Create) -> Result<Self::InDb> {
        let mut session = db.client().start_session().await.map_err(|_| ())?;
        let txn_options = TransactionOptions::builder().build();
        session
            .start_transaction()
            .with_options(txn_options)
            .await
            .map_err(|_| ())?;

        let filter: Result<Document> = ProductFind(ProductFetch {
            id: body.product_id,
        })
        .ref_into();

        let product_exists = db
            .collection::<<Product as Model>::InDb>(Product::COLLECTION_NAME)
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

impl Updatable for ProductVar {
    type UpdateInDb = ProductVarUpdateInDb;

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

impl Deletable for ProductVar {
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

impl Filterable for ProductVar {
    type FilterInDb = ProductVarFilterInDb;
}
