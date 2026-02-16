use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

use super::settings::DatabaseSettings;

pub async fn create_pool(settings: &DatabaseSettings) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(settings.max_connections)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&settings.url)
        .await
}
