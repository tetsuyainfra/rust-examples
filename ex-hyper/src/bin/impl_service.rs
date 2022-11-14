#[allow(unused)]
use std::{boxed::Box, convert::Infallible, future::Future, pin::Pin};

use bytes::Bytes;
use http_body_util::Full;
use hyper::{
    body::{self, Body},
    service::Service,
    Request, Response,
};

#[tokio::main]
async fn main() {}

struct Base {}
impl Service<Request<body::Incoming>> for Base {
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<body::Incoming>) -> Self::Future {
        Box::pin(async move {
            println!("req: {:?}", req);
            Ok(Response::new(Full::new(Bytes::from("abc"))))
        })
    }
}

struct Sub {
    base: Base,
}
impl Sub {
    fn new(base: Base) -> Self {
        Self { base }
    }
}
impl Service<Request<body::Incoming>> for Sub {
    type Response = Response<Full<Bytes>>;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request<body::Incoming>) -> Self::Future {
        self.base.call(req)
        // Box::pin(async move {
        //     println!("req: {:?}", req);
        //     Ok(Response::new(Full::new(Bytes::from("abc"))))
        // })
    }
}
