use std::collections::HashMap;

#[allow(unused)]
fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let scores2: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
    println!("{:?}", scores2);

    println!("{:?}", scores2.get(&String::from("Blue")));

    for (k, v) in &scores2 {
        println!("{}: {}", k, v);
    }
}
