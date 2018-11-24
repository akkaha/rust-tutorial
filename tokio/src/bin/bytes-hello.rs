extern crate bytes;

use bytes::{BytesMut, BufMut, BigEndian};

fn main() {
    let mut buf = BytesMut::with_capacity(1024);
    buf.put(&b"hello world"[..]);
    buf.put_u16::<BigEndian>(1234);
}
