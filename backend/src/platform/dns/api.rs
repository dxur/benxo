use chrono::{DateTime, Utc};
use o2o::o2o;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use super::domain::*;
use crate::types::id::Id;

#[derive(Debug, Serialize, o2o, TS)]
#[ts(export, bound = "")]
#[from_owned(DomainRecord)]
pub struct DomainDto {
    #[from(@._id.into())]
    pub id: Id,
    pub domain: String,
    #[from(~.into())]
    pub business_id: Id,
    #[from(~.to_chrono())]
    pub expires_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, TS)]
#[ts(export, bound = "")]
pub struct DomainListResponse {
    pub domains: Vec<DomainDto>,
}
