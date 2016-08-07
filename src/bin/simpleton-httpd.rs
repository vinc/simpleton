extern crate time;
extern crate simpleton;

use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
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

fn handle_client(mut stream: TcpStream, opts: Options) {
    if opts.debug {
        println!("");
    }
    
    //let mut buf: Vec<u8> = vec![];
    //let _ = stream.read_to_end(&mut buf);
    let mut buf = [0; 256];
    let _ = stream.read(&mut buf);

    let req = match Request::from_str(str::from_utf8(&buf).unwrap()) {
        Err(_)  => return,
        Ok(req) => req
    };

    let mut res = Response::new();

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
    } else if !path.is_file() {
        // TODO: 404
    }
    if opts.debug {
        println!("DEBUG: path = {:?}", path);
    }

    if req.method == "GET" {
        match File::open(path) {
            Err(_) => {
                if opts.debug {
                    println!("ERROR: could not open file");
                }
                res.status_code = 404;
                res.status_message = "Not Found";
            },
            Ok(mut file) => {
                if let Err(_) = file.read_to_end(&mut res.body) {
                    if opts.debug {
                        println!("ERROR: could not read file");
                    }
                    res.status_code = 404;
                    res.status_message = "Not Found";
                }
            }
        }
    }

    res.set_header("content-type", "text/html; charset=utf-8");
    res.send(&stream);

    print_log(req, res, &stream);
}

fn print_log(req: Request, res: Response, stream: &TcpStream) {
    let address = stream.peer_addr().unwrap().ip();

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
