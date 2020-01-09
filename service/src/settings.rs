use config::{Config, ConfigError, Environment};
use serde::Deserialize;

/// Structure to represent the settings for the service
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: Option<u16>,
    pub database_url: String,
}

impl Settings {
    /// Construct the settings from the current environment
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(Environment::default())?;

        s.try_into()
    }
}
