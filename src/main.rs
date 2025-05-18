use deeproxy::{
    cache::Cache,
    cli::{self, CLI},
    server::Server,
};

#[tokio::main]
async fn main() {
    let cache = Cache::new().await;
    let cli = CLI::init();

    match cli.command() {
        cli::Command::ClearCache => {
            println!("Clearing the cache..");
            cache.clear().await.expect("Error clearning cache!");
            println!("ALL CLEAR!");
        }
        cli::Command::StartServer(port, origin) => {
            let server = Server::new(port, origin);
            let _ = server.start(cache).await;
        }
    }
}
