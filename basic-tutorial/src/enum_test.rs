#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

#[allow(unused)]
fn route(ip_type: IpAddrKind) {}

#[derive(Debug)]
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

#[derive(Debug)]
enum IpAddr2 {
    V4(String),
    V6(String),
}

#[derive(Debug)]
enum IpAddr3 {
    V4(u8, u8, u8, u8),
}

#[allow(unused)]
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 枚举和结构体还有另一个相似点：就像可以使用 `impl` 来为结构体定义方法那样，也可以在枚举上定义方法。
impl Message {
    fn call(&self) {
        println!("In call: {:?}", self);
    }
}

fn main() {
    println!("{:?}, {:?}", IpAddrKind::V4, IpAddrKind::V6);
    route(IpAddrKind::V6);

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    println!("{:?}", home);

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    println!("{:?}", loopback);

    println!(
        "{:?}, {:?}",
        IpAddr2::V4(String::from("127.0.0.1")),
        IpAddr2::V6(String::from("::1"))
    );

    println!("{:?}", IpAddr3::V4(127, 0, 0, 1));

    let m = Message::Write(String::from("Hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some(String::from("a string"));
    println!("{:?}, {:?}", some_number, some_string.unwrap());

    let absent_number: Option<i32> = None;
    println!("{:?}", absent_number);
}
