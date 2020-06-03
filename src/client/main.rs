#![allow(unused)]
use std::net::TcpStream;
use std::io;
use std::io::prelude::*;
use std::fs::File;
use clap::{Arg, App};
use std::path::Path;

mod send_file;
use send_file::*;

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
        send_file(path)
    }
}
