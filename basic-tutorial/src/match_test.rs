#[allow(unused)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(unused)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter state: {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    println!("{}", value_in_cents(Coin::Penny));
    println!("{}", value_in_cents(Coin::Quarter(UsState::Alabama)));
    println!("{:?}", plus_one(Some(5)));

    // 通配符 `_`
    let some_u8_value = Some(0u8);
    let m = match some_u8_value {
        Some(1) => println!("one"),
        Some(2) => println!("two"),
        Some(3) => println!("three"),
        _ => (),
    };
    println!("{:?}", m);

    // `if let` 获取通过等号分隔的一个模式和一个表达式。它的工作方式与 `match` 相同，这里的表达式对应
    // `match` 而模式则对应第一个分支。
    if let Some(0) = some_u8_value {
        println!("zero");
    }
}
