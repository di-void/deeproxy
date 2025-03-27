use http_body_util::Empty;
use hyper::{self, body::Bytes, Request, Response, Uri};
use hyper_util::rt::TokioIo;
use tokio::net::TcpStream;

// https://github.com/hyperium/hyper/blob/master/examples/client.rs

pub async fn fetch(
    url: Uri,
) -> Result<Response<hyper::body::Incoming>, Box<dyn std::error::Error>> {
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let addr = format!("{}:{}", host, port);
    let stream = TcpStream::connect(addr).await?;
    let io = TokioIo::new(stream);

    let (mut sender, conn) = hyper::client::conn::http1::handshake(io).await?;
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Connection failed: {:?}", err);
        }
    });

    let authority = url.authority().unwrap().clone();

    let path = url.path();
    let req = Request::builder()
        .uri(path)
        .header(hyper::header::HOST, authority.as_str())
        .body(Empty::<Bytes>::new())?;

    let res = sender.send_request(req).await?;

    println!("Response: {}", res.status());
    println!("Headers: {:#?}\n", res.headers());

    Ok(res)
}
