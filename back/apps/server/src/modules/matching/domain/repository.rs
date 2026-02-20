use async_trait::async_trait;
use uuid::Uuid;

use super::{MatchingQueueEntry, Pairing};
use crate::shared::AppError;

pub struct MatchCandidate {
    pub user_id: Uuid,
    pub affinity_score: i32,
}

#[async_trait]
pub trait MatchingRepository: Send + Sync {
    async fn add_to_queue(&self, entry: &MatchingQueueEntry) -> Result<(), AppError>;
    async fn remove_from_queue(&self, user_id: Uuid) -> Result<(), AppError>;
    async fn find_in_queue(&self, user_id: Uuid) -> Result<Option<MatchingQueueEntry>, AppError>;

    async fn find_best_match(
        &self,
        user_id: Uuid,
        user_topics: i32,
        user_blocked_users: &[Uuid],
    ) -> Result<Option<MatchCandidate>, AppError>;

    async fn create_pairing(&self, pairing: &Pairing) -> Result<(), AppError>;
    async fn find_pairing_for_user(&self, user_id: Uuid) -> Result<Option<Pairing>, AppError>;
    async fn delete_pairing_for_user(&self, user_id: Uuid) -> Result<(), AppError>;

    async fn execute_match(
        &self,
        matched_user_id: Uuid,
        pairing: &Pairing,
    ) -> Result<(), AppError>;

    async fn is_user_paired(&self, user_id: Uuid) -> Result<bool, AppError>;
    async fn is_user_in_queue(&self, user_id: Uuid) -> Result<bool, AppError>;
}
