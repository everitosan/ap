use async_trait::async_trait;
use uuid::Uuid;

use crate::modules::auth::domain::User;
use crate::shared::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn update_profile(&self, id: Uuid, username: &str, topic_ids: &[i32]) -> Result<(), AppError>;
    async fn update_address(&self, id: Uuid, address: &serde_json::Value) -> Result<(), AppError>;
}
