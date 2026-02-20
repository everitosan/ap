use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::modules::auth::domain::User;
use crate::modules::users::domain::UserRepository;
use crate::shared::AppError;

// Bitmask conversion utilities

/// Converts a bitmask to an array of topic IDs
/// Example: 21 (binary: 10101) -> [1, 3, 5]
fn bitmask_to_topics(bitmask: i32) -> Vec<i32> {
    let mut topics = Vec::new();
    for id in 1..=32 {  // Support up to 32 topics (i32 has 32 bits)
        if (bitmask & (1 << (id - 1))) != 0 {
            topics.push(id);
        }
    }
    topics
}

/// Converts an array of topic IDs to a bitmask
/// Example: [1, 3, 5] -> 1 + 4 + 16 = 21 (binary: 10101)
fn topics_to_bitmask(topic_ids: &[i32]) -> i32 {
    topic_ids.iter().fold(0, |mask, &id| {
        mask | (1 << (id - 1))
    })
}

#[derive(FromRow)]
struct UserRow {
    id: uuid::Uuid,
    phone: String,
    username: Option<String>,
    address: Option<serde_json::Value>,
    topics: i32,
    blocked_users: serde_json::Value,
    created: chrono::DateTime<chrono::Utc>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
}

fn parse_blocked_users(json: &serde_json::Value) -> Vec<Uuid> {
    json.as_array()
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().and_then(|s| Uuid::parse_str(s).ok()))
                .collect()
        })
        .unwrap_or_default()
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            phone: row.phone,
            username: row.username,
            address: row.address,
            topics: bitmask_to_topics(row.topics),
            blocked_users: parse_blocked_users(&row.blocked_users),
            created: row.created,
            last_login: row.last_login,
        }
    }
}

pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, phone, username, address, topics, blocked_users, created, last_login FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(User::from))
    }

    async fn update_profile(&self, id: Uuid, username: &str, topic_ids: &[i32]) -> Result<(), AppError> {
        let topics_bitmask = topics_to_bitmask(topic_ids);  // Convert Vec<i32> to bitmask
        sqlx::query("UPDATE users SET username = $2, topics = $3 WHERE id = $1")
            .bind(id)
            .bind(username)
            .bind(topics_bitmask)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn update_address(&self, id: Uuid, address: &serde_json::Value) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET address = $2 WHERE id = $1")
            .bind(id)
            .bind(address)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
