use actix_web::{web, HttpResponse};

use crate::modules::auth::infrastructure::{get_user, register_or_login, validate_code};
use crate::modules::topics::infrastructure::get_topics;
use crate::shared::ApiResponse;

/// Health check endpoint
async fn health() -> HttpResponse {
    HttpResponse::Ok().json(ApiResponse::new("ok"))
}

/// Configura todas las rutas de la API
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/health", web::get().to(health))
            .route("/topics", web::get().to(get_topics))
            .route("/register", web::post().to(register_or_login))
            .route("/login", web::post().to(register_or_login))
            .route("/phone-validate", web::post().to(validate_code))
            .route("/user", web::get().to(get_user)),
    );
}
