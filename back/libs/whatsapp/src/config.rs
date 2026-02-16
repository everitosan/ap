#[derive(Debug, Clone)]
pub struct Config {
    pub webhook_token: String,
    pub phone_number_id: String,
    pub graph_api_token: String,
    pub graph_api_version: String,
    pub graph_api_url: String,
}

impl Config {
    pub fn new(
        webhook_token: impl Into<String>,
        phone_number_id: impl Into<String>,
        graph_api_token: impl Into<String>,
        graph_api_version: impl Into<String>,
        graph_api_url: impl Into<String>,
    ) -> Self {
        Self {
            webhook_token: webhook_token.into(),
            phone_number_id: phone_number_id.into(),
            graph_api_token: graph_api_token.into(),
            graph_api_version: graph_api_version.into(),
            graph_api_url: graph_api_url.into(),
        }
    }

    pub fn base_url(&self) -> String {
        format!(
            "{}/{}/{}",
            self.graph_api_url, self.graph_api_version, self.phone_number_id
        )
    }
}
