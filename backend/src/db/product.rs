use bson::DateTime;
use field::*;
use indexmap::{IndexMap, IndexSet};
use mongodb::bson::to_document;
use mongodb::bson::{doc, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use super::*;
use crate::models::product::*;
use crate::register_model;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ProductInDb {
    pub _id: ObjectId,
    pub business_id: ObjectId,
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
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl IntoFilter for ProductFetch {
    fn into_filter(&self) -> Result<Document> {
        let filter = match self {
            Self::Id(id) => FindInDb { _id: id }.into_filter()?,
            Self::Slug(slug) => doc! {slug: slug},
        };
        Ok(filter)
    }
}

impl Into<ProductFetch> for &ProductFetch {
    fn into(self) -> ProductFetch {
        (*self).clone()
    }
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

impl Into<ProductInDb> for ByBusinessId<ProductCreate> {
    fn into(self) -> ProductInDb {
        let ByBusinessId { business_id, body } = self;
        ProductInDb {
            _id: ObjectId::new(),
            business_id,
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
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Into<ProductFetch> for &ProductUpdate {
    fn into(self) -> ProductFetch {
        ProductFetch::Id(self.id)
    }
}

impl Into<ProductFetch> for &ProductDelete {
    fn into(self) -> ProductFetch {
        ProductFetch::Id(self.id)
    }
}

impl Into<Result<Document>> for &ProductUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

pub struct Product;

register_model!(Product);
impl ModelInDb for Product {
    const COLLECTION_NAME: &'static str = "products";

    type InDb = ProductInDb;

    async fn init_coll(db: &Db) -> Result<()> {
        let keys_doc = doc! {
            field!(business_id @ ProductInDb): 1,
            field!(slug @ ProductInDb): 1,
        };

        let partial_filter_expression = doc! {
            field!(slug @ ProductInDb): { "$gt": "" }
        };

        let coll = db.collection::<Self::InDb>(Self::COLLECTION_NAME);
        coll.create_index(
            IndexModel::builder()
                .keys(doc! {
                   field!(business_id @ ProductInDb): 1,
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
    type FindInDb = ByBusinessId<ProductFetch>;
}

impl FetchableInDb for Product {
    type FetchInDb = ByBusinessId<ProductFetch>;
}

impl ListableInDb for Product {
    type ListInDb = ByBusinessId<Void>;
}

impl CreatableInDb for Product {
    type CreateInDb = ByBusinessId<ProductCreate>;
}

impl UpdatableInDb for Product {
    type UpdateInDb = ByBusinessId<ProductUpdate>;
}

impl DeletableInDb for Product {
    type DeleteInDb = ByBusinessId<ProductDelete>;
}
