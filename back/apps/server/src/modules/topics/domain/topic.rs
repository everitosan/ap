use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Topic entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: Uuid,
    pub name: String,
}
