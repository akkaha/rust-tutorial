#[allow(unused)]
fn main() {
    let v1: Vec<i32> = Vec::new();
    println!("{:?}", v1);

    // `vec!` 宏，这个宏会根据我们提供的值来创建一个新的 `Vec`。
    let v2 = vec![1, 2, 3];
    println!("{:?}", v2);

    let mut v3 = Vec::new();
    v3.push(4);
    v3.push(5);
    v3.push(6);
    println!("{:?}", v3);

    for i in &mut v3 {
        println!("item: {}", i);
        *i += 100;
    }
    // let does_not_exist = &v3[100];
    let does_not_exist = v3.get(100);
    println!("{:?}", does_not_exist);
    for i in &v3 {
        println!("item: {}", i);
    }
}
