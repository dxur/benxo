pub mod api;
pub mod models;
pub mod validators;

// just for cargo check
#[cfg(not(feature = "wasm"))]
mod extractors;
#[cfg(not(feature = "wasm"))]
#[macro_use]
extern crate dotenv_codegen;
