fn main() {
    let number = 3;
    if number < 5 {
        println!("condition is true")
    } else {
        println!("condition is false")
    }

    let number = if true {
        5
    } else {
        6
    };
    println!("value from if expression {}", number);

    // lets loop
    let mut index = 0;
    while index < 5 {
        println!("Again {}", index);
        index = index + 1;
    }

    // iterate array
    let a = [10, 20, 30, 40, 50];
    for ele in a.iter() {
        println!("iter: {}", ele);
    }

    for num in (1..4).rev() {
        println!("{}!", num);
    }
}
