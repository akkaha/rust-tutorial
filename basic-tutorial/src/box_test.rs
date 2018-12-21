use std::ops::Deref;

use List::{Cons, Nil};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1,
                    Box::new(Cons(2,
                                  Box::new(Cons(3,
                                                Box::new(Nil))))));
    println!("{:?}", list);

    let x = 5;
    let y = MyBox::new(x);

    println!("{}", x);
    println!("{:?}", y);
    println!("{:?}", *y);
    println!("{:?}", *(y.deref()));

    let m = MyBox::new(String::from("Rust"));

    // 解引用强制多态是 Rust 表现在函数或方法传参上的一种便利。其将实现了 `Deref`
    // 的类型的引用转换为原始类型通过 `Deref` 所能够转换的引用类型的引用。
    // 当这种特定类型的引用作为实参传递给和形参类型不同的函数或方法时，解引用强制多态将自动发生。
    // 这时会有一系列的 `deref` 方法被调用，把我们提供的类型转换成了参数所需的类型。

    // 这里使用 `&m` 调用 `hello` 函数，其为 `MyBox<String>` 值所引用。因为 `MyBox<T>`
    // 上实现了 `Deref` trait，Rust 可以通过 `deref` 调用将 `&MyBox<String>` 变为 `&String`。
    // 标准库中提供了 `String` 上的 `Deref` 实现，其会返回字符串 slice，这可以在
    // `Deref` 的 API 文档中看到。Rust 再次调用 `deref` 将 `&String` 变为 `&str`，这就符合 `hello` 函数的定义了。
    hello(&m);

    hello(&(*m)[..]);

    // Rust 在发现类型和trait实现满足三种情况时会进行解引用强制多态：
    // - 当 `T: Deref<Target=U> 时从 `&T` 到 `&U`
    // - 当 `T: DefefMut<Target=U> 时从 `&mut T` 到 `&mut U`
    // - 当 `T: Deref<Target=U>` 时从 `&mut T` 到 `&U`
}

#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

#[derive(Debug)]
struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.0
    }
}

fn hello(name: &str) {
    println!("hello, {}", name);
}
