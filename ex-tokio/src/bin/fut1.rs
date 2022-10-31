// https://zenn.dev/335g/scraps/d42051832dcc55
#![allow(unused)]

use futures_util::{Future, FutureExt};
use std::future::ready;
use std::net::TcpStream;
use std::pin::Pin;
use std::task::{Context, Poll};
//
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
//
use bytes::BytesMut;

pub struct HttpConnection<IO> {
    io: IO,
    pub buffer: BytesMut,
}

impl<IO> HttpConnection<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    pub fn new(io: IO) -> Self {
        Self {
            io,
            buffer: BytesMut::with_capacity(4096),
        }
    }
    pub fn pinned(io: IO) -> Pin<Box<Self>> {
        Box::pin(Self::new(io))
    }

    pub async fn read_check(&mut self) -> Result<(), http::Error> {
        self.io.read_buf(&mut self.buffer).await;

        Ok(())
    }
}

impl<IO> Future for HttpConnection<IO>
where
    IO: AsyncRead + AsyncWrite + Unpin,
{
    type Output = Result<(), http::Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

use tokio_test::io::Builder;

#[tokio::main]
async fn main() -> () {
    let mut mock_io = Builder::new().read(b"hello ").read(b"world!").build();

    let mut con = HttpConnection::new(mock_io);
    // con.read_check().await;
    println!("bufffer: {:?}", &con.buffer);
    con.read_check().await;
    println!("bufffer: {:?}", &con.buffer);

    let mut mock_io = Builder::new().read(b"hello ").read(b"world!").build();
    let mut con = HttpConnection::pinned(mock_io);
    let x = con.await;
    // println!("bufffer: {:?}", con.buffer);
    // let x = con.await;
    // println!("bufffer: {:?}", con.buffer);
}
