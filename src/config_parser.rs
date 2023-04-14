use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Request {
    pub name: String,
    pub method: String,
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Workspace {
    pub name: String,
    pub description: Option<String>,
    pub requests: Option<Vec<Request>>,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct Config {
    pub workspaces: Vec<Workspace>,
}

#[derive(Debug)]
pub enum ConfigParserError {
    FileNotFound,
    InvalidConfig(String),
}

pub type ConfigResult<T> = Result<T, ConfigParserError>;

pub fn read_config(path: &str) -> ConfigResult<Config> {
    let file_contents = fs::read_to_string(path).map_err(|_| ConfigParserError::FileNotFound)?;
    let config: Config = parse_config(&file_contents)?;

    Ok(config)
}

pub fn parse_config(config: &str) -> Result<Config, ConfigParserError> {
    let config: Config = serde_json::from_str(&config)
        .map_err(|e| ConfigParserError::InvalidConfig(e.to_string()))?;
    Ok(config)
}
