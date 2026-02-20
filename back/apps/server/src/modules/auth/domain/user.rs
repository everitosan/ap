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
    pub topics: Vec<i32>,
    pub blocked_users: Vec<Uuid>,
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
            topics: Vec::new(),
            blocked_users: Vec::new(),
            created: Utc::now(),
            last_login: None,
        }
    }
}
