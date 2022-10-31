// https://zenn.dev/335g/scraps/d42051832dcc55
#![allow(unused)]

use futures_util::future::BoxFuture;
use std::future::Future;
use std::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};
//
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
//
use bytes::BytesMut;

#[pin_project::pin_project(project = ConnectionProj)]
struct Connection<IO> {
    name: &'static str,
    #[pin]
    dispatcher: Option<Pin<Box<Dispatcher<IO>>>>,
}
// enum ConnectionState<IO> {
//     Prepare,
//     Connected(Pin<Box<Dispatcher<IO>>>),
//     Closed,
// }

impl<IO> Connection<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    fn new(name: &'static str, io: IO) -> Self {
        Self {
            name: name,
            dispatcher: Some(Box::pin(Dispatcher::new(io))),
        }
    }
}

impl<IO> Future for Connection<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    type Output = Result<usize, std::io::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        *this.name = "abc";

        Poll::Pending
    }
}

//--------------------------------------------------------------------------------
struct Dispatcher<IO> {
    io: IO,
    buffer: BytesMut,
}

impl<IO> Dispatcher<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    fn new(io: IO) -> Self {
        Self {
            io,
            buffer: BytesMut::with_capacity(4096),
        }
    }

    async fn read(&mut self) -> Result<usize, std::io::Error> {
        self.io.read_buf(&mut self.buffer).await
    }
}

#[tokio::main]
async fn main() -> () {
    let mock_io = tokio_test::io::Builder::new().read(b"abcde").build();
    let conn = Connection::new("abcde", mock_io);
}
