#[cfg(test)]
use rustyful::config_parser::{parse_config, Config, Request, Workspace};
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
