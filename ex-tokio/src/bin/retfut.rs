// https://zenn.dev/335g/scraps/d42051832dcc55

use futures_util::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct ReturnFuture<T>(Option<T>);

impl<T> ReturnFuture<T> {
    fn new(x: T) -> Self {
        ReturnFuture(Some(x))
    }
}

// TがUnpinを実装していたら、var.awaitできるってこと？
impl<T> Future for ReturnFuture<T>
where
    T: Unpin,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, _ctx: &mut Context<'_>) -> Poll<Self::Output> {
        let elem = self.get_mut().0.take().expect("");

        Poll::Ready(elem)
    }
}

#[tokio::main]
async fn main() {
    let f = ReturnFuture::new(1);

    println!("f: {}", f.await);
}

#[cfg(test)]
mod tests {
    #![allow(unused)]
    use std::boxed::Box;
    use std::marker::PhantomPinned;
    use std::pin::Pin;

    use crate::ReturnFuture;
    use pin_project::pin_project;

    #[pin_project]
    #[derive(Debug)]
    struct X {
        #[pin]
        v: i32,
        v_ptr: *const i32,
    }

    impl X {
        fn new() -> Self {
            let mut this = Self {
                v: 1,
                v_ptr: std::ptr::null(),
            };
            this.v_ptr = &this.v;
            this
        }
        fn pinned() -> Pin<Box<Self>> {
            let mut boxed = Box::pin(Self::new());
            let ptr = &boxed.v;
            boxed.as_mut().get_mut().v_ptr = ptr;
            boxed
        }

        fn set_ptr(self: &mut Self) {
            self.v_ptr = &self.v;
        }

        fn show(self: &Self) {
            println!("self: {:p}", self);
            let is_same = (&self.v as *const i32) == self.v_ptr;
            println!("ptr_check: {}, {:p} | {:p}", is_same, &self.v, self.v_ptr);
        }
    }

    #[tokio::test]
    async fn test_cant_compile() {
        println!("--------- x1 ----------");
        let mut x1 = X::new(); // returnで場所が変わるのでずれる
        x1.show();
        println!("- x1.set_ptr()");
        x1.set_ptr();
        x1.show();
        println!("----- move variable"); // vとv_ptrの場所がずれる
        let x12 = x1;
        x12.show();

        println!("--------- x2 ----------");
        let x2 = X::pinned(); // pinにいれておくとreturnでずれない
        x2.show();
        println!("----- move variable"); // もちろんvariableを移動してもずれない
        let x3 = x2;
        x3.show();
        // Boxだとメモリコピーの可能性があるけどPinに入ってるのでそれも心配ない
    }

    #[derive(Debug)]
    struct Y {
        v: i32,
        _pinned: PhantomPinned,
    }

    impl Y {
        fn new() -> Self {
            Self {
                v: 1,
                _pinned: PhantomPinned,
            }
        }
    }

    #[tokio::test]
    async fn test_fut() {
        let f = ReturnFuture::new(1);
        println!("f: {}", f.await);

        let f = ReturnFuture::new(X::new());
        println!("f: {:?}", f.await);

        // Yがpinされてないのでawaitできない
        // let f = ReturnFuture::new(Y::new());
        // println!("f: {:?}", f.await);
    }

    #[derive(Debug)]
    struct Z {
        y: Y,
    }

    impl Z {
        fn new() -> Self {
            Z { y: Y::new() }
        }
    }

    #[tokio::test]
    async fn test_z() {
        // 内部フィールドが全部UnpinでないとUnpinとされない
        // y: Yがpinされてないのでawaitできない
        // let f = ReturnFuture::new(Z::new());
        // println!("f: {:?}", f.await);
    }
}
