#[allow(unused)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// Tuple structs
#[derive(Debug)]
struct Color(i32, i32, i32);

#[derive(Debug)]
struct Point(i32, i32, i32);

fn main() {
    let mut user1 = User {
        email: String::from("hello@gmail.com"),
        username: String::from("hello"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("world@gmail.com");

    let user2 = build_user(String::from("ha@g.com"), String::from("ha"));
    println!("user2.username: {}, user2.email: {}", user2.username, user2.email);

    let user3 = User {
        username: String::from("user3"),
        ..user2
    };
    println!("user3.username: {}, user3.email: {}", user3.username, user3.email);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("black: {:?}, origin: {:#?}", black, origin);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
