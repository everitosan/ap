use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::matching::domain::MatchingRepository;
use crate::modules::users::domain::UserRepository;
use crate::shared::AppError;

pub struct PartnerInfo {
    pub username: Option<String>,
    pub address: Option<serde_json::Value>,
}

pub enum MatchStatus {
    Paired { partner: PartnerInfo },
    Queued { queued_at: DateTime<Utc> },
    Idle,
}

pub async fn get_match_status<MR: MatchingRepository, UR: UserRepository>(
    user_id: Uuid,
    matching_repo: &MR,
    user_repo: &UR,
) -> Result<MatchStatus, AppError> {
    if let Some(pairing) = matching_repo.find_pairing_for_user(user_id).await? {
        let partner_id = pairing.get_partner_id(user_id)
            .ok_or_else(|| AppError::Internal("Invalid pairing state".into()))?;

        let partner = user_repo.find_by_id(partner_id).await?
            .ok_or_else(|| AppError::NotFound("Partner not found".into()))?;

        return Ok(MatchStatus::Paired {
            partner: PartnerInfo {
                username: partner.username,
                address: partner.address,
            },
        });
    }

    if let Some(queue_entry) = matching_repo.find_in_queue(user_id).await? {
        return Ok(MatchStatus::Queued { queued_at: queue_entry.queued_at });
    }

    Ok(MatchStatus::Idle)
}
