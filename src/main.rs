use deeproxy::{
    cache::Cache,
    cli::{self, CLI},
    server::Server,
};

#[tokio::main]
async fn main() {
    let cache = Cache::new();
    // get logger
    // let logger = Logger::new();
    let cli = CLI::init();

    match cli.command() {
        cli::Command::ClearCache => {
            println!("Clearing the cache..");
            cache.clear().expect("Error clearning cache!");
        }
        cli::Command::StartServer(port, origin) => {
            // init server
            let server = Server::new(port, origin);
            // // start the server
            // server.start(cache, logger);
            let _ = server.start(cache).await;
        }
    }
}
