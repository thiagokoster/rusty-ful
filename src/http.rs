use crate::config_parser::Request;
use reqwest;
use reqwest::{Error, Response};

pub enum Method {
    GET,
    POST,
}

impl std::str::FromStr for Method {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "GET" => Ok(Method::GET),
            "POST" => Ok(Method::POST),
            _ => Err(format!("'{}' is not a valid value for Method", s)),
        }
    }
}

pub async fn make_request(method: &Method, request: &Request) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    let request_builder = match method {
        Method::GET => client.get(&request.url),
        Method::POST => {
            let mut req = client.post(&request.url);
            if let Some(request_body) = &request.body {
                let request_body = serde_json::to_string(&request_body).unwrap();
                println!("Request Body: {}", request_body);
                req = req.body(request_body);
            }
            req
        }
    };
    Ok(request_builder.send().await?)
}
