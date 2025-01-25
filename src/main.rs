use dcache::cli::{CLI, Command};

// #[tokio::main]
// async fn main() {
fn main() {
    // get caching layer
    // let cache = Cache::new();
    // get logger
    // let logger = Logger::new();
    // init cli
    let cli = CLI::init();

    match cli.command() {
        // clear cache command
        Command::ClearCache => {
            // cache.clear();
            println!("Clearing the cache..");
        }
        // start server
        Command::StartServer(port, origin) => {
            // init server
            println!("PORT: {}, ORIGIN: {}", port, origin);
            // let server = Server::new(port, origin);
            // // start the server
            // server.start(cache, logger);
        }
    }
}