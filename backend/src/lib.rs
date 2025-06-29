#![allow(dead_code)]

#[macro_use]
extern crate dotenv_codegen;

pub mod db;
pub mod events;
pub mod extractors;
pub mod middlewares;
pub mod models;
pub mod routes;
pub mod utils;
pub mod validators;

use std::sync::Arc;

pub trait State: Send + Sync + Clone {}
pub trait WithDb: State {
    fn db(&self) -> &Arc<db::Db>;
}
