// https://docs.rs/hyper/latest/hyper/server/index.html
// https://hyper.rs/guides/1/server/hello-world/
// https://github.com/hyperium/hyper/blob/master/examples/hello.rs
// https://github.com/hyperium/hyper/blob/master/examples/service_struct_impl.rs

use std::convert::Infallible;
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use tokio::net::TcpListener;

pub struct Server {
    port: u16,
    origin: String,
}

impl Server {
    pub fn new(port: u16, origin: String) -> Self {
        Self { port, origin }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        println!("PORT: {}, ORIGIN: {}", self.port, self.origin);

        let listener = TcpListener::bind(addr).await?;

        // We start a loop to continuously accept incoming connections
        loop {
            let (stream, _) = listener.accept().await?;

            // Use an adapter to access something implementing `tokio::io` traits as if they implement
            // `hyper::rt` IO traits.
            let io = TokioIo::new(stream);

            // Spawn a tokio task to serve multiple connections concurrently
            tokio::task::spawn(async move {
                // Finally, we bind the incoming connection to our `hello` service
                if let Err(err) = http1::Builder::new()
                    // `service_fn` converts our function in a `Service`
                    .serve_connection(io, service_fn(serve))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

// hello
async fn serve(req: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
