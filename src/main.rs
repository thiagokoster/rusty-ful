mod config_parser;
use tokio;
mod cli;
mod manager;

#[tokio::main]
async fn main() {
    println!("--- RUSTYFul ---");
    let mut config = init_config();

    let manager = manager::CLIManager {
        config: &mut config,
    };
    cli::client(&manager);
}

pub fn init_config() -> config_parser::Config {
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
