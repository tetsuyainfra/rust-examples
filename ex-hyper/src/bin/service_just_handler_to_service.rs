//
//
use std::{
    boxed::Box, convert::Infallible, error::Error as StdError, fmt::Debug, future::Future,
    net::SocketAddr, pin::Pin, time::Duration,
};
//
use bytes::Bytes;
use http_body_util::{combinators::BoxBody, Full};
use hyper::{
    body::Incoming,
    service::{service_fn, Service},
    Request, Response,
};

#[tokio::main]
async fn main() {
    use ex_hyper::server::serve;
    let addr = SocketAddr::from(([127, 0, 0, 1], 7145));

    let svc = service_fn(|req| async move {
        println!("{:?}", &req);
        //
        let resp = Response::builder().body(Full::new(Bytes::from("aaa")));
        resp
    });

    serve(addr, svc).await;
}

type HttpRequest = Request<Incoming>;
type HttpResponse = Response<Full<Bytes>>;
