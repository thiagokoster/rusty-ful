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

pub async fn make_request(method: &Method, url: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();

    let request_builder = match method {
        Method::GET => client.get(url),
        Method::POST => client.post(url),
    };
    Ok(request_builder.send().await?)
}
