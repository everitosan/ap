use chrono::{DateTime, Duration, Utc};
use uuid::Uuid;

/// ValidationCode entity
#[derive(Debug, Clone)]
pub struct ValidationCode {
    pub user_id: Uuid,
    pub code: String,
    pub expires_at: DateTime<Utc>,
}

impl ValidationCode {
    /// Creates a new validation code with 5 minute expiration
    pub fn new(user_id: Uuid, code: String) -> Self {
        Self {
            user_id,
            code,
            expires_at: Utc::now() + Duration::minutes(5),
        }
    }

    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
