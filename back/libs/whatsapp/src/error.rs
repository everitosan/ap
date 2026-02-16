use serde_json::Value;

#[derive(Debug, thiserror::Error)]
pub enum WhatsappError {
    #[error("client error ({status}): {message}")]
    ClientError {
        status: u16,
        message: String,
        details: Value,
    },

    #[error("server error: {0}")]
    ServerError(#[from] reqwest::Error),

    #[error("config error: {0}")]
    ConfigError(String),
}
