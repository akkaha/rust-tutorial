extern crate tokio;
extern crate tokio_io;

use std::env;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::prelude::*;
use tokio_io::codec::BytesCodec;

fn main() {
    // Allow passing an address to listen on as the first argument of this program, but otherwise
    // we'll just set up our TCP listener on 127.0.0.1:8080 for connections.
    let addr = env::args().nth(1).unwrap_or("127.0.0.1:8080".to_string());
    let addr = addr.parse::<SocketAddr>().unwrap();

    // Next up we create a TCP listener which will listen for incoming connections.
    // This TCP listener is bound to the address we determined above and must be associated
    // with an event loop, so we pass in a handle to our event loop. After the socket's created we
    // inform that we're ready to go and start accepting connections.
    let socket = TcpListener::bind(&addr).unwrap();
    println!("Listening on: {}", socket.local_addr().unwrap());

    // Here we convert the `TcpListener` to a stream of incoming  connections with the `incoming`
    // method. We then define how to process each element in the stream with the `for_each` method.
    //
    // This combinator, defined on the `Stream` trait, will allow us to define a computation th
    // happen for all items on the stream (in this case TCP connection to happen for all items on
    // the stream (in the case TCP connections made to the server). The return value of the `for_each`
    // method is itself a future representing processing the entire stream of connections, and ends
    // up being our sever.
    let done = socket
        .incoming()
        .map_err(|e| println!("failed to accept socket; error = {:?}", e))
        .for_each(move |socket| {
            // Once we're inside this closure this represents an accepted client
            // from our server. The `socket` is the client connections (similar to
            // how the standard library operates).
            //
            // We're parsing each socket with the `BytesCodec` included in `tokio_io`,
            // and then we `split` each codec into the reader/writer halves.
            let framed = socket.framed(BytesCodec::new());
            let (_writer, reader) = framed.split();

            let processor = reader
                .for_each(|bytes| {
                    println!("bytes: {:?}", bytes);
                    Ok(())
                })
                // After our copy operation is complete we just print out some helpful information.
                .and_then(|()| {
                    println!("Socket received FIN packet and closed connection");
                    Ok(())
                })
                .or_else(|err| {
                    println!("Socket closed with error: {:?}", err);
                    Err(err)
                })
                .then(|result| {
                    println!("Socket closed with result: {:?}", result);
                    Ok(())
                });

            // And this is where much of the magic of this server happens.
            // We crucially want all clients to make progress concurrently, rather than
            // blocking one on completion of another. To achieve this we use the `tokio::spawn`
            // function to execute the work in the background.
            //
            // Essentially here we're executing a new task to run concurrently,
            // which will allow all of our clients to processed concurrently.
            tokio::spawn(processor)
        });

    // And finally now that we've define what our server is, we run it!
    //
    // This start the Tokio runtime, spawns the server task, and blocks the
    // current thread until all tasks complete execution. Since the `done` task
    // never completes (it just keeps accepting sockets), `tokio::run` blocks
    // forever (until ctrl-c is pressed).
    tokio::run(done);
}