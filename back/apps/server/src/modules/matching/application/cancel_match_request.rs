use uuid::Uuid;

use crate::modules::matching::domain::MatchingRepository;
use crate::shared::AppError;

pub enum CancelResult {
    RemovedFromQueue,
    Unpaired,
    NotInMatchingState,
}

pub async fn cancel_match_request<R: MatchingRepository>(
    user_id: Uuid,
    repository: &R,
) -> Result<CancelResult, AppError> {
    if repository.is_user_in_queue(user_id).await? {
        repository.remove_from_queue(user_id).await?;
        return Ok(CancelResult::RemovedFromQueue);
    }

    if repository.is_user_paired(user_id).await? {
        repository.delete_pairing_for_user(user_id).await?;
        return Ok(CancelResult::Unpaired);
    }

    Ok(CancelResult::NotInMatchingState)
}
