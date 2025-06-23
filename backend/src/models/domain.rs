use macros::Model;
use serde::{Deserialize, Serialize};

use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DomainFetch {
    pub domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DomainList {
    pub store_id: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DomainCreate {
    pub domain: String,
    pub store_id: String,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct DomainDelete {
    pub domain: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct DomainPublic {
    pub domain: String,
    pub store_id: String,
}

#[derive(Model)]
#[model(public=DomainPublic, fetch=DomainFetch, create=DomainCreate, delete=DomainDelete)]
pub struct Domain;
