extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

fn main() {
    let addr = "127.0.0.1:8080".parse().unwrap();

    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    let listener = TcpListener::bind(&addr).unwrap();
    // The server task asynchronously iterates over and processes each incoming connection.
    let server = listener.incoming().for_each(|socket| {
        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

        let connection = io::write_all(socket, "hello world\n")
            .then(|res| {
                println!("wrote messages; success={:?}", res.is_ok());
                Ok(())
            });
        // Spawn a new task that processes the socket:
        tokio::spawn(connection);

        Ok(())
    }).map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error handling and helps avoid
        // silencing failures.
        // In our example, we only going to log the error to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:8080");

    // Start the Tokio runtime.
    //
    // The Tokio is a pre-configured "out of the box" runtime for building
    // asynchronous applications. It includes both a reactor and a task
    // scheduler. This means applications are multi-threaded by default.
    //
    // This function blocks until the runtime reaches an idle state.
    // Idle is defined as all spawned tasks have completed and all I/O resources
    // (TCP sockets in our case) have been dropped.
    //
    // In our example, we have not defined a shutdown strategy, so this will block
    // until `ctrl-c` is pressed at the terminal.
    tokio::run(server);
}
