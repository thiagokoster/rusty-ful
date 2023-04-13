use config_parser::{Config, Workspace};
use tokio;
mod cli;
mod config_parser;
mod http;

#[tokio::main]
async fn main() {
    println!("--- RUSTYFul ---");
    let config = init_config();

    let matches = cli::client().get_matches();

    if let Some(workspace_matches) = matches.subcommand_matches("workspace") {
        let name = workspace_matches.get_one::<String>("name");
        if workspace_matches.get_flag("list") {
            list_workspaces(&config);
        } else {
            println!("No command specified for workspace");
        }
    } else {
        println!("No workspace specified");
    }
}

fn init_config() -> Config {
    match config_parser::read_config("workspaces.json") {
        Ok(config) => {
            // Config parsed successfully
            config
        }
        Err(e) => {
            eprintln!("Error: {:?}", e);
            std::process::exit(1);
        }
    }
}

fn list_workspaces(config: &config_parser::Config) {
    for (i, workspace) in config.workspaces.iter().enumerate() {
        println!(
            "{} - {}: {}",
            i,
            workspace.name,
            workspace.description.as_ref().unwrap()
        );

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
