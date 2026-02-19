use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
pub struct SessionData {
    pub user_id: Uuid,
    pub phone: String,
}

impl SessionData {
    pub fn new(user_id: Uuid, phone: String) -> Self {
        Self { user_id, phone }
    }
}
