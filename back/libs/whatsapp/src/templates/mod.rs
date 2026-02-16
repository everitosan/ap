pub mod builder;
pub mod parameters;

use serde_json::{Value, json};

use crate::client::Client;
use crate::error::WhatsappError;

use self::builder::build_components;
use self::parameters::Parameters;

#[derive(Debug, Clone, Copy, Default)]
pub enum Language {
    #[default]
    EsMx,
    EsEs,
    EnUs,
    EnGb,
    PtBr,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::EsMx => "es_MX",
            Language::EsEs => "es_ES",
            Language::EnUs => "en_US",
            Language::EnGb => "en_GB",
            Language::PtBr => "pt_BR",
        }
    }
}

pub async fn send(
    client: &Client,
    to: &str,
    template_name: &str,
    params: &Parameters,
    language: Language,
) -> Result<Value, WhatsappError> {
    let components = build_components(params);

    let body = json!({
        "messaging_product": "whatsapp",
        "recipient_type": "individual",
        "to": to,
        "type": "template",
        "template": {
            "name": template_name,
            "language": { "code": language.code() },
            "components": components,
        }
    });

    client.post("messages", &body).await
}
