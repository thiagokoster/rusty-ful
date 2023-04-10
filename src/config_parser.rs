use serde::Deserialize;
use std::collections::HashMap;
use toml::de::Error;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Request {
    pub name: String,
    pub method: String,
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Workspace {
    pub description: Option<String>,
    pub requests: Option<Vec<Request>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub workspace: HashMap<String, Workspace>,
}

pub fn parse_config(config: &str) -> Result<Config, Error> {
    let config: Config = toml::from_str(&config)?;
    Ok(config)
}
