use clap::{Arg, Command};

pub fn client() -> Command {
    let workspace_cmd: Command = Command::new("workspace").about("Manage workspaces").arg(
        Arg::new("list")
            .long("list")
            .long("list")
            .short('l')
            .required(false)
            .num_args(0)
            .help("List all workspaces"),
    );

    Command::new("Rustyrust")
        .version("0.1.0")
        .author("thiagokoster")
        .about("A Rust command-line HTTP client")
        .subcommand(workspace_cmd)
}

//.arg(
//            Arg::new("method")
//                .short('m')
//                .long("method")
//                .help("The HTTP method to use for the request")
//                .default_value("get"),
//        )
//        .arg(
//            Arg::new("url")
//                .help("The URL to make a request to")
//                .required(true)
//                .index(1),
//        )
