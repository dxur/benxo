use crate::{models::user::*, register_model};
use bson::DateTime;
use field::field;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInDb {
    pub _id: ObjectId,
    pub business_id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize)]
pub struct UserFindInDb {
    email: String,
}

impl IntoFilter for UserFindInDb {
    fn into_filter(&self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<UserPublic> for UserInDb {
    fn into(self) -> UserPublic {
        UserPublic {
            id: self._id,
            name: self.name,
            email: self.email,
        }
    }
}

register_model!(User);
impl ModelInDb for User {
    const COLLECTION_NAME: &'static str = "users";
    const UNIQUE_INDICES: &'static [(&'static [&'static str], bool)] =
        &[(&[field!(email @ UserInDb)], true)];

    type InDb = UserInDb;
}

impl Into<UserFindInDb> for &UserFetch {
    fn into(self) -> UserFindInDb {
        UserFindInDb {
            email: self.email.clone(),
        }
    }
}

impl Into<UserInDb> for UserCreate {
    fn into(self) -> UserInDb {
        UserInDb {
            _id: ObjectId::new(),
            business_id: ObjectId::new(),
            name: self.name,
            email: self.email,
            password: self.password,
            created_at: DateTime::now(),
            updated_at: DateTime::now(),
        }
    }
}

impl Into<UserFindInDb> for &UserUpdate {
    fn into(self) -> UserFindInDb {
        UserFindInDb {
            email: self.email.clone(),
        }
    }
}

impl Into<Result<Document>> for &UserUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<UserFindInDb> for &UserDelete {
    fn into(self) -> UserFindInDb {
        UserFindInDb {
            email: self.email.clone(),
        }
    }
}

impl FindableInDb for User {
    type FindInDb = UserFindInDb;
}

impl FetchableInDb for User {
    type FetchInDb = UserFetch;
}

impl CreatableInDb for User {
    type CreateInDb = UserCreate;
}

impl UpdatableInDb for User {
    type UpdateInDb = UserUpdate;
}

impl DeletableInDb for User {
    type DeleteInDb = UserDelete;
}
