// 在任意给定时间, 要么只能有一个可变引用，要么只能有多个不可变引用
// 引用必须总是有效的
#[allow(unused)]
fn main() {
    let s1 = String::from("hello");

    // &s1 语法创建一个指向 `s1` 的引用, 但是并不拥有它. 因为并不拥有这个值, 当引用离开作用域时
    // 其指向的值也不会被丢弃.
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    // 可变引用, 必须将 `s` 改为 `mut`. 然后创建一个可变引用 `&mut s` 和接受一个可变引用
    let mut s = String::from("hello");
    change(&mut s);
    println!("mutable reference: {}", s);

    println!("no dangle: {}", no_dangle());
}

// 以一个对象的`引用`作为参数而不是获取值的所有权
// 使用 `&` 引用相反的操作是 `解引用(dereferencing)`, 它使用解引用运算符, `*`.
//
// 变量 `s` 有效的作用域与函数参数的作用域一样, 不过当引用离开作用域后并不丢弃它指向的数据, 因为我们没有所有权.
// 我们将获取引用作为函数参数称为 `借用`(borrowing).
// 不过可变引用有一个很大的限制: 在特定作用域中的特定数据有且只有一个引用。
fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some: &mut String) {
    some.push_str(", world");
}

//fn dangle() -> &String {  // dangle 返回一个 String 的引用
//    let s = String::from("hello");  // s 是新创建的 String
//    &s  // 我们返回 String 的引用, s
//}  // 这里, s 移出了作用域, 并被丢弃. 它的内存被释放.

// 所有权被移动出去, 所以没有值被释放.
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
