use uuid::Uuid;

use crate::modules::auth::domain::{UserRepository, ValidationCode, ValidationCodeRepository};
use crate::modules::auth::infrastructure::{OtpGenerator, PhoneNotifier};
use crate::shared::AppError;

/// Resend OTP for an existing user
/// - Fetches user by ID
/// - Deletes any existing codes
/// - Generates new OTP code
/// - Sends notification
pub async fn resend_otp(
    user_id: Uuid,
    user_repo: &dyn UserRepository,
    code_repo: &dyn ValidationCodeRepository,
    otp_generator: &dyn OtpGenerator,
    notifier: &dyn PhoneNotifier,
) -> Result<(), AppError> {
    // Find user
    let user = user_repo
        .find_by_id(user_id)
        .await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    // Delete any existing codes for this user
    code_repo.delete_by_user_id(user.id).await?;

    // Generate and save new code
    let otp = otp_generator.generate();
    let validation_code = ValidationCode::new(user.id, otp.clone());
    code_repo.create(&validation_code).await?;

    // Send notification
    notifier.send(&user.phone, &otp).await?;

    Ok(())
}
