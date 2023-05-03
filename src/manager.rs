use reqwest::Response;

use crate::config_parser::Config;
use crate::http;

pub struct CLIManager<'a> {
    pub config: &'a mut Config,
}

impl<'a> CLIManager<'a> {
    pub fn list_workspaces(&self, all: bool) {
        for workspace in self.config.workspaces.iter() {
            println!(
                "{} - {}: {}",
                workspace.id,
                workspace.name,
                workspace.description.as_ref().unwrap()
            );
            if !all {
                continue;
            }
            if let Some(requests) = &workspace.requests {
                for request in requests.iter() {
                    println!(
                        "  {} - {}: {} {}",
                        request.id, request.name, request.method, request.url
                    );
                }
            }
        }
    }

    pub async fn make_request(&self, id: &str) {
        for workspace in self.config.workspaces.iter() {
            if let Some(requests) = &workspace.requests {
                if let Some(request) = requests
                    .iter()
                    .find(|&request| request.id.to_string() == id)
                {
                    println!("Making request with id: {} ...", id);
                    match http::make_request(&request.method.parse().unwrap(), &request).await {
                        Ok(result) => self.print_response(result).await,
                        Err(err) => println!("{}", err),
                    }
                } else {
                    println!("Request with id '{}' not found", id);
                }
            }
        }
    }

    async fn print_response(&self, response: Response) {
        println!("--- Response ---");
        println!("{} - {}", response.url(), response.status());
        println!("{}", response.text().await.unwrap());
    }
}
