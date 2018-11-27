// Rust 代码中的函数和变量名使用 snake case 规范风格（所有字母都是小写并使用下划线分隔单词）。
//
// 语句(Statements) 是执行一些操作但不返回的指令。表达式(Expressions)计算并产生一个值。
// 表达式的结尾没有分号，如果在表达式的结尾加上分号，它就变成了语句，而语句不会返回值。

#[allow(unused)]
fn main() {
    let z = another_function(1, 2);
    println!("{}", z);
}

fn another_function(x: i32, y: i32) -> i32 {
    { x + y }
}
