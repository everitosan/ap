use async_trait::async_trait;

use super::topic::Topic;
use crate::shared::AppError;

/// Port for topic persistence
#[async_trait]
pub trait TopicRepository: Send + Sync {
    async fn get_all(&self) -> Result<Vec<Topic>, AppError>;
}
