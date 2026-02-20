use actix_web::{web, HttpResponse, ResponseError};
use serde::Serialize;
use uuid::Uuid;

use crate::modules::matching::application;
use crate::modules::matching::infrastructure::PostgresMatchingRepository;
use crate::modules::users::infrastructure::PostgresUserRepository;
use crate::shared::{ApiResponse, AppState, AuthSession};

#[derive(Serialize)]
pub struct MatchStatusResponse {
    pub status: String,
    pub partner_id: Option<Uuid>,
    pub queued_at: Option<chrono::DateTime<chrono::Utc>>,
}

pub async fn request_match(
    session: AuthSession,
    state: web::Data<AppState>,
) -> HttpResponse {
    let matching_repo = PostgresMatchingRepository::new(state.db_pool.clone());
    let user_repo = PostgresUserRepository::new(state.db_pool.clone());

    match application::request_match(session.user_id, &matching_repo, &user_repo).await {
        Ok(result) => {
            let response = match result {
                application::MatchResult::Matched { partner_id, .. } => {
                    MatchStatusResponse {
                        status: "paired".into(),
                        partner_id: Some(partner_id),
                        queued_at: None,
                    }
                }
                application::MatchResult::Queued { queued_at } => {
                    MatchStatusResponse {
                        status: "queued".into(),
                        partner_id: None,
                        queued_at: Some(queued_at),
                    }
                }
                application::MatchResult::AlreadyPaired { partner_id } => {
                    MatchStatusResponse {
                        status: "already_paired".into(),
                        partner_id: Some(partner_id),
                        queued_at: None,
                    }
                }
                application::MatchResult::AlreadyInQueue { queued_at } => {
                    MatchStatusResponse {
                        status: "already_queued".into(),
                        partner_id: None,
                        queued_at: Some(queued_at),
                    }
                }
            };
            HttpResponse::Ok().json(ApiResponse::new(response))
        }
        Err(e) => e.error_response(),
    }
}

pub async fn cancel_match(
    session: AuthSession,
    state: web::Data<AppState>,
) -> HttpResponse {
    let matching_repo = PostgresMatchingRepository::new(state.db_pool.clone());

    match application::cancel_match_request(session.user_id, &matching_repo).await {
        Ok(result) => {
            let message = match result {
                application::CancelResult::RemovedFromQueue => "Removed from queue",
                application::CancelResult::Unpaired => "Unpaired successfully",
                application::CancelResult::NotInMatchingState => "Not in matching state",
            };
            HttpResponse::Ok().json(ApiResponse::new(message))
        }
        Err(e) => e.error_response(),
    }
}

pub async fn get_match_status(
    session: AuthSession,
    state: web::Data<AppState>,
) -> HttpResponse {
    let matching_repo = PostgresMatchingRepository::new(state.db_pool.clone());

    match application::get_match_status(session.user_id, &matching_repo).await {
        Ok(status) => {
            let response = match status {
                application::MatchStatus::Paired { partner_id } => {
                    MatchStatusResponse {
                        status: "paired".into(),
                        partner_id: Some(partner_id),
                        queued_at: None,
                    }
                }
                application::MatchStatus::Queued { queued_at } => {
                    MatchStatusResponse {
                        status: "queued".into(),
                        partner_id: None,
                        queued_at: Some(queued_at),
                    }
                }
                application::MatchStatus::Idle => {
                    MatchStatusResponse {
                        status: "idle".into(),
                        partner_id: None,
                        queued_at: None,
                    }
                }
            };
            HttpResponse::Ok().json(ApiResponse::new(response))
        }
        Err(e) => e.error_response(),
    }
}
