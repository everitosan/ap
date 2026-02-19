use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::modules::auth::application::{request_otp, verify_otp};
use crate::modules::auth::domain::{SessionData, UserRepository};
use crate::modules::auth::infrastructure::{
    PostgresUserRepository, PostgresValidationCodeRepository, RandomOtpGenerator,
};
use crate::shared::{ApiResponse, AppState, AppError};

const TMP_COOKIE_NAME: &str = "ap_tmp";
const SESSION_COOKIE_NAME: &str = "ap";
const SESSION_DURATION_DAYS: i64 = 7;

#[derive(Deserialize)]
pub struct PhoneRequest {
    telephone: String,
}

#[derive(Deserialize)]
pub struct CodeRequest {
    code: String,
}

/// POST /api/v1/register
/// POST /api/v1/login
pub async fn register_or_login(
    state: web::Data<AppState>,
    body: web::Json<PhoneRequest>,
) -> HttpResponse {
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    let code_repo = PostgresValidationCodeRepository::new(state.db_pool.clone());
    let otp_generator = RandomOtpGenerator;

    match request_otp(
        &body.telephone,
        &user_repo,
        &code_repo,
        &otp_generator,
        state.phone_notifier.as_ref(),
    )
    .await
    {
        Ok(result) => {
            let encrypted_value = state.cookie_encryptor.encrypt(&result.user_id.to_string());
            let cookie = Cookie::build(TMP_COOKIE_NAME, encrypted_value)
                .path("/")
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish();

            HttpResponse::Created()
                .cookie(cookie)
                .json(ApiResponse::new("OTP sent"))
        }
        Err(e) => e.error_response(),
    }
}

/// POST /api/v1/phone-validate
pub async fn validate_code(
    req: HttpRequest,
    state: web::Data<AppState>,
    body: web::Json<CodeRequest>,
) -> HttpResponse {
    // Get user_id from encrypted cookie
    let user_id = match req.cookie(TMP_COOKIE_NAME) {
        Some(cookie) => {
            let decrypted = match state.cookie_encryptor.decrypt(cookie.value()) {
                Some(value) => value,
                None => return AppError::BadRequest("Invalid session".into()).error_response(),
            };
            match Uuid::parse_str(&decrypted) {
                Ok(id) => id,
                Err(_) => return AppError::BadRequest("Invalid session".into()).error_response(),
            }
        }
        None => return AppError::BadRequest("No session found".into()).error_response(),
    };

    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    let code_repo = PostgresValidationCodeRepository::new(state.db_pool.clone());

    if let Err(e) = verify_otp(user_id, &body.code, &user_repo, &code_repo).await {
        return e.error_response();
    }

    // Fetch user to get phone number
    let user = match user_repo.find_by_id(user_id).await {
        Ok(Some(user)) => user,
        Ok(None) => return AppError::BadRequest("User not found".into()).error_response(),
        Err(e) => return e.error_response(),
    };

    // Create session data
    let session = SessionData::new(user.id, user.phone);
    let session_json = serde_json::to_string(&session).unwrap();
    let encrypted_session = state.cookie_encryptor.encrypt(&session_json);

    // Create session cookie (valid for 1 week)
    let session_cookie = Cookie::build(SESSION_COOKIE_NAME, encrypted_session)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .max_age(Duration::days(SESSION_DURATION_DAYS))
        .finish();

    // Remove temporary cookie
    let mut remove_tmp_cookie = Cookie::build(TMP_COOKIE_NAME, "")
        .path("/")
        .finish();
    remove_tmp_cookie.make_removal();

    HttpResponse::Ok()
        .cookie(session_cookie)
        .cookie(remove_tmp_cookie)
        .json(ApiResponse::new("Verified"))
}
