use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct MatchingQueueEntry {
    pub user_id: Uuid,
    pub queued_at: DateTime<Utc>,
}

impl MatchingQueueEntry {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id,
            queued_at: Utc::now(),
        }
    }
}
