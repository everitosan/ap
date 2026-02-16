use async_trait::async_trait;

use crate::shared::AppError;

/// Trait for sending SMS notifications
#[async_trait]
pub trait PhoneNotifier: Send + Sync {
    async fn send(&self, phone: &str, code: &str) -> Result<(), AppError>;
}

/// Stub implementation that just logs
pub struct StubPhoneNotifier;

#[async_trait]
impl PhoneNotifier for StubPhoneNotifier {
    async fn send(&self, phone: &str, code: &str) -> Result<(), AppError> {
        tracing::info!(phone = %phone, code = %code, "SMS notification (stub)");
        Ok(())
    }
}
