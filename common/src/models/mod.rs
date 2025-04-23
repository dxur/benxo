pub mod product;

pub use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

pub trait Model {
    type ModelFetch: Send + Sync + Serialize + for<'a> Deserialize<'a>;
    type ModelCreate: Send + Sync + Serialize + for<'a> Deserialize<'a>;
    type ModelUpdate: Send + Sync + Serialize + for<'a> Deserialize<'a>;
    type ModelDelete: Send + Sync + Serialize + for<'a> Deserialize<'a>;
    type ModelPublic: Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

pub trait ModelFilter: Model {
    type ModelFilter: Send + Sync + Serialize + for<'a> Deserialize<'a>;
}

#[derive(Debug, Serialize, Deserialize)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Page<T> {
    pub data: Vec<T>,
    pub total: usize,
    pub page: usize,
    pub per_page: usize,
}

impl<T> Page<T> {
    pub fn total_pages(&self) -> usize {
        (self.total + self.per_page - 1) / self.per_page
    }

    pub fn has_next(&self) -> bool {
        self.page < self.total_pages()
    }

    pub fn has_prev(&self) -> bool {
        self.page > 1
    }

    pub fn start_index(&self) -> usize {
        (self.page - 1) * self.per_page + 1
    }

    pub fn end_index(&self) -> usize {
        usize::min(self.page * self.per_page, self.total)
    }
}
