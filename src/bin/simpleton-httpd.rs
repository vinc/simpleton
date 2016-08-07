extern crate time;
extern crate simpleton;

use std::collections::HashMap;
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

    let mut lines = str::from_utf8(&buf).unwrap().lines();

    // Parse the request line
    let req_line = lines.next().unwrap();
    let req_line_fields: Vec<&str> = req_line.split_whitespace().collect();
    // TODO: Check req_lien_fields
    let req = Request {
        method:  req_line_fields[0],
        uri:     req_line_fields[1],
        version: req_line_fields[2],
        address: stream.peer_addr().unwrap().ip()
    };
    if opts.debug {
        println!("> {} {} {}", req.method, req.uri, req.version);
    }

    // Parse the headers
    let mut req_headers = HashMap::new();
    for line in lines {
        if opts.debug {
            println!("> {}", line);
        }
        let mut fields = line.splitn(2, ":");
        if let Some(field_name) = fields.next() {
            if let Some(field_value) = fields.next() {
                let name = field_name.trim();
                let value = field_value.trim();
                req_headers.insert(name, value);
            }
        }
        if line == "" {
            break; // End of headers
        }
    }

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
                match file.read_to_end(&mut res.body) {
                    Err(_) => {
                        if opts.debug {
                            println!("ERROR: could not read file");
                        }
                        res.status_code = 404;
                        res.status_message = "Not Found";
                    }
                    Ok(_) => { }
                }
            }
        }
    }

    res.set_header("content-type", "text/html; charset=utf-8");
    res.send(&stream);

    print_log(req, res);
}

fn print_log(req: Request, res: Response) {
    println!(
        "{} - - [{}] \"{} {} {}\" {}",
        req.address,
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
