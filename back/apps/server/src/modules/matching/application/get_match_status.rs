use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::matching::domain::MatchingRepository;
use crate::shared::AppError;

pub enum MatchStatus {
    Paired { partner_id: Uuid },
    Queued { queued_at: DateTime<Utc> },
    Idle,
}

pub async fn get_match_status<R: MatchingRepository>(
    user_id: Uuid,
    repository: &R,
) -> Result<MatchStatus, AppError> {
    if let Some(pairing) = repository.find_pairing_for_user(user_id).await? {
        let partner_id = pairing.get_partner_id(user_id)
            .ok_or_else(|| AppError::Internal("Invalid pairing state".into()))?;
        return Ok(MatchStatus::Paired { partner_id });
    }

    if let Some(queue_entry) = repository.find_in_queue(user_id).await? {
        return Ok(MatchStatus::Queued { queued_at: queue_entry.queued_at });
    }

    Ok(MatchStatus::Idle)
}
