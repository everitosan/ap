use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize)]
pub struct Pairing {
    pub id: Uuid,
    pub user_a_id: Uuid,
    pub user_b_id: Uuid,
    pub affinity_score: i32,
    pub created_at: DateTime<Utc>,
}

impl Pairing {
    pub fn new(user1_id: Uuid, user2_id: Uuid, affinity_score: i32) -> Self {
        let (user_a_id, user_b_id) = if user1_id < user2_id {
            (user1_id, user2_id)
        } else {
            (user2_id, user1_id)
        };

        Self {
            id: Uuid::new_v4(),
            user_a_id,
            user_b_id,
            affinity_score,
            created_at: Utc::now(),
        }
    }

    pub fn get_partner_id(&self, user_id: Uuid) -> Option<Uuid> {
        if self.user_a_id == user_id {
            Some(self.user_b_id)
        } else if self.user_b_id == user_id {
            Some(self.user_a_id)
        } else {
            None
        }
    }
}
