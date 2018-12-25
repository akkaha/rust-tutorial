extern crate webserver;

use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;

use webserver::ThreadPool;

#[allow(unused)]
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        pool.execute(|| {
            handle_connection(stream);
        })
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
    // let contents = fs::read_to_string("README.md").unwrap();
    let contents = String::from("<p style=\"color:red;\">hello</p>");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
