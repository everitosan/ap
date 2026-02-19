use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::modules::auth::application::{request_otp, resend_otp, verify_otp};
use crate::modules::auth::domain::{SessionData, UserRepository};
use crate::modules::auth::infrastructure::{
    PostgresUserRepository, PostgresValidationCodeRepository, RandomOtpGenerator,
};
use crate::shared::{ApiResponse, AppState, AppError, AuthSession};

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

#[derive(Serialize)]
pub struct UserResponse {
    pub username: Option<String>,
    pub topics: Option<serde_json::Value>,
    pub created: DateTime<Utc>,
    pub filled_address: bool,
}

/// GET /api/v1/user
pub async fn get_user(
    session: AuthSession,
    state: web::Data<AppState>,
) -> HttpResponse {
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    match user_repo.find_by_id(session.user_id).await {
        Ok(Some(user)) => {
            let response = UserResponse {
                username: user.username,
                topics: user.topics,
                created: user.created,
                filled_address: user.address.is_some(),
            };
            HttpResponse::Ok().json(ApiResponse::new(response))
        }
        Ok(None) => AppError::NotFound("User not found".into()).error_response(),
        Err(e) => e.error_response(),
    }
}

/// POST /api/v1/resend-code
pub async fn resend_code(
    req: HttpRequest,
    state: web::Data<AppState>,
) -> HttpResponse {
    // Get user_id from encrypted ap_tmp cookie
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
    let otp_generator = RandomOtpGenerator;

    match resend_otp(
        user_id,
        &user_repo,
        &code_repo,
        &otp_generator,
        state.phone_notifier.as_ref(),
    )
    .await
    {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::new("OTP resent")),
        Err(e) => e.error_response(),
    }
}

#[derive(Deserialize)]
pub struct FillProfileRequest {
    username: String,
    topics: serde_json::Value,
}

/// POST /api/v1/fill-profile
pub async fn fill_profile(
    session: AuthSession,
    state: web::Data<AppState>,
    body: web::Json<FillProfileRequest>,
) -> HttpResponse {
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    match user_repo.update_profile(session.user_id, &body.username, &body.topics).await {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::new("Profile updated")),
        Err(e) => e.error_response(),
    }
}
