extern "C" {
    fn abs(input: i32) -> i32;
}

static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
    println!("name is : {}", HELLO_WORLD);
}
