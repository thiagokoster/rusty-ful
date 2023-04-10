use serde::Deserialize;
use std::{collections::HashMap, fs};
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

#[derive(Debug)]
pub enum ConfigParserError {
    FileNotFound,
    InvalidToml(String),
}

pub type ConfigResult<T> = Result<T, ConfigParserError>;

pub fn read_config(path: &str) -> ConfigResult<Config> {
    let file_contents = fs::read_to_string(path).map_err(|_| ConfigParserError::FileNotFound)?;
    let config: Config = parse_config(&file_contents)?;

    Ok(config)
}

pub fn parse_config(config: &str) -> Result<Config, ConfigParserError> {
    let config: Config =
        toml::from_str(&config).map_err(|e| ConfigParserError::InvalidToml(e.to_string()))?;
    Ok(config)
}
