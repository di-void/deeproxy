// https://docs.rs/hyper/latest/hyper/ 
// https://hyper.rs/guides/1/server/hello-world/

struct Server {
    port: u16,
    origin: String,
}

impl Server {
    fn new(port: 16, origin: String) -> Self {
        Self { port, origin }
    }

    fn start() {
        todo!()
    }
}