use super::{environment::Environment, scheme::Scheme};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub scheme: Scheme,
    pub environment: Environment,
    pub domain: String,
}

impl Default for ApplicationSettings {
    fn default() -> Self {
        Self {
            port: 80,
            host: "127.0.0.1".into(),
            scheme: Default::default(),
            environment: Default::default(),
            domain: Default::default(),
        }
    }
}
