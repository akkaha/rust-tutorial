// `slice` 是另一个没有所有权的数据类型 `slice`. `slice` 允许你引用集合中一段连续的元素序列,
// 而不用引用整个集合.

#[allow(unused)]
fn main() {
    let mut words = String::from("hello world");
    let word = first_word(&words);
    println!("word: {}, words: {}", word, words);

    // string slice
    let mut s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];
    println!("slice: {} {} {}", hello, world, &s[0..]);
}

// 函数含有一个 `&String`, 因为我们不需要所有权.
fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
