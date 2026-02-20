use uuid::Uuid;

use crate::modules::auth::domain::User;
use crate::modules::users::domain::UserRepository;
use crate::shared::AppError;

pub async fn get_user<R: UserRepository>(
    user_id: Uuid,
    repository: &R,
) -> Result<User, AppError> {
    match repository.find_by_id(user_id).await? {
        Some(user) => Ok(user),
        None => Err(AppError::NotFound("User not found".into())),
    }
}
