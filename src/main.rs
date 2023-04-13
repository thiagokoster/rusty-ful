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
            println!("Listing all workspaces");
            print_config(config, name);
        } else {
            println!("No command specified for workspace");
        }
    } else {
        println!("No workspace specified");
    }
}

fn init_config() -> Config {
    match config_parser::read_config("config.toml") {
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

fn print_config(config: Config, workspace: Option<&String>) {
    if workspace.is_some() {
        let name = workspace.unwrap();
        if let Some(workspace) = &config.workspace.get(name) {
            print_workspace(&name, workspace);
        } else {
            println!("Non existing workspace");
        }
        return;
    }

    for (key, value) in config.workspace.into_iter() {
        print_workspace(&key, &value)
    }
}

fn print_workspace(name: &str, workspace: &Workspace) {
    println!(
        "{} - {}",
        name,
        workspace.description.as_ref().unwrap_or(&String::default())
    );
    if let Some(requests) = &workspace.requests {
        for (i, request) in requests.iter().enumerate() {
            println!("  {} - {}", i, request.name)
        }
        println!()
    }
}
