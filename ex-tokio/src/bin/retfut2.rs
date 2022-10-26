// https://zenn.dev/335g/scraps/d42051832dcc55
// pin_projectを使ってみる
//

struct SelfRef<T> {
    x: T,
    x_ptr: *const T,
}

impl<T> SelfRef<T> {
    fn new(x: T) -> SelfRef<T> {
        let mut this = Self {
            x: x,
            x_ptr: std::ptr::null(),
        };
        this.x_ptr = &this.x;
        assert_eq!(&this.x as *const T, this.x_ptr);

        this
    }
    fn set_ptr(&mut self) {
        println!("set_ptr()");
        self.x_ptr = &self.x;
    }

    fn is_same(&self) -> bool {
        (&self.x as *const T) == self.x_ptr
    }

    fn ptr_check(&self) {
        println!(
            "ptr_check: {}, {:p} | {:p}",
            &self.is_same(),
            &self.x,
            self.x_ptr
        );
    }
}

#[tokio::main]
async fn main() {
    println!("----- r1");
    let mut r1 = SelfRef::new(1);
    r1.ptr_check();
    r1.set_ptr();
    r1.ptr_check();

    println!("----- r2");
    let r2 = r1;
    r2.ptr_check();
}
