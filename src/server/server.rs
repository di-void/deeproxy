// https://docs.rs/http/latest/http/request/struct.Request.html
// https://github.com/hyperium/hyper/tree/master/examples

use std::convert::Infallible;
use std::marker::{Send, Sync};
use std::net::SocketAddr;

use http_body_util::Full;
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{self, Request, Response};
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
        let url = self.origin.parse::<hyper::Uri>().unwrap();

        // only support http (for now)
        if let Some("https") = url.scheme_str() {
            println!("Only http origins are supported");
            return Ok(());
        }

        let in_addr = SocketAddr::from(([127, 0, 0, 1], self.port));
        let listener = TcpListener::bind(in_addr).await?;

        println!("Listening on http://{}", in_addr);
        println!("Proxying on {}", &url);

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let origin = url.clone();

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
    origin: hyper::Uri,
) -> Result<Response<hyper::body::Incoming>, Infallible> {
    use super::client;

    let uri = &req.uri().to_string();
    let trimmed = uri.strip_prefix("/").unwrap_or(uri);
    let req_path = format!("{}{}", origin, trimmed);

    println!("\nPATH: {}", req_path);
    // forward request to new address
    let res = client::fetch(req_path.parse::<hyper::Uri>().unwrap())
        .await
        .unwrap();

    // cache response by request uri
    // returned (cached?) response
    Ok(res)
}

// hello
async fn _hello_service(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<Full<Bytes>>, Infallible> {
    println!("URI: {}", req.uri());

    Ok(Response::new(Full::new(Bytes::from("Hello, World!"))))
}
