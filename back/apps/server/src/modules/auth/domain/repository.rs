use async_trait::async_trait;
use uuid::Uuid;

use super::user::User;
use super::validation_code::ValidationCode;
use crate::shared::AppError;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, AppError>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn create(&self, user: &User) -> Result<(), AppError>;
    async fn update_last_login(&self, id: Uuid) -> Result<(), AppError>;
    async fn update_profile(&self, id: Uuid, username: &str, topics: &serde_json::Value) -> Result<(), AppError>;
}

#[async_trait]
pub trait ValidationCodeRepository: Send + Sync {
    async fn create(&self, code: &ValidationCode) -> Result<(), AppError>;
    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<ValidationCode>, AppError>;
    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), AppError>;
}
