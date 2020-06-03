#![allow(unused)]
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use std::path::Path;

fn connect() -> TcpStream {
    TcpStream::connect("127.0.0.1:8001")
        .expect("Couldn't connect to the server...")
}
fn main() {
    let matches = App::new("Transferrous Client")
        .version("0.1")
        .author("km42")
        .about("Transfer files")
        .arg(Arg::new("INPUT")
            .about("Sets the input file to use")
            .required(true)
            .index(1))
        .get_matches();

    if let Some(ref in_file) = matches.value_of("INPUT") {
        let path = Path::new(matches.value_of("INPUT").unwrap());
        if !path.exists() {
            println!("File not found");
            return;
        } else {
            let mut file = File::open(path).expect("couldn't open file");
            
            let mut stream: TcpStream = connect();
            println!("Connected to the server!");

            stream.write(format!("{}\n", path.to_str().unwrap()).as_bytes());

            let mut buf = vec![0; 10000];
            loop {
                match file.read(buf.as_mut_slice()) {
                  Ok(num_bytes) => {
                    if num_bytes > 0 {
                        stream.write_all(&buf[0..num_bytes]);
                    } else {
                        break;
                    }
                  },
                  Err(_) => {
                    println!("ERR on readfile; breaking");
                    break;
                  }
                }
            }
            
            println!("finished writing to server");        
        }
    }
}
