use std::env;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() {
    let mut args = env::args();
    let _    = args.next();
    let host = args.next().unwrap();
    let path = args.next().unwrap();

    let user_agent = "SimpletonHTTP/0.0.0";

    let mut stream = TcpStream::connect(host.as_str()).unwrap();
    let _ = stream.write(format!("GET {} HTTP/1.1\n", path).as_bytes());
    let _ = stream.write(format!("Host: {}\n", host).as_bytes());
    let _ = stream.write(format!("User-Agent: {}\n", user_agent).as_bytes());
    let _ = stream.write(b"Accept: */*\n");
    let _ = stream.write(b"\n");

    let mut buf: Vec<u8> = vec![];
    let _ = stream.read_to_end(&mut buf);
    println!("{}", str::from_utf8(&buf).unwrap());
}
