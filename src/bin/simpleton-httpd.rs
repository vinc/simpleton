extern crate time;
extern crate simpleton;

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::IpAddr;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::process::exit;
use std::str;
use std::thread;

use simpleton::http::server::{Options, Request, Response};

fn main() {
    let opts = Options::from_args(std::env::args().collect());

    println!("{}", opts.name);

    let listener = match TcpListener::bind((opts.address, opts.port)) {
        Err(e)       => exit_on_error(e),
        Ok(listener) => listener
    };
    println!("Listening on {}:{}", opts.address, opts.port);

    for stream in listener.incoming() {
        match stream {
            Err(e)     => exit_on_error(e),
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream, opts)
                });
            }
        }
    }

    drop(listener);
}

fn handle_client(stream: TcpStream, opts: Options) {
    let address = stream.peer_addr().unwrap().ip();

    // Read the request message
    let mut lines = vec![];
    let reader = BufReader::new(&stream);
    for line in reader.lines() {
        match line {
            Err(_) => return,
            Ok(line) => {
                if line == "" {
                    break
                } else {
                    lines.push(line)
                }
            }
        }
    }
    let buf = lines.join("\n");

    let req = match Request::from_str(&buf) {
        Err(_)  => return,
        Ok(req) => req
    };

    let mut res = Response::new();

    if req.method != "GET" && req.method != "HEAD" {
        res.status_code = 501;
        res.status_message = "Not Implemented";
        res.send(&stream);
        print_log(address, req, res);
        return;
    }

    let indexes = vec!["index.htm", "index.html"];
    let p = String::from(opts.root_path) + &req.get_uri();
    let mut path = PathBuf::from(p);
    if path.is_dir() {
        for index in indexes {
            if path.join(index).is_file() {
                path.push(index);
                break;
            }
        }
    } // NOTE: we could check 404 here with `else if !path.is_file()`

    if let Err(_) = read_file(path.to_str().unwrap(), &mut res.body) {
        res.status_code = 404;
        res.status_message = "Not Found";
        res.send(&stream);
        print_log(address, req, res);
        return;
    }

    res.set_header("content-type", "text/html; charset=utf-8");

    if req.method == "HEAD" {
        res.send_head(&stream);
    } else {
        res.send(&stream);
    }

    print_log(address, req, res);
}

fn read_file(path: &str, buf: &mut Vec<u8>) -> Result<(), String> {
    match File::open(path) {
        Err(_) => return Err("Could not parse request line".into()),
        Ok(mut file) => {
            if let Err(_) = file.read_to_end(buf) {
                return Err("Could not parse request line".into())
            }
        }
    }
    Ok(())
}

fn print_log(address: IpAddr, req: Request, res: Response) {
    println!(
        "{} - - [{}] \"{} {} {}\" {}",
        address,
        res.date,
        req.method,
        req.uri,
        req.version,
        res.status_code
    );
}

fn exit_on_error(e: std::io::Error) -> ! {
    let mut stderr = std::io::stderr();
    writeln!(&mut stderr, "Error: {}", e.description()).unwrap();
    exit(1);
}
