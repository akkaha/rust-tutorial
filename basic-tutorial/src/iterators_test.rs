fn main() {
    // 迭代器是惰性的
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    let v2: Vec<_> = v1_iter.map(|x| x + 1).collect();

    for val in v2.iter() {
        println!("Got: {}", val);
    }

    let c = Counter::new();
    for val in c {
        println!("{}", val);
    }
}

struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += 1;

        if self.count < 6 {
            Some(self.count)
        } else {
            None
        }
    }
}
