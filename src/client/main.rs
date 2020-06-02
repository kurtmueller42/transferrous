#![allow(unused)]
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::fs::File;


fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8001")
        .expect("Couldn't connect to the server...");

    println!("Connected to the server!");
    let mut file = File::open("transferrous.in").expect("couldn't open infile");
    let mut buf = vec![0; 10000];
    loop {
        match file.read(buf.as_mut_slice()) {
          Ok(num_bytes) => {
            if num_bytes > 0 {
                stream.write(&buf[0..num_bytes]);
            } else {
                println!("breaking EOF");
                break;
            }
          },
          Err(_) => {
            println!("breaking");
            break;
          }
        }
    }
    
    println!("finished writing to server");
}
