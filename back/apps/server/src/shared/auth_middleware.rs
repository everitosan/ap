use actix_web::{dev::Payload, web, FromRequest, HttpRequest};
use std::future::{ready, Ready};
use uuid::Uuid;

use crate::modules::auth::domain::SessionData;
use super::{AppError, AppState};

const SESSION_COOKIE_NAME: &str = "ap";

/// Extractor that validates the session cookie and provides session data
pub struct AuthSession {
    pub user_id: Uuid,
    pub phone: String,
}

impl FromRequest for AuthSession {
    type Error = AppError;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        ready(extract_session(req))
    }
}

fn extract_session(req: &HttpRequest) -> Result<AuthSession, AppError> {
    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| AppError::Internal("App state not found".into()))?;

    let cookie = req
        .cookie(SESSION_COOKIE_NAME)
        .ok_or_else(|| AppError::Unauthorized("No session found".into()))?;

    let decrypted = state
        .cookie_encryptor
        .decrypt(cookie.value())
        .ok_or_else(|| AppError::Unauthorized("Invalid session".into()))?;

    let session_data: SessionData = serde_json::from_str(&decrypted)
        .map_err(|_| AppError::Unauthorized("Invalid session data".into()))?;

    Ok(AuthSession {
        user_id: session_data.user_id,
        phone: session_data.phone,
    })
}
