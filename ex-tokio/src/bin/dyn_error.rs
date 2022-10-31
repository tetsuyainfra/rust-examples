#![allow(unused)]
use std::error::Error as StdError;

use bytes::Bytes;
use http_body_util::Full;
use hyper::{body::Body, Request, Response};
use hyper::{body::Incoming, service::service_fn};

#[tokio::main]
async fn main() {
    let handle = tokio::spawn(async {
        //
        // let x = test_handler_send().await;
        // let x = test_handler_sync().await;
        let x = test_handler_static().await;

        let s = service_fn(|r| {
            //
            async {
                //
                test_handler_service(r).await
            }
        });

        let stream = tokio_test::io::Builder::new().build();
        let b = hyper::server::conn::http1::Builder::new();
        b.serve_connection(stream, s).await;
    });

    let _r = handle.await;
}

async fn test_handler_send() -> Result<i32, Box<dyn StdError + Send>> {
    Ok(1)
}

async fn test_handler_sync() -> Result<i32, Box<dyn StdError + Send + Sync>> {
    Ok(1)
}

async fn test_handler_static() -> Result<i32, Box<dyn StdError + Send + Sync + 'static>> {
    Ok(1)
}

async fn test_handler_service(
    req: Request<Incoming>,
) -> Result<Response<Full<Bytes>>, hyper::Error> {
    todo!();
}
