use serde::Deserialize;

const MAX_VERSION: u32 = 1;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: u32,
    pub name: String,
    pub auther: String,
    pub description: String,
}
