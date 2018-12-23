use std::thread;
use std::time::Duration;

fn main() {
    let v = vec![1, 2, 3];
    // 通过在闭包之前增加 `move` 关键字，我们强制闭包获取其使用的值得所有权，而不是任由 Rust
    // 推断它所应该借用值。
    let handler = thread::spawn(move || {
        for i in 1..10 {
            println!("spawned thread: hi number {}, here's a vector : {:?}", i, v);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("the main thread: hi number {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    handler.join().unwrap();
    // drop(v);
}
