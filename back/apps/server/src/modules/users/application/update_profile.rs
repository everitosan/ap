use uuid::Uuid;

use crate::modules::users::domain::UserRepository;
use crate::shared::AppError;

pub async fn update_profile<R: UserRepository>(
    user_id: Uuid,
    username: &str,
    topic_ids: &[i32],
    repository: &R,
) -> Result<(), AppError> {
    repository.update_profile(user_id, username, topic_ids).await
}
