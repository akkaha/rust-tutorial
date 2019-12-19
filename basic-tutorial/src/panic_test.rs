use std::fs::File;
use std::io;
use std::io::ErrorKind;
use std::io::Read;

fn main() {
    // panic!("crash and burn");
    // let v = vec![1, 2, 3];
    // v[99];

    let f = File::open("README.md");
    println!("{:?}", f);
    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("There was a problem opening the file: {:?}", error);
        }
    };
    println!("{:?}", f);
    let f = File::open("README.md").map_err(|error| {
        if ErrorKind::NotFound == error.kind() {
            File::create("README.md").unwrap_or_else(|error| {
                panic!("Tried to create file but there was a problem: {:?}", error);
            })
        } else {
            panic!("There was a problem opening the file: {:?}", error);
        }
    });
    println!("{:?}", f);
    println!("{:?}", read_username_from_file());
    println!("{:?}", read_user_from_file_short());
}

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("README.md");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

// 传播错误的简写 `?`, `?` 只能用于放回 `Result` 的函数。
fn read_user_from_file_short() -> Result<String, io::Error> {
    let mut s = String::new();
    File::open("README.md")?.read_to_string(&mut s)?;
    Ok(s)
}
