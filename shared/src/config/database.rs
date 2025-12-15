use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: Option<String>,
    pub database_name: String,
    pub min_connections: u32,
    pub max_connections: u32,
    pub connect_timeout_ms: u64,
    pub idle_timeout_ms: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 5432,
            username: "postgres".to_string(),
            password: Some("postgres".to_string()),
            database_name: "postgres".to_string(),
            min_connections: 2,
            max_connections: 10,
            connect_timeout_ms: 30000,
            idle_timeout_ms: 60000,
        }
    }
}

impl DatabaseConfig {
    pub fn from_env() -> Self {
        let mut config = Self::default();

        if let Ok(host) = env::var("DB_HOST") {
            config.host = host;
        }
        if let Ok(port) = env::var("DB_PORT") {
            if let Ok(p) = port.parse() {
                config.port = p;
            }
        }
        if let Ok(username) = env::var("DB_USER") {
            config.username = username;
        }
        if let Ok(password) = env::var("DB_PASSWORD") {
            config.password = Some(password);
        }
        if let Ok(dbname) = env::var("DB_NAME") {
            config.database_name = dbname;
        }

        config
    }

    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password.as_deref().unwrap_or(""),
            self.host,
            self.port,
            self.database_name
        )
    }
}
