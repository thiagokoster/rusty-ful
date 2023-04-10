use maplit::hashmap;

#[cfg(test)]
use rustyful::config_parser::{parse_config, Config, Request, Workspace};
#[test]
fn test_parse_config_file() {
    let file_content = r#"
            [workspace.one]
            description = "one"
            [[workspace.one.requests]]
            name = "Request 1"
            method = "GET"
            url = "https://example.com/api/1"

            [workspace.two]

            "#;
    let expected = Config {
        workspace: hashmap! {
        "one".to_string() => Workspace {
            description:Some("one".to_string()),
            requests: Some(vec![
                Request{
                    name: "Request 1".to_string(),
                    method: "GET".to_string(),
                    url: "https://example.com/api/1".to_string()
                }])
        },
        "two".to_string() => Workspace { description: None, requests: None }
        },
    };
    let parsed_config = parse_config(file_content).unwrap();
    assert_eq!(parsed_config, expected)
}
