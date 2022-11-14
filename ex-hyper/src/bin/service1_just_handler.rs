//
//
use std::{
    boxed::Box, convert::Infallible, future::Future, net::SocketAddr, pin::Pin, time::Duration,
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

    let svc = service_fn(|req| async {
        //
        println!("req: {:?}", &req);
        (RequestHandler {}).call(req).await
    });

    serve(addr, svc).await;
}

type HttpRequest = Request<Incoming>;
type HttpResponse = Response<Full<Bytes>>;
trait Handler {
    type Response;
    type Error;
    type Future: Future<Output = Result<Self::Response, Self::Error>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future;
}

#[derive(Clone, Copy)]
struct RequestHandler;
impl Handler for RequestHandler {
    type Response = HttpResponse;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        println!("request: {:?}", &request);
        let resp = Response::new(Full::new(Bytes::from("hello from handler")));
        Box::pin(async move { Ok::<_, Infallible>(resp) })
    }
}

/*
#[derive(Clone)]
struct TimeWith<T> {
    inner_handler: T,
    duration: Duration,
}

impl<T> Handler for Timeout<T>
where
    T: Handler + Clone,
{
    type Future = Pin<Box<dyn Future<Output = Result<HttpResponse, Error>>>>;

    fn call(&mut self, request: HttpRequest) -> Self::Future {
        // Get an owned clone of `&mut self`
        let mut this = self.clone();

        Box::pin(async move {
            let result =
                tokio::time::timeout(this.duration, this.inner_handler.call(request)).await;

            match result {
                Ok(Ok(response)) => Ok(response),
                Ok(Err(error)) => Err(error),
                Err(_timeout) => Err(Error::timeout()),
            }
        })
    }
}
*/
