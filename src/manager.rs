use crate::config_parser::Config;

pub struct CLIManager<'a> {
    pub config: &'a mut Config,
}

impl<'a> CLIManager<'a> {
    pub fn list_workspaces(&self, all: bool) {
        for (i, workspace) in self.config.workspaces.iter().enumerate() {
            println!(
                "{} - {}: {}",
                i,
                workspace.name,
                workspace.description.as_ref().unwrap()
            );
            if !all {
                continue;
            }
            if let Some(requests) = &workspace.requests {
                for (i, request) in requests.iter().enumerate() {
                    println!(
                        "  {} - {}: {} {}",
                        i, request.name, request.method, request.url
                    );
                }
            }
        }
    }
}
