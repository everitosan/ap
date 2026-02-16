use sqlx::PgPool;

/// Estado compartido de la aplicación
/// Contiene las dependencias inyectadas para todos los handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

impl AppState {
    pub fn new(db_pool: PgPool) -> Self {
        Self { db_pool }
    }
}
