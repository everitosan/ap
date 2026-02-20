use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::modules::matching::domain::{MatchingQueueEntry, MatchingRepository, Pairing};
use crate::modules::users::domain::UserRepository;
use crate::shared::AppError;

pub enum MatchResult {
    Matched { partner_id: Uuid, affinity_score: i32 },
    Queued { queued_at: DateTime<Utc> },
    AlreadyPaired { partner_id: Uuid },
    AlreadyInQueue { queued_at: DateTime<Utc> },
}

fn topics_to_bitmask(topic_ids: &[i32]) -> i32 {
    topic_ids.iter().fold(0, |mask, &id| {
        mask | (1 << (id - 1))
    })
}

pub async fn request_match<MR: MatchingRepository, UR: UserRepository>(
    user_id: Uuid,
    matching_repo: &MR,
    user_repo: &UR,
) -> Result<MatchResult, AppError> {
    if let Some(existing_pairing) = matching_repo.find_pairing_for_user(user_id).await? {
        let partner_id = existing_pairing.get_partner_id(user_id)
            .ok_or_else(|| AppError::Internal("Invalid pairing state".into()))?;
        return Ok(MatchResult::AlreadyPaired { partner_id });
    }

    if let Some(queue_entry) = matching_repo.find_in_queue(user_id).await? {
        return Ok(MatchResult::AlreadyInQueue { queued_at: queue_entry.queued_at });
    }

    let user = user_repo.find_by_id(user_id).await?
        .ok_or_else(|| AppError::NotFound("User not found".into()))?;

    let user_topics = topics_to_bitmask(&user.topics);

    if let Some(candidate) = matching_repo.find_best_match(
        user_id,
        user_topics,
        &user.blocked_users,
    ).await? {
        let pairing = Pairing::new(user_id, candidate.user_id, candidate.affinity_score);
        matching_repo.execute_match(candidate.user_id, &pairing).await?;

        return Ok(MatchResult::Matched {
            partner_id: candidate.user_id,
            affinity_score: candidate.affinity_score,
        });
    }

    let queue_entry = MatchingQueueEntry::new(user_id);
    let queued_at = queue_entry.queued_at;
    matching_repo.add_to_queue(&queue_entry).await?;

    Ok(MatchResult::Queued { queued_at })
}
