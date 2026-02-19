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
    Es,
    EsEs,
    EnUs,
    En,
    EnGb,
    PtBr,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::EsMx => "es_MX",
            Language::Es => "es",
            Language::EsEs => "es_ES",
            Language::EnUs => "en_US",
            Language::En => "en",
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

    println!("{}",&body);

    client.post("messages", &body).await
}
