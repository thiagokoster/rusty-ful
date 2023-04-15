use crate::manager::CLIManager;
use clap::{Arg, Command};

pub fn client(manager: &CLIManager) {
    let list_cmd: Command = Command::new("list").about("List workspaces").arg(
        Arg::new("all")
            .long("all")
            .short('a')
            .required(false)
            .num_args(0),
    );

    let command = Command::new("Rustyrust")
        .version("0.1.0")
        .author("thiagokoster")
        .about("A Rust command-line HTTP client")
        .subcommand(list_cmd);
    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("list", _args)) => {
            manager.list_workspaces(_args.get_flag("all"));
        }
        _ => short_help(),
    }
}

fn short_help() {
    println!("Use -h or --help for help: rustyful -h");
}
