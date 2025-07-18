use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::types::{email::Email, name::Name, phone::PhoneNumber, username::Username};

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    Active,
    Inactive,
    Banned,
    Deleted,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserRecord {
    pub _id: ObjectId,
    pub email: Email,
    pub username: Username,
    pub first_name: Name,
    pub last_name: Name,
    pub phone: PhoneNumber,
    pub password_hash: String,
    pub last_password_reset: Option<DateTime>,
    pub status: UserStatus,
    pub last_login: DateTime,
    pub login_attempts: u32,
    pub locked_until: Option<DateTime>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl UserRecord {
    pub fn new(
        email: Email,
        username: Username,
        first_name: Name,
        last_name: Name,
        phone: PhoneNumber,
        password_hash: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            _id: ObjectId::new(),
            email,
            username,
            first_name,
            last_name,
            phone,
            password_hash,
            status: UserStatus::Active,
            last_password_reset: None,
            last_login: now,
            login_attempts: 0,
            locked_until: None,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn is_locked(&self) -> bool {
        if let Some(locked_until) = self.locked_until {
            DateTime::now() < locked_until
        } else {
            false
        }
    }

    pub fn can_login(&self) -> bool {
        matches!(self.status, UserStatus::Active) && !self.is_locked()
    }
}

#[derive(Debug, Clone, Default)]
pub struct UserFilter {
    pub email: Option<String>,
    pub phone: Option<String>,
    pub username: Option<String>,
    pub status: Option<UserStatus>,
    pub created_after: Option<DateTime>,
    pub created_before: Option<DateTime>,
}
