pub mod auth;
pub mod channel;
// pub mod delivery;
pub mod domain;
pub mod file;
pub mod order;
pub mod product;
pub mod settings;
pub mod store;
pub mod user;

pub use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use ts_rs::TS;

pub use crate::db::*;
use crate::utils::types::IntoInner;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ById<T = ObjectId> {
    pub id: T,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Pagination {
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}

impl Pagination {
    pub fn limit(&self) -> usize {
        self.per_page.unwrap_or(10).min(100)
    }

    pub fn offset(&self) -> usize {
        let page = self.page.unwrap_or(1).max(1);
        (page - 1) * self.limit()
    }

    pub fn page(&self) -> usize {
        self.page.unwrap_or(1).max(1)
    }

    pub fn per_page(&self) -> usize {
        self.per_page.unwrap_or(10).min(100)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

impl<T> Default for Page<T> {
    fn default() -> Self {
        Self {
            data: vec![],
            total: 0,
            page: 1,
            per_page: 10,
        }
    }
}

impl<T, U: Into<T>> IntoInner<Page<T>> for Page<U> {
    fn into_inner(self) -> Page<T> {
        Page {
            data: self.data.into_iter().map(|v| v.into()).collect(),
            total: self.total,
            page: self.page,
            per_page: self.per_page,
        }
    }
}
