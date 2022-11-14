#[tokio::main]
async fn main() {
    fn_();
    fn_mut_();
    fn_once_();

    // ?Sized(fat pointer)
    let fn_func: Box<dyn Fn() -> i32> = Box::new(fi32);
    let fn_: Box<dyn Fn() -> i32> = Box::new(|| {
        //
        1
    });
    let fn_mut_: Box<dyn FnMut() -> i32> = Box::new(|| {
        //
        1
    });
    let fn_once_: Box<dyn FnOnce() -> i32> = Box::new(|| {
        //
        1
    });
    // let fn_clone = fn_.clone()

    // Sized
    let f: fn() -> i32 = || {
        //
        1
    };
    println!("fn_func= {:?}", as_raw_bytes(&fn_func));
    println!("Fn     = {:?}", as_raw_bytes(&fn_));
    println!("FnMut  = {:?}", as_raw_bytes(&fn_mut_));
    println!("FnOnce = {:?}", as_raw_bytes(&fn_once_));
    println!("fn     = {:?}", as_raw_bytes(&f));
}

// xの中身をバイト列として見るための関数
// https://qnighy.hatenablog.com/entry/2017/03/04/131311
fn as_raw_bytes<'a, T: ?Sized>(x: &'a T) -> &'a [u8] {
    unsafe { std::slice::from_raw_parts(x as *const T as *const u8, std::mem::size_of_val(x)) }
}

fn fi32() -> i32 {
    1
}

fn fn_() {
    println!("--- Fn --");
    let f: Box<dyn Fn() -> i32> = Box::new(|| {
        //
        1
    });
    println!("{}", f());
    println!("{}", f());
}

fn fn_mut_() {
    println!("--- FnMut --");
    let mut i = 1;
    // mutが必要
    let mut f: Box<dyn FnMut() -> i32> = Box::new(|| {
        //
        i = i + 1;
        i
    });
    println!("{}", f());
    println!("{}", f());
}

fn fn_once_() {
    println!("--- FnOnce --");
    let mut i = 1;
    let f: Box<dyn FnOnce() -> i32> = Box::new(|| {
        i = i + 1;
        i
    });
    println!("{}", f());
    // println!("{}", f()); // 一度しか呼び出せない
}
