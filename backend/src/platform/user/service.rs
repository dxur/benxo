use bcrypt::verify;
use bson::oid::ObjectId;
use chrono::{Duration, Utc};

use super::api::*;
use super::domain::*;
use super::repo::UserRepo;
use crate::platform::user::mail::send_verification_email;
use crate::platform::user::mail::send_verification_otp;
use crate::types::email::Email;
use crate::types::phone::PhoneNumber;
use crate::utils::error::{ApiError, ApiResult};
use crate::utils::rand::generate_otp;
use crate::utils::serde_helpers::JsonOption;

mod auth;

pub struct UserService<R: UserRepo> {
    repo: R,
}

impl<R: UserRepo> UserService<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    pub async fn me(&self, id: ObjectId) -> ApiResult<UserDto> {
        let user = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| ApiError::not_found("user", id.to_hex()))?;

        Ok(UserDto::from(user))
    }

    pub async fn update_user(&self, id: ObjectId, update_req: UserUpdate) -> ApiResult<UserDto> {
        let mut user = self
            .repo
            .find_by_id(id)
            .await?
            .ok_or_else(|| ApiError::NotFound {
                resource: "user".into(),
                id: id.to_hex().into(),
            })?;

        if let JsonOption::Value(username) = update_req.username {
            if let Some(existing) = self.repo.find_by_username(username.as_str()).await? {
                if existing._id != user._id {
                    return Err(ApiError::conflict("user", "Username already exists"));
                }
            }
            user.username = username;
        }

        if let JsonOption::Value(first_name) = update_req.first_name {
            user.first_name = first_name;
        }

        if let JsonOption::Value(last_name) = update_req.last_name {
            user.last_name = last_name;
        }

        let updated_user = self.repo.update(id, user).await?;
        Ok(UserDto::from(updated_user))
    }
}
