use secrecy::{ExposeSecret, Secret};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseSettings {
    pub port: u16,
    pub host: String,
    pub password: Secret<String>,
    pub user: String,
    pub name: String,
    pub init_wait_interval: u64,
    pub init_wait_retry_count: u8,
}

impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.password.expose_secret(),
            self.host,
            self.port,
            self.name
        )
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        Self {
            port: 5433,
            host: "localhost".into(),
            password: Secret::new("password".into()),
            user: "user".into(),
            name: "track".into(),
            init_wait_interval: 1000u64,
            init_wait_retry_count: 5,
        }
    }
}
