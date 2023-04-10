use config_parser::Config;
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
        if workspace_matches.get_flag("list") {
            println!("Listing all workspaces");
            print_config(config);
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

fn print_config(config: Config) {
    for (key, value) in config.workspace.into_iter() {
        println!("{} - {}", key, value.description.unwrap_or_default());
        if value.requests.is_some() {
            for (i, request) in value.requests.unwrap().iter().enumerate() {
                println!("  {} - {}", i, request.name)
            }
        }
    }
}
