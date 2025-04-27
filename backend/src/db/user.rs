use crate::models::user::*;
use field::*;
use mongodb::bson::{doc, oid::ObjectId, to_document, Document};
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UserInDb {
    pub _id: ObjectId,
    pub name: String,
    pub email: String,
    pub password: String,
    pub permissions: UserPermissions,
}

#[derive(Debug, Serialize)]
pub struct UserFindInDb(pub UserFetch);

#[derive(Debug, Serialize)]
pub struct UserUpdateInDb(pub UserUpdate);

impl Into<UserPublic> for UserInDb {
    fn into(self) -> UserPublic {
        UserPublic {
            id: self._id,
            name: self.name,
            email: self.email,
            permissions: self.permissions,
        }
    }
}

impl ModelInDb for User {
    const COLLECTION_NAME: &'static str = "users";
    const UNIQUE_INDICES: &'static [&'static str] = &[];

    type InDb = UserInDb;
}

impl Into<Result<Document>> for &UserFindInDb {
    fn into(self) -> Result<Document> {
        to_document(self).map_err(|e| Error { msg: e.to_string() })
    }
}

impl FindableInDb for User {
    type FindInDb = UserFindInDb;
}

impl Into<UserFindInDb> for &UserFetch {
    fn into(self) -> UserFindInDb {
        UserFindInDb(UserFetch { id: self.id })
    }
}

impl FetchableInDb for User {
    type FetchInDb = UserFetch;
}

impl Into<UserInDb> for UserCreate {
    fn into(self) -> UserInDb {
        UserInDb {
            _id: ObjectId::new(),
            name: self.name,
            email: self.email,
            password: self.password,
            permissions: self.permissions,
        }
    }
}

impl CreatableInDb for User {
    type CreateInDb = UserCreate;
}

impl From<UserUpdate> for UserUpdateInDb {
    fn from(value: UserUpdate) -> Self {
        UserUpdateInDb(value)
    }
}

impl Into<UserFindInDb> for &UserUpdateInDb {
    fn into(self) -> UserFindInDb {
        UserFindInDb(UserFetch { id: self.0.id })
    }
}

impl Into<Result<Document>> for &UserUpdateInDb {
    fn into(self) -> Result<Document> {
        to_document(&self.0).map_err(|e| Error { msg: e.to_string() })
    }
}

impl UpdatableInDb for User {
    type UpdateInDb = UserUpdateInDb;
}

impl Into<UserFindInDb> for &UserDelete {
    fn into(self) -> UserFindInDb {
        UserFindInDb(UserFetch { id: self.id })
    }
}

impl DeletableInDb for User {
    type DeleteInDb = UserDelete;
}
