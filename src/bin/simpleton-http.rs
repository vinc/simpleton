extern crate simpleton;

use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::io::BufReader;
use std::str;

use simpleton::http::Request;

fn main() {
    let mut verbose = false;

    let args: Vec<_> = env::args().filter(|arg| {
        if arg == "--verbose" {
            verbose = true;
        }
        !arg.starts_with("--")
    }).collect();

    if args.len() < 3 {
        println!("Usage: simpleton-http [--verbose] <host> <path>");
        return;
    }

    let host = &args[1];
    let path = &args[2];

    let mut req = Request::new("GET", &host, &path);

    let stream = TcpStream::connect(host.as_str()).unwrap();

    req.send(&stream);

    if verbose {
        for line in req.to_string().lines() {
            println!("> {}", line);
        }
    }

    let mut is_header = true;
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        match line {
            Err(_) => continue,
            Ok(line) => {
                if is_header {
                    if verbose {
                        println!("< {}", line);
                    }
                } else {
                    println!("{}", line);
                }
                if line == "" {
                    is_header = false;
                }
            }
        }
    }
}
