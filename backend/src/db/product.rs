use field::*;
use indexmap::{IndexMap, IndexSet};
use mongodb::bson::to_document;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use super::*;
use crate::events::Event;
pub use crate::models::product::Product;
use crate::models::product::*;
use crate::AppState;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductInDb {
    pub _id: ObjectId,
    pub store_id: String,
    pub name: String,
    pub description: String,
    pub featured: bool,
    pub category: String,
    pub base_price: f32,
    pub base_compare_price: f32,
    pub base_images: Vec<String>,
    pub options: IndexMap<String, IndexSet<String>>,
    pub variants: Vec<ProductVariant>,
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
            base_compare_price: self.base_compare_price,
            base_images: self.base_images,
            options: self.options,
            variants: self.variants,
            slug: self.slug,
        }
    }
}

impl Into<ProductInDb> for ByStoreId<ProductCreate> {
    fn into(self) -> ProductInDb {
        let ByStoreId { store_id, body } = self;
        ProductInDb {
            _id: ObjectId::new(),
            store_id,
            name: body.name,
            category: body.category,
            slug: body.slug,
            description: Default::default(),
            featured: Default::default(),
            base_price: Default::default(),
            base_compare_price: Default::default(),
            base_images: Default::default(),
            options: Default::default(),
            variants: Default::default(),
        }
    }
}

impl Into<FindInDb> for &ProductFetch {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
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
            field!(store_id @ ProductInDb): 1,
            field!(slug @ ProductInDb): 1,
        };

        let partial_filter_expression = doc! {
            field!(slug @ ProductInDb): { "$gt": "" }
        };

        let coll = db.collection::<Self::InDb>(Self::COLLECTION_NAME);
        coll.create_index(
            IndexModel::builder()
                .keys(doc! {
                   field!(store_id @ ProductInDb): 1,
                })
                .build(),
        )
        .await
        .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))?;

        coll.create_index(
            IndexModel::builder()
                .keys(keys_doc)
                .options(
                    IndexOptions::builder()
                        .unique(true)
                        .partial_filter_expression(partial_filter_expression)
                        .build(),
                )
                .build(),
        )
        .await
        .map_or_else(|e| Err(Error { msg: e.to_string() }), |_| Ok(()))
    }
}

impl FindableInDb for Product {
    type FindInDb = ByStoreId<FindInDb>;
}

impl FetchableInDb for Product {
    type FetchInDb = ByStoreId<ProductFetch>;
}

impl ListableInDb for Product {
    type ListInDb = ByStoreId<Void>;
}

impl CreatableInDb for Product {
    type CreateInDb = ByStoreId<ProductCreate>;
    async fn on_create(state: &AppState, body: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductCreated(body.clone()))
            .await;
    }
}

impl UpdatableInDb for Product {
    type UpdateInDb = ByStoreId<ProductUpdate>;

    async fn on_update(state: &AppState, update: &Self::UpdateInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductUpdated(update.body.clone(), value.clone()))
            .await;
    }
}

impl DeletableInDb for Product {
    type DeleteInDb = ByStoreId<ProductDelete>;

    async fn on_delete(state: &AppState, _: &Self::DeleteInDb, value: &Self::InDb) {
        state
            .event_bus
            .emit(Event::ProductDeleted(value.clone()))
            .await;
    }
}
