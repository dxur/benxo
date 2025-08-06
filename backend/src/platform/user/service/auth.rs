use tokio::task;
use tracing::{error, info, instrument, warn};

use crate::{
    platform::user::mail::send_reset_email,
    types::{name::Name, password::Password, username::Username},
    utils::jwt::decode_jwt,
};

use super::*;

impl<R: UserRepo> UserService<R> {
    #[instrument(skip(self), fields(step = ?step, token_type = %token.type_name()))]
    pub async fn auth(
        &self,
        step: AuthStep,
        token: UserToken,
    ) -> ApiResult<(UserToken, MessageResponse)> {
        match (step, token) {
            (AuthStep::SignupEmail { email }, _) => self.handle_email_step(email).await,
            (AuthStep::SignupPhone { otp, phone }, UserToken::SignupEmail { email, otp_hash }) => {
                self.handle_phone_step(email, phone, otp, otp_hash).await
            }
            (
                AuthStep::SignupFinalize {
                    otp,
                    first_name,
                    last_name,
                    username,
                    password,
                },
                UserToken::SignupPhone {
                    email,
                    phone,
                    otp_hash,
                },
            ) => {
                self.handle_finalize_step(
                    email, phone, otp, otp_hash, first_name, last_name, username, password,
                )
                .await
            }
            (AuthStep::ResetPassword { email }, _) => {
                self.handle_req_reset_password_step(email).await
            }
            (
                AuthStep::ResetPasswordFinalize { otp, password },
                UserToken::ResetPassword { email, otp_hash },
            ) => {
                self.handle_reset_password_step(email, otp, otp_hash, password)
                    .await
            }
            (AuthStep::Login { email, password }, _) => {
                self.handle_login_step(email, password).await
            }
            (step, token) => {
                warn!(
                    step = ?step,
                    token_type = %token.type_name(),
                    "Invalid step/token combination"
                );
                Err(ApiError::invalid_token())
            }
        }
    }

    #[instrument(skip(self), fields(email = %email))]
    async fn handle_email_step(&self, email: Email) -> ApiResult<(UserToken, MessageResponse)> {
        let otp = generate_otp()?;
        let otp_hash = blake3::hash(otp.as_bytes());

        let count = self
            .repo
            .count(UserFilter {
                email: Some(email.as_str().into()),
                ..Default::default()
            })
            .await?;

        if count > 0 {
            warn!("Email already exists");
        } else {
            send_verification_email(&email, &otp, Duration::hours(1))
                .await
                .map_err(|e| {
                    error!(error = ?e, "Failed to send verification email");
                    e
                })?;
            info!("Verification email sent");
        }

        Ok((
            UserToken::SignupEmail { email, otp_hash },
            MessageResponse {
                message: "We sent you a verification email".to_string(),
            },
        ))
    }

    #[instrument(skip(self), fields(email = %email, phone = %phone))]
    async fn handle_phone_step(
        &self,
        email: Email,
        phone: PhoneNumber,
        otp: String,
        otp_hash: blake3::Hash,
    ) -> ApiResult<(UserToken, MessageResponse)> {
        let provided_otp_hash = blake3::hash(otp.as_bytes());
        if provided_otp_hash != otp_hash {
            warn!("Invalid OTP provided");
            return Err(ApiError::unauthorized("Invalid OTP"));
        }

        let otp = generate_otp()?;
        let otp_hash = blake3::hash(otp.as_bytes());

        let count = self
            .repo
            .count(UserFilter {
                phone: Some(phone.as_str().into()),
                ..Default::default()
            })
            .await?;

        if count == 0 {
            send_verification_otp(&phone, &otp).await.map_err(|e| {
                error!(error = ?e, "Failed to send verification OTP");
                e
            })?;

            info!("Verification code sent");
        } else {
            warn!("Phone already exists");
        }

        let phone_token = UserToken::SignupPhone {
            email,
            phone,
            otp_hash,
        };

        Ok((
            phone_token,
            MessageResponse {
                message: "We sent you a verification code".to_string(),
            },
        ))
    }

    #[instrument(skip(self, password), fields(email = %email, username = %username))]
    async fn handle_finalize_step(
        &self,
        email: Email,
        phone: PhoneNumber,
        otp: String,
        otp_hash: blake3::Hash,
        first_name: Name,
        last_name: Name,
        username: Username,
        password: Password,
    ) -> ApiResult<(UserToken, MessageResponse)> {
        let provided_otp_hash = blake3::hash(otp.as_bytes());
        if provided_otp_hash != otp_hash {
            warn!("Invalid OTP provided");
            return Err(ApiError::unauthorized("Invalid OTP"));
        }

        let password_hash =
            task::spawn_blocking(move || bcrypt::hash(password.as_str(), bcrypt::DEFAULT_COST))
                .await
                .map_err(|e| {
                    error!(error = ?e, "Join failed");
                    ApiError::internal("Join failed")
                })?
                .map_err(|e| {
                    error!(error = ?e, "Failed to hash password");
                    ApiError::internal("Can't hash the password")
                })?;

        let user_record = UserRecord::new(
            email.clone(),
            username.clone(),
            first_name,
            last_name,
            phone,
            password_hash,
        );

        let user = self.repo.create(user_record).await.map_err(|e| {
            error!(error = ?e, "Failed to create user");
            e
        })?;

        info!(user_id = %user._id.to_hex(), "User created successfully");

        Ok((
            UserToken::UserSession(UserSession {
                user_id: user._id,
                email: user.email,
            }),
            MessageResponse {
                message: "You have successfully signed up".to_string(),
            },
        ))
    }

