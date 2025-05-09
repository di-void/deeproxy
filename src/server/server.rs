use std::convert::Infallible;
use std::net::SocketAddr;

use crate::cache::{Cache, CachedResponse};
use bytes::Bytes;
use http_body_util::Full;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{self, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

pub struct Server {
    port: u16,
    origin: String,
}

impl Server {
    pub fn new(port: u16, origin: String) -> Self {
        Self { port, origin }
    }

    pub async fn start(&self, cache: Cache) -> Result<(), Box<dyn std::error::Error>> {
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
        let cache_lock = Arc::new(RwLock::new(cache));

        loop {
            let (stream, _) = listener.accept().await?;
            let io = TokioIo::new(stream);
            let origin = url.clone();
            let cache_lock = Arc::clone(&cache_lock);

            tokio::task::spawn(async move {
                if let Err(err) = http1::Builder::new()
                    .serve_connection(
                        io,
                        service_fn(|req| proxy(req, origin.clone(), Arc::clone(&cache_lock))),
                    )
                    .await
                {
                    eprintln!("Error serving connection: {:?}", err);
                }
            });
        }
    }
}

type CacheLock = Arc<RwLock<Cache>>;

async fn proxy(
    req: Request<hyper::body::Incoming>,
    origin: hyper::Uri,
    cache_lock: CacheLock,
) -> Result<Response<Full<Bytes>>, Infallible> {
    use super::client;

    let uri = &req.uri().to_string();
    let trimmed = uri.strip_prefix("/").unwrap_or(uri);
    let req_path = format!("{}{}", origin, trimmed);

    println!("\nPATH: {}", req_path);
    // err?
    let cache = cache_lock.read().await;

    match cache.get(&req_path).await {
        Ok(Some(res)) => {
            // convert cached response to http response type
            let mut builder = Response::builder();
            for (key, val) in res.headers.iter() {
                builder = builder.header(key, val);
            }

            let response = builder
                .status(res.status)
                .body(Full::new(Bytes::from(res.body)))
                .unwrap();

            Ok(response)
        }
        Ok(None) => {
            let _res = client::fetch(req_path.parse::<hyper::Uri>().unwrap())
                .await
                .unwrap();

            // Ok(res)
            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::new()))
                .unwrap();

            Ok(response)
            // serialize response
            // cache response by request uri
            // returned (cached?) response
        }
        Err(e) => {
            eprintln!("an error occurred");

            let response = Response::builder()
                .status(StatusCode::NOT_FOUND)
                .body(Full::new(Bytes::new()))
                .unwrap();

            Ok(response)
        }
    }
}

// https://docs.rs/http/latest/http/request/struct.Request.html
// https://docs.rs/http/latest/http/response/struct.Response.html
// https://docs.rs/hyper/latest/hyper/index.html
// https://github.com/hyperium/hyper/tree/master/examples
// https://docs.rs/http-body-util/0.1.3/http_body_util/
// https://docs.rs/hyper/latest/hyper/body/index.html
