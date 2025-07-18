use blake3::Hash;
use bson::oid::ObjectId;
use chrono::{DateTime, Duration, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use tower_cookies::{Cookie, Cookies};
use ts_rs::TS;

use super::domain::*;
use crate::{
    types::{email::Email, name::Name, password::Password, phone::PhoneNumber, username::Username},
    utils::{
        error::ApiError,
        jwt::{decode_jwt, encode_jwt},
        serde_helpers::JsonOption,
    },
};

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub enum AuthStep {
    SignupEmail {
        email: Email,
    },
    SignupPhone {
        token: String,
        phone: PhoneNumber,
    },
    SignupOtp {
        otp: String,
    },
    SignupFinalize {
        first_name: Name,
        last_name: Name,
        username: Username,
        password: Password,
    },
    Login {
        email: Email,
        password: Password,
    },
    ResetPassword {
        email: Email,
    },
    ResetPasswordFinalize {
        token: String,
        password: Password,
    },
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerificationToken {
    pub email: Email,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UserToken {
    None,
    SignupEmail {
        email: Email,
    },
    SignupPhone {
        email: Email,
        phone: PhoneNumber,
        otp_hash: Hash,
    },
    Signup {
        email: Email,
        phone: PhoneNumber,
    },
    UserSession(UserSession),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserSession {
    pub user_id: ObjectId,
}

impl TryFrom<UserToken> for UserSession {
    type Error = ApiError;

    fn try_from(value: UserToken) -> Result<Self, Self::Error> {
        match value {
            UserToken::UserSession(session) => Ok(session),
            _ => Err(ApiError::unauthorized(Option::<String>::None)),
        }
    }
}

impl UserToken {
    pub fn type_name(&self) -> &'static str {
        match self {
            UserToken::None => "None",
            UserToken::SignupEmail { .. } => "Signup Email",
            UserToken::SignupPhone { .. } => "Signup Phone",
            UserToken::Signup { .. } => "Signup",
            UserToken::UserSession { .. } => "User Session",
        }
    }
}

impl TryFrom<&Cookies> for UserToken {
    type Error = ApiError;

    fn try_from(value: &Cookies) -> Result<Self, Self::Error> {
        Ok(value
            .get("user_token")
            .map(|val| decode_jwt(val.value()).unwrap_or(UserToken::None))
            .unwrap_or(UserToken::None))
    }
}

impl<'c> TryInto<Cookie<'c>> for UserToken {
    type Error = ApiError;
    fn try_into(self) -> Result<Cookie<'c>, ApiError> {
        let token = encode_jwt(self, Duration::days(7))?;
        Ok(Cookie::build(("user_token", token)).path("/").build())
    }
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct UserUpdate {
    pub username: JsonOption<Username>,
    pub first_name: JsonOption<Name>,
    pub last_name: JsonOption<Name>,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct UserLogin {
    pub email: Email,
    pub password: Password,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct PasswordReset {
    pub email: Email,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct PasswordResetConfirm {
    pub token: String,
    pub new_password: Password,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct PasswordChange {
    pub current_password: Password,
    pub new_password: Password,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct EmailVerification {
    pub token: String,
}

#[derive(Debug, Clone, Serialize, TS, o2o)]
#[from_owned(UserRecord)]
#[ts(export)]
pub struct UserView {
    #[from(@._id.to_hex())]
    pub id: String,
    pub email: Email,
    pub username: Username,
    pub first_name: Name,
    pub last_name: Name,
    pub phone: PhoneNumber,
    pub status: UserStatus,
    #[from(~.to_chrono())]
    pub last_login: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone)]
pub struct LoginResponse {
    pub user: UserView,
    pub token: String,
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct MessageResponse {
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, TS)]
#[ts(export)]
pub struct UserListQuery {
    pub page: Option<u32>,
    pub limit: Option<u32>,
    pub status: Option<UserStatus>,
    pub search: Option<String>,
}

#[derive(Debug, Clone, Serialize, TS)]
#[ts(export)]
pub struct UserListResponse {
    pub users: Vec<UserView>,
    pub total: u64,
    pub page: u32,
    pub limit: u32,
    pub total_pages: u32,
}
