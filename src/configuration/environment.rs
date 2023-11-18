use super::error::ConfigurationError;
use serde::Deserialize;
use std::fmt::Display;

static DEV: &str = "dev";
static TEST: &str = "test";
static PROD: &str = "prod";

#[derive(Debug, Deserialize, PartialEq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Environment {
    Dev,
    Test,
    Prod,
}

impl AsRef<str> for Environment {
    fn as_ref(&self) -> &str {
        match self {
            Environment::Dev => DEV,
            Environment::Test => TEST,
            Environment::Prod => PROD,
        }
    }
}

impl Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Environment::Dev => write!(f, "{DEV}"),
            Environment::Test => write!(f, "{TEST}"),
            Environment::Prod => write!(f, "{PROD}"),
        }
    }
}

impl TryFrom<String> for Environment {
    type Error = ConfigurationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.as_str() {
            "dev" => Ok(Environment::Dev),
            "test" => Ok(Environment::Test),
            "prod" => Ok(Environment::Prod),
            _ => Err(ConfigurationError::ParseEnvFailed(value)),
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::Prod
    }
}
