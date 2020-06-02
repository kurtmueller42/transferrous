#![allow(unused)]
use std::io;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::io::prelude::*;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut file = File::create("transferrous.out")?;
    let mut buf = vec![0; 10000];
    loop {
        match stream.read(buf.as_mut_slice()) {
          Ok(num_bytes) => {
            if num_bytes > 0 {
                file.write(&buf[0..num_bytes]);
            }
          },
          Err(_) => {
            break;
          }
        }
      }
    Ok(())
}

fn listen() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("new client!");
                handle_client(stream);
            }
            Err(e) => { /* connection failed */ }
        }
    }
    Ok(())
}


fn main() -> io::Result<()> {
    listen()
}
