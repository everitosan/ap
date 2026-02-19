use std::sync::Arc;

use sqlx::PgPool;

use crate::modules::auth::infrastructure::PhoneNotifier;
use super::CookieEncryptor;

/// Estado compartido de la aplicación
/// Contiene las dependencias inyectadas para todos los handlers
#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
    pub phone_notifier: Arc<dyn PhoneNotifier>,
    pub cookie_encryptor: CookieEncryptor,
}

impl AppState {
    pub fn new(
        db_pool: PgPool,
        phone_notifier: Arc<dyn PhoneNotifier>,
        cookie_encryptor: CookieEncryptor,
    ) -> Self {
        Self {
            db_pool,
            phone_notifier,
            cookie_encryptor,
        }
    }
}
