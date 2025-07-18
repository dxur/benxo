use bson::{oid::ObjectId, DateTime};
use uuid::Uuid;

use super::api::*;
use super::domain::*;
use super::repo::BusinessRepo;
use crate::platform::user::api::MessageResponse;
use crate::platform::user::api::UserSession;
use crate::utils::error::{ApiError, ApiResult};
use crate::utils::serde_helpers::JsonOption;

pub struct BusinessService<B: BusinessRepo> {
    business_repo: B,
}

impl<B: BusinessRepo> BusinessService<B> {
    pub fn new(business_repo: B) -> Self {
        Self { business_repo }
    }

    pub async fn create(user: UserSession, create_req: BusinessCreate) -> ApiResult<BusinessView> {
        todo!()
    }

    pub async fn current(business: BusinessSession) -> ApiResult<BusinessView> {
        todo!()
    }
}
