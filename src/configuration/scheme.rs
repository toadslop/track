use serde::Deserialize;
use std::fmt::Display;

static HTTP: &str = "http";
static HTTPS: &str = "https";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Scheme {
    Http,
    Https,
}

impl From<Scheme> for config::ValueKind {
    fn from(value: Scheme) -> Self {
        config::ValueKind::String(value.to_string())
    }
}

impl AsRef<str> for Scheme {
    fn as_ref(&self) -> &str {
        match self {
            Scheme::Http => HTTP,
            Scheme::Https => HTTPS,
        }
    }
}

impl Default for Scheme {
    fn default() -> Self {
        Self::Https
    }
}

impl Display for Scheme {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scheme::Http => write!(f, "{HTTP}"),
            Scheme::Https => write!(f, "{HTTPS}"),
        }
    }
}
