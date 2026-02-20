use uuid::Uuid;

use crate::modules::users::domain::UserRepository;
use crate::shared::AppError;

pub async fn update_address<R: UserRepository>(
    user_id: Uuid,
    address: &serde_json::Value,
    repository: &R,
) -> Result<(), AppError> {
    repository.update_address(user_id, address).await
}
