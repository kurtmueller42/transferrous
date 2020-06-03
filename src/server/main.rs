#![allow(unused)]
use std::io;
use std::net::{TcpListener, TcpStream};
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

const CHUNK_SIZE: usize = 65536;

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut buf = vec![0; CHUNK_SIZE];

    let mut outfile_name = String::new();
    
    reader.read_line(&mut outfile_name);
    &outfile_name.pop();

    let mut file = File::create(format!("{}.out", &outfile_name))?;

    loop {
        match reader.read(buf.as_mut_slice()) {
          Ok(num_bytes) => {
            if num_bytes > 0 {
                file.write(&buf[0..num_bytes]);
            } else {
                println!("Finished reading from socket");
                break;
            }
          },
          Err(_) => {
            println!("Error reading from socket");
            break;
          }
        }
      }
    Ok(())
}

fn listen() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8001")?;

    println!("Now listening for clients!");

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
