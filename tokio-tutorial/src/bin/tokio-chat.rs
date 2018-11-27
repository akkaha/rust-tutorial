//! A chat server that broadcasts a message to all connections.
//!
//! This example is explicitly more verbose than it has to be. This is to illustrate more concepts.
//!
//! A chat server for telnet clients. After a telnet client connects, ths first line should contain
//! the client's name. After that, all lines send by a client are broadcasted to all other connected
//! clients.
//!
//! Because the client is telnet, lines are delimited by "\r\n".
//!
//! You can run the `telnet` command in any number of additional windows.
//!

extern crate bytes;
extern crate futures;
extern crate tokio;

use bytes::{BufMut, Bytes, BytesMut};
use futures::future::{self, Either};
use futures::sync::mpsc;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tokio::io;
use tokio::net::{TcpListener, TcpStream};
use tokio::prelude::*;

/// shorthand for the transmit half of the message channel.
type Tx = mpsc::UnboundedSender<Bytes>;
/// shorthand for the receive half of the message channel.
type Rx = mpsc::UnboundedReceiver<Bytes>;

/// data that is shared between all peers in the chan server.
///
/// This is the set of `Tx` handles for all connected clients. Whenever a message is received from a
/// client, it is broadcasted to all peers by iterating over the `peers` entries and sending a copy
/// of the message on each `Tx`
struct Shared {
    peers: HashMap<SocketAddr, Tx>
}

/// The state for each connected client.
struct Peer {
    /// Name of peer.
    ///
    /// When a client connects, the first line send is treated as the client's name (like alice or
    /// bob). The name is used to preface all messages that arrive from the client so that we can
    /// simulate a real chat server:
    ///
    /// ```text
    /// alice: Hello everyone.
    /// bob: Welcome to telnet chat!
    /// ```
    name: BytesMut,

}

fn main() {
    println!("tokio-chat")
}