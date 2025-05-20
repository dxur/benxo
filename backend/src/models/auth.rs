use serde::Deserialize;

use super::*;

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct LoginCredentials {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct RegisterCredentials {
    pub name: String,
    pub email: String,
    pub password: String,
}
