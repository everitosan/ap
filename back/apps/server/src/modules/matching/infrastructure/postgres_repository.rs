use async_trait::async_trait;
use sqlx::{FromRow, PgPool, Row};
use uuid::Uuid;

use crate::modules::matching::domain::{MatchCandidate, MatchingQueueEntry, MatchingRepository, Pairing};
use crate::shared::AppError;

#[derive(FromRow)]
struct MatchingQueueRow {
    user_id: uuid::Uuid,
    queued_at: chrono::DateTime<chrono::Utc>,
}

impl From<MatchingQueueRow> for MatchingQueueEntry {
    fn from(row: MatchingQueueRow) -> Self {
        MatchingQueueEntry {
            user_id: row.user_id,
            queued_at: row.queued_at,
        }
    }
}

#[derive(FromRow)]
struct PairingRow {
    id: uuid::Uuid,
    user_a_id: uuid::Uuid,
    user_b_id: uuid::Uuid,
    affinity_score: i32,
    created_at: chrono::DateTime<chrono::Utc>,
}

impl From<PairingRow> for Pairing {
    fn from(row: PairingRow) -> Self {
        Pairing {
            id: row.id,
            user_a_id: row.user_a_id,
            user_b_id: row.user_b_id,
            affinity_score: row.affinity_score,
            created_at: row.created_at,
        }
    }
}

pub struct PostgresMatchingRepository {
    pool: PgPool,
}

impl PostgresMatchingRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl MatchingRepository for PostgresMatchingRepository {
    async fn add_to_queue(&self, entry: &MatchingQueueEntry) -> Result<(), AppError> {
        sqlx::query("INSERT INTO matching_queue (user_id, queued_at) VALUES ($1, $2)")
            .bind(entry.user_id)
            .bind(entry.queued_at)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn remove_from_queue(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM matching_queue WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn find_in_queue(&self, user_id: Uuid) -> Result<Option<MatchingQueueEntry>, AppError> {
        let row: Option<MatchingQueueRow> = sqlx::query_as(
            "SELECT user_id, queued_at FROM matching_queue WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(MatchingQueueEntry::from))
    }

    async fn find_best_match(
        &self,
        user_id: Uuid,
        user_topics: i32,
        user_blocked_users: &[Uuid],
    ) -> Result<Option<MatchCandidate>, AppError> {
        let blocked_users_str: Vec<String> = user_blocked_users.iter().map(|id| id.to_string()).collect();

        let row = sqlx::query(
            r#"
            WITH blocked_by_requester AS (
                SELECT unnest($3::text[]) AS blocked_id
            ),
            blocked_by_candidate AS (
                SELECT mq.user_id
                FROM matching_queue mq
                JOIN users u ON u.id = mq.user_id
                WHERE u.blocked_users @> to_jsonb($1::text)
            )
            SELECT mq.user_id, BIT_COUNT(u.topics & $2) as affinity_score
            FROM matching_queue mq
            JOIN users u ON u.id = mq.user_id
            WHERE mq.user_id != $1
              AND mq.user_id::text NOT IN (SELECT blocked_id FROM blocked_by_requester)
              AND mq.user_id NOT IN (SELECT user_id FROM blocked_by_candidate)
              AND (u.topics & $2) > 0
            ORDER BY affinity_score DESC, mq.queued_at ASC
            LIMIT 1
            "#
        )
        .bind(user_id)
        .bind(user_topics)
        .bind(&blocked_users_str)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(|r| MatchCandidate {
            user_id: r.get("user_id"),
            affinity_score: r.get::<i64, _>("affinity_score") as i32,
        }))
    }

    async fn create_pairing(&self, pairing: &Pairing) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO pairings (id, user_a_id, user_b_id, affinity_score, created_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(pairing.id)
        .bind(pairing.user_a_id)
        .bind(pairing.user_b_id)
        .bind(pairing.affinity_score)
        .bind(pairing.created_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_pairing_for_user(&self, user_id: Uuid) -> Result<Option<Pairing>, AppError> {
        let row: Option<PairingRow> = sqlx::query_as(
            "SELECT id, user_a_id, user_b_id, affinity_score, created_at FROM pairings WHERE user_a_id = $1 OR user_b_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(row.map(Pairing::from))
    }

    async fn delete_pairing_for_user(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM pairings WHERE user_a_id = $1 OR user_b_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn execute_match(
        &self,
        matched_user_id: Uuid,
        pairing: &Pairing,
    ) -> Result<(), AppError> {
        let mut tx = self.pool.begin().await?;

        sqlx::query("DELETE FROM matching_queue WHERE user_id = $1")
            .bind(matched_user_id)
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "INSERT INTO pairings (id, user_a_id, user_b_id, affinity_score, created_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(pairing.id)
        .bind(pairing.user_a_id)
        .bind(pairing.user_b_id)
        .bind(pairing.affinity_score)
        .bind(pairing.created_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn is_user_paired(&self, user_id: Uuid) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM pairings WHERE user_a_id = $1 OR user_b_id = $1) as exists"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get("exists"))
    }

    async fn is_user_in_queue(&self, user_id: Uuid) -> Result<bool, AppError> {
        let row = sqlx::query(
            "SELECT EXISTS(SELECT 1 FROM matching_queue WHERE user_id = $1) as exists"
        )
        .bind(user_id)
        .fetch_one(&self.pool)
        .await?;
        Ok(row.get("exists"))
    }
}
