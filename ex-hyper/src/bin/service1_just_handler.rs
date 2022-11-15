//
//
use std::{
    boxed::Box,
    convert::Infallible,
    fmt::{self, format},
    future::Future,
    net::SocketAddr,
    pin::Pin,
    time::Duration,
};
//
use bytes::Bytes;
use chrono::{self, TimeZone};
use http_body_util::{combinators::BoxBody, BodyExt, Full};
use hyper::{
    body::{self, Incoming},
    service::{service_fn, Service},
    Request, Response,
};

#[tokio::main]
async fn main() {
    use ex_hyper::server::serve;
    let addr = SocketAddr::from(([127, 0, 0, 1], 7145));

    // let svc = service_fn(|req| async {
    //     //
    //     let mut base = Base {
    //         s: String::from("hello base"),
    //     };
    //     base.call(req).await
    // });
    // serve(addr, svc).await;

    let svc2 = service_fn(|req: Request<Incoming>| async move {
        //
        println!("req: {:?}", &req);
        let base = Base {
            s: String::from("hello base"),
        };
        let mut time_with = TimeWith {
            inner_future: base,
            from_time: chrono::Local::now(),
        };
        time_with.call(req).await
    });

    serve(addr, svc2).await;
}

type HttpRequest = Request<Incoming>;
type HttpResponse = Response<Full<Bytes>>;

#[derive(Clone)]
struct Base {
    s: String,
}
impl<Request> Service<Request> for Base
where
    Request: fmt::Debug,
{
    type Response = HttpResponse;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request) -> Self::Future {
        println!("call in Service for Base {:?}", &req);
        let this = self.clone();

        let future = Box::pin(async {
            let body = (Full::new(Bytes::from(this.s)));
            let resp = Response::builder().body(body).unwrap();
            Ok::<_, Infallible>(resp)
        });
        future
    }
}

#[derive(Clone)]
struct TimeWith<T> {
    inner_future: T,
    from_time: chrono::DateTime<chrono::Local>,
}

impl<T, Request> Service<Request> for TimeWith<T>
where
    T: Service<Request, Response = HttpResponse, Error = Infallible> + Clone + 'static,
    Request: fmt::Debug + 'static,
{
    // type Response = T::Response;
    // type Error = T::Error;
    // type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;
    type Response = T::Response;
    type Error = T::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn call(&mut self, req: Request) -> Self::Future {
        let mut this = self.clone();

        let future = Box::pin(async move {
            use bytes::Buf;
            // Collect inner future's body
            let resp = this.inner_future.call(req).await.unwrap();
            let collected_body = resp.collect().await.unwrap();
            let buf = collected_body.to_bytes();
            // println!("collected_body: {:?}", &buf);
            let body_str = String::from_utf8_lossy(&buf[..]);

            // Rebuild body
            let s = format!("{:?} with {:?}", body_str, this.from_time);
            let body = Full::new(Bytes::from(s));
            let mut resp = Response::builder().body(body).unwrap();
            Ok::<_, Infallible>(resp)
        });
        future
    }
}
