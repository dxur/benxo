use common::models::ObjectId;
use mongodb::{options::ClientOptions, Client, Database};
use std::ops::Deref;

pub const DEFAULT_UUID: ObjectId = ObjectId::from_bytes([0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);

pub struct DB(Database);

impl DB {
    pub async fn new(uri: &str, name: &str) -> Self {
        let client_options = ClientOptions::parse(uri).await.unwrap();
        DB(Client::with_options(client_options).unwrap().database(name))
    }
}

impl Deref for DB {
    type Target = Database;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
