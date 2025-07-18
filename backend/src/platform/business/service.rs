use bson::{oid::ObjectId, DateTime};
use uuid::Uuid;

use super::api::*;
use super::domain::*;
use super::repo::BusinessRepo;
use crate::utils::error::{ApiError, ApiResult};
use crate::utils::serde_helpers::JsonOption;

pub struct BusinessService<B: BusinessRepo> {
    business_repo: B,
}

impl<B: BusinessRepo> BusinessService<B> {
    pub fn new(business_repo: B) -> Self {
        Self { business_repo }
    }
}
