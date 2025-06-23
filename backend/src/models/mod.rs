pub mod auth;
pub mod channel;
pub mod delivery;
pub mod domain;
pub mod order;
pub mod product;
pub mod settings;
pub mod store;
pub mod theme;
pub mod user;

use std::fmt::Debug;

pub use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, Default, PartialEq, Eq)]
pub struct Void;

impl<T> From<&T> for Void {
    fn from(_: &T) -> Self {
        Void
    }
}

pub trait Model {
    type Public: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

pub trait Fetchable: Model {
    type Fetch: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

pub trait Creatable: Model {
    type Create: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

pub trait Updatable: Model {
    type Update: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

pub trait Deletable: Model {
    type Delete: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

pub trait Filterable: Model {
    type Filter: Debug + Send + Sync + Serialize + for<'a> Deserialize<'a>;
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
