use reqwest::Error;
use reqwest::{self, Response};

pub async fn make_request(method: &str, url: &str) -> Result<String, Error> {
    let client = reqwest::Client::new();

    let request_builder = match method {
        "get" => client.get(url),
        "post" => client.post(url),
        _ => panic!("Unsuported method"),
    };
    Ok(request_builder.send().await?.text().await?)
}
