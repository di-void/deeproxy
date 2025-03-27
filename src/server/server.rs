// https://docs.rs/http/latest/http/request/struct.Request.html
// https://github.com/hyperium/hyper/tree/master/examples

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

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let in_addr = SocketAddr::from(([127, 0, 0, 1], self.port));

        let listener = TcpListener::bind(in_addr).await?;

        println!("Listening on http://{}", in_addr);
        println!("Proxying on {}", self.origin);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let origin = self.origin.clone();

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(io, service_fn(|req| proxy(req, origin.clone())))
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

async fn proxy(
    req: Request<hyper::body::Incoming>,
    origin: String,
) -> Result<Response<String>, Infallible> {
    let req_path = format!("{}{}", origin, req.uri());

    println!("PATH: {}", req_path);
    // forward request to new address
    // cache response by request uri
    // returned (cached?) response
    Ok(Response::new(String::from("Hello proxy")))
}

// hello
async fn _hello_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("URI: {}", req.uri());

    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
