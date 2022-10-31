use std::task::Poll;

struct Fut {
    i: u32,
}

impl Fut {
    fn awaiting(&mut self) -> Poll<u32> {
        println!("awaiting({})", self.i);

        if self.i < 5 {
            self.i = self.i + 1;
            Poll::Pending
        } else {
            Poll::Ready(self.i)
        }
    }
}

fn main() {
    let mut f = Fut { i: 0 };

    loop {
        let r = f.awaiting();
        match r {
            Poll::Ready(v) => {
                println!("v {}", v);
                break;
            }
            Poll::Pending => continue,
        }
    }
}
