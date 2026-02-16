use uuid::Uuid;

use crate::modules::auth::domain::{UserRepository, ValidationCodeRepository};
use crate::shared::AppError;

/// Verify OTP code for a user
/// - Validates code exists and matches
/// - Checks expiration
/// - Updates last_login
/// - Deletes used code
pub async fn verify_otp(
    user_id: Uuid,
    code: &str,
    user_repo: &dyn UserRepository,
    code_repo: &dyn ValidationCodeRepository,
) -> Result<(), AppError> {
    // Find validation code
    let validation_code = code_repo
        .find_by_user_id(user_id)
        .await?
        .ok_or_else(|| AppError::BadRequest("No pending code found".to_string()))?;

    // Check if expired
    if validation_code.is_expired() {
        code_repo.delete_by_user_id(user_id).await?;
        return Err(AppError::BadRequest("Code expired".to_string()));
    }

    // Validate code
    if validation_code.code != code {
        return Err(AppError::BadRequest("Invalid code".to_string()));
    }

    // Update last_login
    user_repo.update_last_login(user_id).await?;

    // Delete used code
    code_repo.delete_by_user_id(user_id).await?;

    Ok(())
}
