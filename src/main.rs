use dcache::cli::{self, CLI};
use dcache::server::Server;

#[tokio::main]
async fn main() {
    // get caching layer
    // let cache = Cache::new();
    // get logger
    // let logger = Logger::new();
    // init cli
    let cli = CLI::init();

    match cli.command() {
        // clear cache command
        cli::Command::ClearCache => {
            // cache.clear();
            println!("Clearing the cache..");
        }
        // start server
        cli::Command::StartServer(port, origin) => {
            // init server
            let server = Server::new(port, origin);
            // // start the server
            // server.start(cache, logger);
            let _ = server.start().await;
        }
    }
}
