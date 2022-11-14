#![allow(unused)]
use std::{
    boxed::Box, convert::Infallible, error::Error as StdError, future::Future, net::SocketAddr,
    pin::Pin,
};
//
use bytes::Bytes;
use http_body_util::Full;
use hyper::{
    body::Incoming,
    service::{service_fn, Service},
    Request, Response,
};
use tokio::net::TcpListener;
//
// usage:
//  serve(|req| async {  YourService::new().call(req) })
//

pub async fn serve<SVC, ERR>(addr: SocketAddr, mut svc: SVC) -> anyhow::Result<()>
where
    SVC: Service<
            Request<hyper::body::Incoming>,
            // Error = Infallible,
            Error = ERR,
            Response = Response<Full<Bytes>>,
        > + Copy,
    ERR: Into<Box<dyn StdError + Send + Sync>>,
{
    let listener = TcpListener::bind(addr).await?;
    loop {
        let (stream, remote_addr) = listener.accept().await?;
        println!("from: {}", &remote_addr);

        let b = hyper::server::conn::http1::Builder::new();
        // let make_svc = service_fn(|req| async move { svc.call(req).await });
        let conn = b.serve_connection(stream, svc);
        conn.await;
    }
}
