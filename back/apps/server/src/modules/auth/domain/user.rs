use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

/// User entity
#[derive(Debug, Clone, Serialize)]
pub struct User {
    pub id: Uuid,
    pub phone: String,
    pub username: Option<String>,
    pub address: Option<serde_json::Value>,
    pub topics: Option<serde_json::Value>,
    pub created: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

impl User {
    pub fn new(phone: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            phone,
            username: None,
            address: None,
            topics: None,
            created: Utc::now(),
            last_login: None,
        }
    }
}