    #[instrument(skip(self), fields(email = %email))]
    async fn handle_req_reset_password_step(
        &self,
        email: Email,
    ) -> ApiResult<(UserToken, MessageResponse)> {
        let otp = generate_otp()?;
        let otp_hash = blake3::hash(otp.as_bytes());

        let count = self
            .repo
            .count(UserFilter {
                email: Some(email.as_str().into()),
                ..Default::default()
            })
            .await?;

        if count == 0 {
            warn!("Email does not exists");
        } else {
            send_reset_email(&email, &otp, Duration::minutes(15))
                .await
                .map_err(|e| {
                    error!(error = ?e, "Failed to send reset email");
                    e
                })?;
            info!("reset email sent");
        }

        Ok((
            UserToken::ResetPassword { email, otp_hash },
            MessageResponse {
                message: "We sent you a reset email".to_string(),
            },
        ))
    }

    #[instrument(skip(self, password), fields(email = %email))]
    async fn handle_reset_password_step(
        &self,
        email: Email,
        otp: String,
        otp_hash: blake3::Hash,
        password: Password,
    ) -> ApiResult<(UserToken, MessageResponse)> {
        let mut user = self
            .repo
            .find_by_email(email.as_str())
            .await?
            .ok_or(ApiError::not_found("user", "Not found"))?;

        let password_hash =
            task::spawn_blocking(move || bcrypt::hash(password.as_str(), bcrypt::DEFAULT_COST))
                .await
                .map_err(|e| {
                    error!(error = ?e, "Join failed");
                    ApiError::internal("Join failed")
                })?
                .map_err(|e| {
                    error!(error = ?e, "Failed to hash password");
                    ApiError::internal("Can't hash the password")
                })?;

        user.password_hash = password_hash;
        user.last_password_reset = Some(bson::DateTime::now());
        let id = user._id;

        let _user = self.repo.update(id, user).await?;

        info!(user_id = %id.to_hex(), "Password updated successfully");

        Ok((
            UserToken::None,
            MessageResponse {
                message: "You have reset your password".to_string(),
            },
        ))
    }

    #[instrument(skip(self, password), fields(email = %email))]
    async fn handle_login_step(
        &self,
        email: Email,
        password: Password,
    ) -> ApiResult<(UserToken, MessageResponse)> {
        let mut user = self
            .repo
            .find_by_email(&email.as_str())
            .await?
            .ok_or_else(|| {
                warn!("User not found");
                ApiError::unauthorized("Invalid credentials")
            })?;

        let password_valid: bool;
        (password_valid, user.password_hash) = task::spawn_blocking(move || {
            verify(password.as_str().as_bytes(), &user.password_hash)
                .map(|v| (v, user.password_hash))
        })
        .await
        .map_err(|e| {
            error!(error = ?e, "Join failed");
            ApiError::internal("Join failed")
        })?
        .map_err(|e| {
            error!(error = ?e, "Password verification failed");
            ApiError::internal("Password verification failed")
        })?;

        if !password_valid {
            warn!(
                user_id = %user._id.to_hex(),
                login_attempts = user.login_attempts,
                "Invalid password provided"
            );

            self.repo.increment_login_attempts(user._id).await?;

            if user.login_attempts >= 4 {
                let lock_until = bson::DateTime::from_chrono(Utc::now() + Duration::hours(1));
                self.repo.lock_user(user._id, lock_until).await?;
                warn!(
                    user_id = %user._id.to_hex(),
                    "Account locked due to too many failed attempts"
                );
            }

            return Err(ApiError::unauthorized("Invalid credentials"));
        }

        if user.is_locked() {
            warn!(user_id = %user._id.to_hex(), "Attempt to login to locked account");
            return Err(ApiError::unauthorized("Account is temporarily locked"));
        }

        if !user.can_login() {
            warn!(user_id = %user._id.to_hex(), "Attempt to login to inactive account");
            return Err(ApiError::unauthorized("Account is not active"));
        }

        self.repo.reset_login_attempts(user._id).await?;
        let id = user._id;
        let email = user.email.clone();
        let mut updated_user = user;
        updated_user.last_login = bson::DateTime::now();
        let _ = self.repo.update(id, updated_user).await.map_err(|e| {
            error!(error = ?e, user_id = %id.to_hex(), "Failed to update user login timestamp");
            e
        })?;

        info!(user_id = %id.to_hex(), "Login successful");

        Ok((
            UserToken::UserSession(UserSession { user_id: id, email }),
            MessageResponse {
                message: "Login successful".to_string(),
            },
        ))
    }
}
