use std::borrow::ToOwned;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::path::{Path, PathBuf, Component};
use std::str;
use std::thread;
use std::env;

fn main() {
    let mut debug = false;

    for arg in env::args() {
        if arg == "--debug" {
            debug = true;
        }
    }

    let address = "127.0.0.1:3000";
    let listener = TcpListener::bind(address).unwrap();
    println!("Simpleton HTTP Server is listening on {}\n", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream, debug)
                });
            }
            Err(e) => {
                // HTTP Connexion failed
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream, debug: bool) {
    if debug {
        println!("");
    }
    
    let req_address = stream.peer_addr().unwrap().ip();
    let mut buf = [0; 256];
    let _ = stream.read(&mut buf);

    let mut lines = str::from_utf8(&buf).unwrap().lines();

    let req_line = lines.next().unwrap();
    //println!("> {}", req_line);
    let req_line_fields: Vec<&str> = req_line.split_whitespace().collect();
    let req_method  = req_line_fields[0];
    let req_uri     = req_line_fields[1];
    let req_version = req_line_fields[2];

    if debug {
        println!("> {} {} {}", req_method, req_uri, req_version);
        for line in lines {
            println!("> {}", line);
        }
    }

    let mut res_status_code = 200;
    let mut res_reason_phrase = "Ok";
    let mut res_body = vec![0; 10];
    let mut res_size = 0;

    // Prevent path traversory attack
    let mut components: Vec<&str> = vec![];
    for component in Path::new(req_uri).components() {
        match component {
            Component::ParentDir => { components.pop(); },
            Component::Normal(s) => { components.push(s.to_str().unwrap()); },
            _                    => { }
        }
    }
    let mut path = PathBuf::from("./");
    for component in components {
        path.push(component); 
    }
    if debug {
        println!("DEBUG: path => {:?}", path);
    }

    if req_method == "GET" {
        match File::open(path) {
            Err(_) => {
                if debug {
                    println!("ERROR: could not open file");
                }
                res_status_code = 404;
                res_reason_phrase = "Not Found";
            },
            Ok(mut file) => {
                match file.read_to_end(&mut res_body) {
                    Err(_) => {
                        if debug {
                            println!("ERROR: could not read file");
                        }
                        res_status_code = 404;
                        res_reason_phrase = "Not Found";
                    }
                    Ok(n) => {
                        res_size = n; // FIXME
                    }
                }
            }
        }
    }

    let mut lines = vec![];
    lines.push(format!("HTTP/1.0 {} {}\n", res_status_code, res_reason_phrase));
    lines.push(format!("Server: SimpletonHTTP/0.0.0\n"));
    //lines.push(format!("Content-Type: text/html; charset=utf-8\n"));
    lines.push(format!("Content-Length: {}\n", res_size));
    for line in lines {
        let _ = stream.write(line.as_bytes());
    }
    let _ = stream.write(b"\n");
    let _ = stream.write(&res_body);
    println!("{} - - \"{} {} {}\" {}", req_address, req_method, req_uri, req_version, res_status_code);
}
