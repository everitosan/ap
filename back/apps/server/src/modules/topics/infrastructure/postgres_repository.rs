use async_trait::async_trait;
use sqlx::{FromRow, PgPool};

use crate::modules::topics::domain::TopicRepository;
use crate::shared::AppError;

/// Database model for topics_catalogue
#[derive(FromRow)]
struct TopicRow {
    id: i32,
    name: String,
}

/// PostgreSQL implementation of TopicRepository
pub struct PostgresTopicRepository {
    pool: PgPool,
}

impl PostgresTopicRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TopicRepository for PostgresTopicRepository {
    async fn get_all(&self) -> Result<Vec<crate::modules::topics::domain::Topic>, AppError> {
        let rows: Vec<TopicRow> = sqlx::query_as(
            r#"SELECT id, name FROM topics_catalogue WHERE active = true ORDER BY name"#
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|r| crate::modules::topics::domain::Topic { id: r.id, name: r.name })
            .collect())
    }
}
