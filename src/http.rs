use reqwest;
use reqwest::{Error, Response};

pub async fn make_request(method: &str, url: &str) -> Result<Response, Error> {
    let client = reqwest::Client::new();

    let request_builder = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        _ => panic!("Unsuported method"),
    };
    Ok(request_builder.send().await?)
}
