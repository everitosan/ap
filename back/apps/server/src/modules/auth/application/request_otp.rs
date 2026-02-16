use uuid::Uuid;

use crate::modules::auth::domain::{
    PhoneNumber, User, UserRepository, ValidationCode, ValidationCodeRepository,
};
use crate::modules::auth::infrastructure::{OtpGenerator, PhoneNotifier};
use crate::shared::AppError;

pub struct RequestOtpResult {
    pub user_id: Uuid,
    pub is_new_user: bool,
}

/// Request OTP for register/login
/// - Validates phone number
/// - Creates user if not exists
/// - Generates OTP code
/// - Saves validation code
/// - Sends notification (SMS)
pub async fn request_otp(
    phone: &str,
    user_repo: &dyn UserRepository,
    code_repo: &dyn ValidationCodeRepository,
    otp_generator: &dyn OtpGenerator,
    notifier: &dyn PhoneNotifier,
) -> Result<RequestOtpResult, AppError> {
    let phone = PhoneNumber::new(phone)?;

    // Find or create user
    let (user, is_new_user) = match user_repo.find_by_phone(phone.value()).await? {
        Some(user) => (user, false),
        None => {
            let user = User::new(phone.value().to_string());
            user_repo.create(&user).await?;
            (user, true)
        }
    };

    // Delete any existing codes for this user
    code_repo.delete_by_user_id(user.id).await?;

    // Generate and save new code
    let otp = otp_generator.generate();
    let validation_code = ValidationCode::new(user.id, otp.clone());
    code_repo.create(&validation_code).await?;

    // Send notification
    notifier.send(phone.value(), &otp).await?;

    Ok(RequestOtpResult {
        user_id: user.id,
        is_new_user,
    })
}
