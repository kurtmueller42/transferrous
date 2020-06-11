#![allow(unused)]
use std::io;
use std::fs::File;
use std::io::prelude::*;
use tokio::io::BufReader;
use tokio::prelude::*;
use tokio::net::{TcpStream, TcpListener};
use std::error::Error;

const CHUNK_SIZE: usize = 65536;

async fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut reader = BufReader::new(stream);
    let mut buf = vec![0; CHUNK_SIZE];

    let mut outfile_name = String::new();
    
    reader.read_line(&mut outfile_name);
    &outfile_name.pop();

    let mut file = File::create(format!("{}.out", &outfile_name))?;

    loop {
        match reader.read(buf.as_mut_slice()).await {

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

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = "127.0.0.1:8001".to_string();
    let mut listener = TcpListener::bind(&addr).await?;
    println!("Now listening for clients!");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            std::thread::sleep(std::time::Duration::new(10,0));
            handle_client(socket).await
        });

    }
}
