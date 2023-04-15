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

#[cfg(test)]
#[test]
fn test_parse_config_file() {
    let file_content = r#"
    {
        "workspaces": [
        {
            "name": "one",
            "description": "first workspace",
            "requests": [
            {
                "name": "R1",
                "method": "GET",
                "url": "url1"
            }
            ]
        },
        {
            "name": "two",
            "description": null
        }
        ]
    }
            "#;
    let expected = Config {
        workspaces: vec![
            Workspace {
                name: String::from("one"),
                description: Some(String::from("first workspace")),
                requests: Some(vec![Request {
                    name: String::from("R1"),
                    method: String::from("GET"),
                    url: String::from("url1"),
                }]),
            },
            Workspace {
                name: String::from("two"),
                description: None,
                requests: None,
            },
        ],
    };
    let parsed_config = parse_config(file_content).unwrap();
    assert_eq!(parsed_config, expected)
}
