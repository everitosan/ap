use actix_web::{web, HttpResponse, ResponseError};

use crate::modules::topics::application::GetTopicsUseCase;
use crate::modules::topics::infrastructure::PostgresTopicRepository;
use crate::shared::{ApiResponse, AppState, AuthSession};

/// GET /api/v1/topics
pub async fn get_topics(
    _session: AuthSession,
    state: web::Data<AppState>,
) -> HttpResponse {
    let repository = PostgresTopicRepository::new(state.db_pool.clone());

    match GetTopicsUseCase::execute(&repository).await {
        Ok(topics) => HttpResponse::Ok().json(ApiResponse::new(topics)),
        Err(e) => e.error_response(),
    }
}
