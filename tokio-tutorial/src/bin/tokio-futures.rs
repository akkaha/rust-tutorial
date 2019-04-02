extern crate bytes;
#[macro_use]
extern crate futures;
extern crate tokio;

use std::fmt;
use std::io::{self, Cursor};

use bytes::{Buf, Bytes};
use futures::{Async, Future, Poll};
use tokio::io::AsyncWrite;
use tokio::net::{ConnectFuture, TcpStream};

//////////////////////////
// Implementing Futures //
//////////////////////////
struct HelloWorld;

impl Future for HelloWorld {
    type Item = String;
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        Ok(Async::Ready("hello world".to_string()))
    }
}

struct Display<T>(T);

impl<T> Future for Display<T>
    where
        T: Future,
        T::Item: fmt::Display,
{
    type Item = ();
    type Error = T::Error;

    fn poll(&mut self) -> Poll<(), Self::Error> {
        /*let value = match self.0.poll() {
            Ok(Async::Ready(value)) => value,
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Err(err) => return Err(err),
        };*/
        let value = try_ready!(self.0.poll());
        println!("{}", value);
        Ok(Async::Ready(()))
    }
}

//////////////////////////
// Getting Asynchronous //
//////////////////////////
struct GetPeerAddr {
    connect: ConnectFuture,
}

impl Future for GetPeerAddr {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        match self.connect.poll() {
            Ok(Async::Ready(socket)) => {
                println!("peer address = {}", socket.peer_addr().unwrap());
                Ok(Async::Ready(()))
            }
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(e) => {
                println!("failed to connect: {}", e);
                Ok(Async::Ready(()))
            }
        }
    }
}

///////////////////////////
// Chaining Computations //
///////////////////////////
/*enum HelloWorld2 {
    Connecting(ConnectFuture),
    Connected(TcpStream, Cursor<Bytes>),
}

impl Future for HelloWorld2 {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        use self::HelloWorld2::*;
        loop {
            match self {
                Connecting(ref mut f) => {
                    let socket = try_ready!(f.poll());
                    let data = Cursor::new(Bytes::from_static(b"hello world"));
                    *self = Connected(socket, data);
                }
                Connected(ref mut socket, ref mut data) => {
                    while data.has_remaining() {
                        try_ready!(socket.write_buf(data));
                    }
                    return Ok(Async::Ready(()));
                }
            }
        }
    }
}*/


fn main() {
    // Implementing futures
    let future = Display(HelloWorld);
    tokio::run(future);

    // Getting asynchronous
    let addr = "220.181.57.216:80".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let get_peer_addr = GetPeerAddr {
        connect: connect_future,
    };
    tokio::run(get_peer_addr);

    // Combinators
    /*let addr = "220.181.57.216:80".parse().unwrap();
    let connect_future = TcpStream::connect(&addr);
    let hello_world2 = HelloWorld2::Connecting(connect_future);
    tokio::run(hello_world2.map_err(|e| println!("{0}", e)))*/
}
