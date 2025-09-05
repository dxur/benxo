use bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DomainRecord {
    pub _id: ObjectId,
    pub domain: String,
    pub business_id: ObjectId,
    pub expires_at: DateTime,
}

impl DomainRecord {
    pub fn new(domain: String, business_id: ObjectId, ttl_hours: u32) -> Self {
        let expires_at =
            DateTime::from_chrono(chrono::Utc::now() + chrono::Duration::hours(ttl_hours as i64));

        Self {
            _id: ObjectId::new(),
            domain: domain.to_lowercase(),
            business_id,
            expires_at,
        }
    }

    pub fn is_expired(&self) -> bool {
        DateTime::now() > self.expires_at
    }

    pub fn refresh(&mut self, ttl_seconds: u32) {
        self.expires_at = DateTime::from_chrono(
            chrono::Utc::now() + chrono::Duration::seconds(ttl_seconds as i64),
        );
    }
}
