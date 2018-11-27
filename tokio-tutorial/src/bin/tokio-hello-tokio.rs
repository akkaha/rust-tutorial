extern crate tokio;

use tokio::io::copy;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    // bind the server's socket.
    let addr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr)
        .expect("unable to bind TCP listener");
    // pull out a stream of sockets for incoming connections.
    let server = listener.incoming()
        .map_err(|e| eprintln!("accept failed = {:?}", e))
        .for_each(|sock| {
            println!("coming connection: {:?}", sock);
            // Split up the reading and writing parts of socket.
            let (reader, writer) = sock.split();
            // A future that echos the data and returns how many bytes were copied...
            let bytes_copied = copy(reader, writer);
            // ... after which we'll print what happened.
            let handle_conn = bytes_copied.map(|amt| {
                println!("wrote {:?} bytes", amt)
            }).map_err(|err| {
                eprintln!("IO error {:?}", err)
            });
            // Spawn the future as a concurrent task.
            tokio::spawn(handle_conn)
        });
    tokio::run(server);
}