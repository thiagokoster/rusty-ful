use crate::manager::CLIManager;
use clap::{Arg, Command};

pub async fn client(manager: &CLIManager<'_>) {
    let list_cmd: Command = Command::new("list").about("List workspaces").arg(
        Arg::new("all")
            .long("all")
            .short('a')
            .required(false)
            .num_args(0),
    );

    let request_cmd: Command = Command::new("make")
        .about("Make a request")
        .arg(Arg::new("request_id").required(true));

    let command = Command::new("Rustyrust")
        .version("0.1.0")
        .author("thiagokoster")
        .about("A Rust command-line HTTP client")
        .subcommand(list_cmd)
        .subcommand(request_cmd);

    let matches = command.get_matches();
    match matches.subcommand() {
        Some(("list", args)) => {
            manager.list_workspaces(args.get_flag("all"));
        }
        Some(("make", args)) => {
            manager
                .make_request(args.get_one::<String>("request_id").unwrap())
                .await
        }
        _ => short_help(),
    }
}

fn short_help() {
    println!("Use -h or --help for help: rustyful -h");
}
