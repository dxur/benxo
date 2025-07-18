use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};

use crate::types::{id::Id, name::Name};

use super::domain::*;

#[derive(Debug, Deserialize)]
pub struct BusinessCreate {
    pub name: Name,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, o2o)]
#[from_owned(BusinessRecord)]
pub struct BusinessView {
    #[from(@._id.into())]
    pub id: Id,
    pub name: Name,
    pub description: Option<String>,
    #[from(~.to_hex())]
    pub owner_id: String,
    pub plan_type: PlanType,
    #[from(~.to_chrono())]
    pub plan_expires_at: DateTime<Utc>,
    pub status: BusinessStatus,
    pub settings: BusinessSettings,
    #[from(~.to_chrono())]
    pub created_at: DateTime<Utc>,
    #[from(~.to_chrono())]
    pub updated_at: DateTime<Utc>,
}
