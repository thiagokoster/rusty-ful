use tokio;

mod cli;
mod http;

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let matches = cli::client().get_matches();

    let method = matches.get_one::<String>("method").unwrap();
    let url = matches.get_one::<String>("url").unwrap();

    match http::make_request(method, url).await {
        Ok(response) => println!("{}", response),
        Err(error) => println!("{:?}", error),
    }
}
