use serde_json::Value;
use tracing::error;

use crate::config::Config;
use crate::error::WhatsappError;

#[derive(Debug, Clone)]
pub struct Client {
    http: reqwest::Client,
    config: Config,
}

impl Client {
    pub fn new(config: Config) -> Self {
        Self {
            http: reqwest::Client::new(),
            config,
        }
    }

    pub fn config(&self) -> &Config {
        &self.config
    }

    pub async fn post(&self, path: &str, body: &Value) -> Result<Value, WhatsappError> {
        let url = format!("{}/{}", self.config.base_url(), path);

        let response = self
            .http
            .post(&url)
            .bearer_auth(&self.config.graph_api_token)
            .json(body)
            .send()
            .await?;

        let status = response.status().as_u16();

        if (200..300).contains(&status) {
            let body: Value = response.json().await?;
            Ok(body)
        } else {
            let body: Value = response
                .json()
                .await
                .unwrap_or(Value::Null);

            let message = body
                .get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .unwrap_or("unknown error")
                .to_string();

            let details = body
                .get("error")
                .cloned()
                .unwrap_or(body.clone());

            error!(status, %message, "WhatsApp API error");

            Err(WhatsappError::ClientError {
                status,
                message,
                details,
            })
        }
    }
}
