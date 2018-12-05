#[allow(unused)]
fn main() {
    let data = "initial contents";
    let mut s = data.to_string();
    println!("{}", s);
    s.push_str(" bar");
    s.push('r');
    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    // let s3 = s1 + &s2;
    // println!("{}", s3);
    let s = format!("{}{}", s1, s2);
    println!("{}", s);
}
