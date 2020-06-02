#![allow(unused)]
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8001")
        .expect("Couldn't connect to the server...");

    println!("Connected to the server!");
    let buf = "foo".as_bytes();
    stream.write(&buf);
}
