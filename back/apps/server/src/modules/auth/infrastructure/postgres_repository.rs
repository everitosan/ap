use async_trait::async_trait;
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::modules::auth::domain::{User, UserRepository, ValidationCode, ValidationCodeRepository};
use crate::shared::AppError;

// Database models

#[derive(FromRow)]
struct UserRow {
    id: uuid::Uuid,
    phone: String,
    username: Option<String>,
    address: Option<serde_json::Value>,
    topics: Option<serde_json::Value>,
    created: chrono::DateTime<chrono::Utc>,
    last_login: Option<chrono::DateTime<chrono::Utc>>,
}

impl From<UserRow> for User {
    fn from(row: UserRow) -> Self {
        User {
            id: row.id,
            phone: row.phone,
            username: row.username,
            address: row.address,
            topics: row.topics,
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
            "SELECT id, phone, username, address, topics, created, last_login FROM users WHERE phone = $1"
        )
        .bind(phone)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(User::from))
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let row: Option<UserRow> = sqlx::query_as(
            "SELECT id, phone, username, address, topics, created, last_login FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;

        Ok(row.map(User::from))
    }

    async fn create(&self, user: &User) -> Result<(), AppError> {
        sqlx::query(
            "INSERT INTO users (id, phone, username, address, topics, created, last_login) VALUES ($1, $2, $3, $4, $5, $6, $7)"
        )
        .bind(user.id)
        .bind(&user.phone)
        .bind(&user.username)
        .bind(&user.address)
        .bind(&user.topics)
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

    async fn update_profile(&self, id: Uuid, username: &str, topics: &serde_json::Value) -> Result<(), AppError> {
        sqlx::query("UPDATE users SET username = $2, topics = $3 WHERE id = $1")
            .bind(id)
            .bind(username)
            .bind(topics)
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
