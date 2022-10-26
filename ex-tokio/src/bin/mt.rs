#[tokio::main]
async fn main() {
    let mut handles = vec![];
    for i in 0..32 {
        println!("create {} task", i);
        let join = tokio::spawn(async move {
            println!("start {}", i);
            let mut tgt = Target::new();
            let mut n = 1;
            for _i in 0..(i32::MAX as i64 * 4) {
                n = tgt.calc(n);
            }
            return n;
        });

        handles.push((i, join));
    }

    for (i, handle) in handles {
        let n = handle.await.unwrap();
        println!("i({}) -> n: {}", i, n);
    }
}

#[derive(Debug)]
struct Target(i32);

impl Target {
    fn new() -> Self {
        Self(0)
    }

    fn calc(&mut self, n: i32) -> i32 {
        let n1 = self.0 + (1 - 2 * (n % 2));
        self.0 = n1;
        n1
    }
}
