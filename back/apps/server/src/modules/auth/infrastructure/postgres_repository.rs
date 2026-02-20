use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::modules::auth::domain::{User, UserRepository, ValidationCode, ValidationCodeRepository};
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

// Database models

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

#[derive(FromRow)]
struct ValidationCodeRow {
    user_id: uuid::Uuid,
    code: String,
    expires_at: chrono::DateTime<chrono::Utc>,
}

impl From<ValidationCodeRow> for ValidationCode {
    fn from(row: ValidationCodeRow) -> Self {
        ValidationCode {
            user_id: row.user_id,
            code: row.code,
            expires_at: row.expires_at,
        }
    }
}

// Repository implementations

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
    async fn find_by_phone(&self, phone: &str) -> Result<Option<User>, AppError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, phone, username, address, topics, blocked_users, created, last_login FROM users WHERE phone = $1"
        )
        .bind(phone)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(User::from))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, phone, username, address, topics, blocked_users, created, last_login FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(User::from))
    }

    async fn create(&self, user: &User) -> Result<(), AppError> {
        let topics_bitmask = topics_to_bitmask(&user.topics);
        let blocked_users_json: serde_json::Value = user.blocked_users
            .iter()
            .map(|id| serde_json::Value::String(id.to_string()))
            .collect();
        sqlx::query(
            "INSERT INTO users (id, phone, username, address, topics, blocked_users, created, last_login) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(user.id)
        .bind(&user.phone)
        .bind(&user.username)
        .bind(&user.address)
        .bind(topics_bitmask)
        .bind(&blocked_users_json)
        .bind(user.created)
        .bind(user.last_login)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn update_last_login(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET last_login = NOW() WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await?;

        Ok(())
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

pub struct PostgresValidationCodeRepository {
    pool: PgPool,
}

impl PostgresValidationCodeRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl ValidationCodeRepository for PostgresValidationCodeRepository {
    async fn create(&self, code: &ValidationCode) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO validation_codes (user_id, code, expires_at) VALUES ($1, $2, $3)"
        )
        .bind(code.user_id)
        .bind(&code.code)
        .bind(code.expires_at)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<ValidationCode>, AppError> {
        let row: Option<ValidationCodeRow> = sqlx::query_as(
            "SELECT user_id, code, expires_at FROM validation_codes WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(ValidationCode::from))
    }

    async fn delete_by_user_id(&self, user_id: Uuid) -> Result<(), AppError> {
        sqlx::query("DELETE FROM validation_codes WHERE user_id = $1")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
