use crate::config_parser::{Config, Request};

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

    pub fn make_request(&self, id: &str) {
        println!("Making request with id: {} ...", id);

        for workspace in self.config.workspaces.iter() {
            if let Some(requests) = &workspace.requests {
                if let Some(request) = requests
                    .iter()
                    .find(|&request| request.id.to_string() == id)
                {
                    println!("{} {}", request.method, request.url);
                }
            }
        }
    }
}
