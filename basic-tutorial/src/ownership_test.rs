// 所有权规则
// 1. Rust 中每一个值都有一个被称为其 `所有者(owner)`的变量
// 2. 值有且只有一个所有者
// 3. 当所有者(变量)离开作用域时，这个值将被丢弃
//

fn main() {
    // String 类型分配到堆上
    let mut s1 = String::from("hello");
    println!("{}", s1);
    s1.push_str(", world!");
    println!("{}", s1);

    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}, (s1 == s2) = {}", s1, s2, s1.eq(&s2));

    // 将值传递给函数在语义上与变量赋值相似。向函数传递值可能会移动或者复制，就像赋值语句一样。
    let s3 = String::from("hello"); // s3 进入作用域
    takes_ownership(s3);  // s3 的值移动到函数里
    // s3 在这里不再有效

    // Rust 有一个叫做 `Copy` trait 的特殊注解，可以用在类似整形这样的存储在栈上的类型上。
    // 如果一个类型拥有 `Copy` trait，一个旧的变量在将其赋值给其他变量后仍然可用。
    // Rust 不允许自身或其他任何部分实现了 `Drop` trait 的类型使用 `Copy` trait。
    // 如果我们对其值离开作用域时需要特殊处理的类型使用 `Copy` 注解，将出现一个编译时错误。
    // 任何简单标量值的组合可以是 `Copy` 的，不需要分配内存或某种形式资源的类型是`Copy`的。
    //
    // - 所有整形类型，如: u32
    // - 布尔类型, bool
    // - 所有浮点数类型，如：f64
    // - 字符类型，char
    // - 元祖，当且仅当其包含类型也是 `Copy` 的时候。比如, (i32, i32) 是 `Copy` 的, 但 (i32, String) 不是
    let x = 5;  // x 进入作用域
    makes_copy(x);  // x 应该移动函数里
    println!("copied i32: {}", x);  // 但 i32 是 Copy 的，所以在后面继续使用x

    // 返回值也可以转移所有权
    let s4 = gives_ownership();
    println!("函数返回值获取所有权: {}", s4);
    let s5 = String::from("hello");
    let s6 = takes_and_gives_back(s5); // s4 被移动到 takes_and_gives_back 中, 它也将返回值移给 s5
    println!("takes and gives back: {}", s6);
}  // 这里, s6 移出作用域并被丢弃. s5 也移出作用域, 但已被移走, 所以什么也不会发生. s4 移出作用域并被丢弃.

fn takes_ownership(some: String) {  // some 进入作用域
    println!("in function takes_ownership: {}", some);
}   // 这里, some 移出作用域并调用 `drop` 方法。占用的内存被释放。

fn makes_copy(some_integer: i32) {  // some 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作。

fn gives_ownership() -> String {
    let some_string = String::from("hello");  // some_string 进入作用域
    some_string  // 返回 some_string 并移除给调用的函数
}

fn takes_and_gives_back(a_string: String) -> String {  // a_string 进入作用域
    a_string  // 返回 a_string 并移出给调用函数
}
