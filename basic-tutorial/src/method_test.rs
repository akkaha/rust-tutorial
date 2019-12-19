#[allow(unused)]
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为了使用函数定义于 `Rectangle` 的上下文中, 我们开始了一个`impl`块. 接着将 `area` 函数移动到
// `impl` 大括号中. 在 `area` 的签名中, 使用 `&self` 来替代 `rectangle: &Rectangle`, 因为该方法
// 位于 `impl Rectangle` 上下文中所以 Rust 知道 `self` 的类型是 `Rectangle`. 注意仍然需要在 `self`
// 前面加上 `&`, 就像 `&Rectangle` 一样. 方法可以获取 `self` 的所有权.
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

// 每个结构体都允许拥有多个 `impl` 块.
impl Rectangle {
    // 关联函数: `impl` 中不以 `self` 作为参数的函数. 这被称为 `关联函数(associated functions)`, 因为它们与
    // 结构体相关联. 它们仍是函数而不是方法, 因为它们不作用于一个结构体的实例. 如: `String::from`
    // 关联函数经常被用作返回一个结构体的新实例的构造函数.
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    // Rust 有一个叫`自动引用和解引用(automatic referencing and dereferencing)`的功能.
    // `rect1.area()` 和 `&rect1.area()` 是等价的.
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        &rect1.area()
    );
    let rect2 = Rectangle {
        width: 22,
        height: 11,
    };
    println!("hold: {}", rect1.can_hold(&rect2));
    println!("square: {:?}", Rectangle::square(240));
}
