use serde::{Deserialize, Serialize};

/// Topic entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Topic {
    pub id: i32,
    pub name: String,
}
