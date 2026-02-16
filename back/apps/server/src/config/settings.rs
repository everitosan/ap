use std::env;

#[derive(Debug, Clone)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub server: ServerSettings,
}

#[derive(Debug, Clone)]
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
}

#[derive(Debug, Clone)]
pub struct ServerSettings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn from_env() -> Result<Self, env::VarError> {
        Ok(Self {
            database: DatabaseSettings {
                url: env::var("DATABASE_URL").unwrap_or_else(|_| {
                    format!(
                        "postgres://{}:{}@{}:{}/{}",
                        env::var("POSTGRES_USER").unwrap_or_else(|_| "postgres".into()),
                        env::var("POSTGRES_PASSWORD").unwrap_or_else(|_| "postgres".into()),
                        env::var("POSTGRES_HOST").unwrap_or_else(|_| "localhost".into()),
                        env::var("POSTGRES_PORT").unwrap_or_else(|_| "5432".into()),
                        env::var("POSTGRES_DB").unwrap_or_else(|_| "postgres".into()),
                    )
                }),
                max_connections: env::var("DATABASE_MAX_CONNECTIONS")
                    .unwrap_or_else(|_| "5".into())
                    .parse()
                    .unwrap_or(5),
            },
            server: ServerSettings {
                host: env::var("SERVER_HOST").unwrap_or_else(|_| "127.0.0.1".into()),
                port: env::var("SERVER_PORT")
                    .unwrap_or_else(|_| "8080".into())
                    .parse()
                    .unwrap_or(8080),
            },
        })
    }
}
