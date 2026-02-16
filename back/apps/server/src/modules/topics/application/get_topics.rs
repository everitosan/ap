use crate::modules::topics::domain::{Topic, TopicRepository};
use crate::shared::AppError;

/// Use case: Get all topics from catalogue
pub struct GetTopicsUseCase;

impl GetTopicsUseCase {
    pub async fn execute(repository: &dyn TopicRepository) -> Result<Vec<Topic>, AppError> {
        repository.get_all().await
    }
}
