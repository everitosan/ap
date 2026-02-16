use async_trait::async_trait;
use whatsapp::{
    templates::{self, builder::ComponentParam, parameters::Parameters, Language},
    Client as WhatsAppClient, Config as WhatsAppConfig, WhatsappError,
};

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

/// WhatsApp implementation using the whatsapp library
pub struct WhatsAppPhoneNotifier {
    client: WhatsAppClient,
    template_name: String,
    language: Language,
}

impl WhatsAppPhoneNotifier {
    pub fn new(config: WhatsAppConfig, template_name: impl Into<String>) -> Self {
        Self {
            client: WhatsAppClient::new(config),
            template_name: template_name.into(),
            language: Language::default(),
        }
    }

    pub fn with_language(mut self, language: Language) -> Self {
        self.language = language;
        self
    }
}

#[async_trait]
impl PhoneNotifier for WhatsAppPhoneNotifier {
    async fn send(&self, phone: &str, code: &str) -> Result<(), AppError> {
        let params = Parameters::new().with_body(vec![ComponentParam::Text(code.to_string())]);

        let fixed_phone = format!("+52{}", phone);

        templates::send(&self.client, &fixed_phone, &self.template_name, &params, self.language)
            .await
            .map_err(|e| AppError::Internal(e.to_string()))?;

        tracing::info!(phone = %phone, "WhatsApp verification code sent");
        Ok(())
    }
}

impl From<WhatsappError> for AppError {
    fn from(err: WhatsappError) -> Self {
        AppError::Internal(err.to_string())
    }
}
