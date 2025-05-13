use crate::models::user::*;
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

    type InDb = UserInDb;
}

impl Into<FindInDb> for &UserFetch {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
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

impl Into<FindInDb> for &UserUpdate {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl Into<Result<Document>> for &UserUpdate {
    fn into(self) -> Result<Document> {
        to_document(&self.body).map_err(|e| Error { msg: e.to_string() })
    }
}

impl Into<FindInDb> for &UserDelete {
    fn into(self) -> FindInDb {
        FindInDb { _id: self.id }
    }
}

impl FindableInDb for User {
    type FindInDb = FindInDb;
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
