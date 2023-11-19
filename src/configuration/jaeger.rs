use super::scheme::Scheme;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct JaegerSettings {
    pub port: u16,
    pub host: String,
    pub scheme: Scheme,
}

impl JaegerSettings {
    pub fn connection_string(&self) -> String {
        format!("{}://{}:{}", self.scheme, self.host, self.port)
    }
}

impl Default for JaegerSettings {
    fn default() -> Self {
        Self {
            port: 4317,
            host: "localhost".into(),
            scheme: Scheme::Https,
        }
    }
}
