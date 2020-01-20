use config::{Config, Environment};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub port: Option<u16>,
    pub database_url: String,
}

impl Settings {
    pub fn new() -> Settings {
        let mut s = Config::new();
        s.merge(Environment::default()).unwrap();

        s.try_into().unwrap()
    }
}
