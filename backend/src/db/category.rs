use bson::DateTime;
use field::field;
use mongodb::bson::{doc, oid::ObjectId};
use serde::{Deserialize, Serialize};

use super::ModelInDb;
use super::*;
use crate::models::category::*;
use crate::register_model;

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryInDb {
    pub _id: ObjectId,
    pub business_id: ObjectId,
    pub name: String,
    pub slug: String,
    pub archived: bool,
    pub created_at: DateTime,
}

impl Into<CategoryPublic> for CategoryInDb {
    fn into(self) -> CategoryPublic {
        CategoryPublic {
            id: self._id,
            name: self.name,
            slug: self.slug,
            archived: self.archived,
            created_at: self.created_at,
        }
    }
}

impl Into<Result<Document>> for &CategoryUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<FindInDb> for &CategoryUpdate {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<FindInDb> for &ByBusinessId<ById> {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.body.id }
    }
}

impl Into<CategoryInDb> for ByBusinessId<CategoryCreate> {
    fn into(self) -> CategoryInDb {
        let ByBusinessId { business_id, body } = self;
        CategoryInDb {
            _id: ObjectId::new(),
            business_id,
            name: body.name,
            slug: body.slug,
            archived: false,
            created_at: DateTime::now(),
        }
    }
}

pub struct Category;

register_model!(Category);
impl ModelInDb for Category {
    const COLLECTION_NAME: &'static str = "Categories";
    const UNIQUE_INDICES: &'static [(&'static [&'static str], bool)] = &[
        (&[field!(business_id @ CategoryInDb)], false),
        (
            &[
                field!(business_id @ CategoryInDb),
                field!(_id @ CategoryInDb),
            ],
            false,
        ),
        (&[field!(slug @ CategoryInDb)], true),
        (&[field!(name @ CategoryInDb)], true),
    ];

    type InDb = CategoryInDb;
}

impl FindableInDb for Category {
    type FindInDb = ByBusinessId<FindInDb>;
}

impl FetchableInDb for Category {
    type FetchInDb = ByBusinessId<ById>;
}

impl UpdatableInDb for Category {
    type UpdateInDb = ByBusinessId<CategoryUpdate>;
}

impl ListableInDb for Category {
    type ListInDb = ByBusinessId<Void>;
}

impl CreatableInDb for Category {
    type CreateInDb = ByBusinessId<CategoryCreate>;
}
