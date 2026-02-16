use actix_web::cookie::{Cookie, SameSite};
use actix_web::{web, HttpRequest, HttpResponse, ResponseError};
use serde::Deserialize;
use uuid::Uuid;

use crate::modules::auth::application::{request_otp, verify_otp};
use crate::modules::auth::infrastructure::{
    PostgresUserRepository, PostgresValidationCodeRepository, RandomOtpGenerator,
};
use crate::shared::{ApiResponse, AppState};

const USER_COOKIE_NAME: &str = "user_id";

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
            let cookie = Cookie::build(USER_COOKIE_NAME, result.user_id.to_string())
                .path("/")
                .http_only(true)
                .same_site(SameSite::Strict)
                .finish();

            HttpResponse::Created()
                .cookie(cookie)
                .json(ApiResponse::success("OTP sent"))
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
    // Get user_id from cookie
    let user_id = match req.cookie(USER_COOKIE_NAME) {
        Some(cookie) => match Uuid::parse_str(cookie.value()) {
            Ok(id) => id,
            Err(_) => {
                return HttpResponse::BadRequest().json(ApiResponse::<String>::error("Invalid session"))
            }
        },
        None => return HttpResponse::BadRequest().json(ApiResponse::<String>::error("No session found")),
    };

    let user_repo = PostgresUserRepository::new(state.db_pool.clone());
    let code_repo = PostgresValidationCodeRepository::new(state.db_pool.clone());

    match verify_otp(user_id, &body.code, &user_repo, &code_repo).await {
        Ok(()) => HttpResponse::Ok().json(ApiResponse::success("Verified")),
        Err(e) => e.error_response(),
    }
}
