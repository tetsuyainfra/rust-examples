// hyper_stream.rsの複数タイプのレスポンスを返すタイプ
// Full<D, Error>のErrorをInfallibleをhyper::Errorに取り替えてBoxBodyに入れるのが肝
// BoxBodyはtrait Bodyを受け取る型
//
// curl localhost:3000 -i
//

#![allow(unused)]
use std::convert::Infallible;
use std::net::SocketAddr;
use std::ops::Add;
use std::task::Poll;
use std::time::{Duration, Instant};

use bytes::Bytes;
use futures_util::future::Pending;
use tokio::io::AsyncBufReadExt;
use tokio::net::TcpListener;
//
use http_body_util::combinators::BoxBody;
use http_body_util::Full;
use hyper::body::Frame;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response, StatusCode};

use tracing::{debug, error, info};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_logger();
    let addr: SocketAddr = ([127, 0, 0, 1], 3000).into();
    let listener = TcpListener::bind(addr).await?;
    println!("Listening on http://{}", addr);

    loop {
        let (stream, _) = listener.accept().await?;

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                // .serve_connection(stream, service_fn(hello))
                .serve_connection(stream, service_fn(my_count))
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn hello(_: Request<hyper::body::Incoming>) -> Result<Response<Full<Bytes>>, Infallible> {
    Ok(Response::new(Full::new(Bytes::from("Hello World!"))))
}

async fn my_count(
    req: Request<hyper::body::Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, Infallible> {
    info!("req: {:?}", &req);
    if req.uri().path() == "/api" {
        let body = BoxBody::new(get_response());
        let resp = Response::builder()
            .status(StatusCode::ACCEPTED)
            .header("Access-Control-Allow-Origin", "*")
            .header("Content-Type", "text/plain")
            // .body(get_response())
            .body(body)
            .unwrap();
        return Ok(resp);
    }
    let frame = full(Bytes::from("hello world boxed full"));
    let body = BoxBody::new(frame);
    let resp = Response::builder().body(body).unwrap();

    Ok(resp)
}

use http_body_util::BodyExt;
fn full<T: Into<Bytes>>(chunk: T) -> BoxBody<Bytes, hyper::Error> {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

fn get_response() -> MyBody {
    MyBody {
        count: 0,
        last_send_time: Instant::now(),
    }
}
pub struct MyBody {
    count: usize,
    last_send_time: Instant,
}
impl hyper::body::Body for MyBody {
    type Data = Bytes;

    // type Error = hyper::Error;
    type Error = hyper::Error;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        let now = Instant::now();
        if now - self.last_send_time > Duration::new(1, 0) {
            let count = self.count.clone();

            // update
            let this = self.get_mut();
            this.count = count + 1;
            this.last_send_time = now;

            let s = format!("count: {}\n", count);
            let frame = Frame::data(Bytes::from(s));
            return Poll::Ready(Some(Ok(frame)));
        }

        cx.waker().wake_by_ref();
        Poll::Pending
    }

    fn is_end_stream(&self) -> bool {
        false
    }

    fn size_hint(&self) -> hyper::body::SizeHint {
        hyper::body::SizeHint::default()
    }
}

// https://github.com/hyperium/hyper/blob/75aac9f47fe0246016e6133cd3cfa35b63c8904e/src/proto/h1/dispatch.rs#L291

////////////////////////////////////////////////////////////////////////////////
fn init_logger() {
    let builder = tracing_subscriber::FmtSubscriber::builder()
        .with_thread_ids(true)
        // .with_max_level(tracing::Level::TRACE);
        .with_max_level(tracing::Level::DEBUG);
    // .with_max_level(tracing::Level::INFO);
    let subscriber = builder.finish();
    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");
}

/*
```javascript
fetch("http://localhost:3000/")
.then(async response => {
  // 結果を数えるための変数
  let count = 0;
  // response.body にレスポンス本文のストリーム（ReadableStream）が入っている
  // ストリームのReaderを作成
  const reader = response.body.getReader();
  while (true) {
    // ストリームからデータを読む
    const {done, value} = await reader.read();
    if (done) {
      // doneがtrueならストリームのデータを全部読み終わった
      break;
    }
    console.log(count++,  new TextDecoder().decode(value)); // Uint8Array to String
  }
  return count;
}).then(count => console.log(count));
```
*/
