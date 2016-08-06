use std::collections::HashMap;
use std::borrow::ToOwned;
use std::fs::File;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream, IpAddr};
use std::path::{Path, PathBuf, Component};
use std::str;
use std::thread;
use std::env;

#[derive(Copy, Clone)]
struct ServerOptions<'a> {
    name: &'a str,
    address: &'a str,
    port: u16,
    debug: bool
}

#[derive(Copy, Clone)]
struct Request<'a> {
    address: IpAddr,
    method: &'a str,
    uri: &'a str,
    version: &'a str
}

#[derive(Clone)]
struct Response<'a> {
    status_code: u16,
    reason_phrase: &'a str,
    body: Vec<u8>,
    size: usize
}

fn main() {
    let mut opts = ServerOptions {
        name: "Simpleton HTTP Server",
        address: "127.0.0.1",
        port: 3000,
        debug: false
    };

    for arg in env::args() {
        if arg == "--debug" {
            opts.debug = true;
        }
    }

    let listener = TcpListener::bind((opts.address, opts.port)).unwrap();
    println!("{} is listening on {}:{}\n", opts.name, opts.address, opts.port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream, opts)
                });
            }
            Err(e) => {
                // HTTP Connexion failed
            }
        }
    }

    drop(listener);
}

fn handle_client(mut stream: TcpStream, opts: ServerOptions) {
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
    let mut req = Request {
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

    let mut res = Response {
        status_code: 200,
        reason_phrase: "Ok",
        body: vec![],
        size: 0
    };

    // Prevent path traversory attack
    let mut components: Vec<&str> = vec![];
    for component in Path::new(req.uri).components() {
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
    if opts.debug {
        println!("DEBUG: path => {:?}", path);
    }

    if req.method == "GET" {
        match File::open(path) {
            Err(_) => {
                if opts.debug {
                    println!("ERROR: could not open file");
                }
                res.status_code = 404;
                res.reason_phrase = "Not Found";
            },
            Ok(mut file) => {
                match file.read_to_end(&mut res.body) {
                    Err(_) => {
                        if opts.debug {
                            println!("ERROR: could not read file");
                        }
                        res.status_code = 404;
                        res.reason_phrase = "Not Found";
                    }
                    Ok(n) => {
                        res.size = n; // FIXME
                    }
                }
            }
        }
    }

    let mut lines = vec![];
    lines.push(format!("HTTP/1.0 {} {}\n", res.status_code, res.reason_phrase));
    lines.push(format!("Server: SimpletonHTTP/0.0.0\n"));
    //lines.push(format!("Content-Type: text/html; charset=utf-8\n"));
    //lines.push(format!("Content-Length: {}\n", res.size));
    for line in lines {
        let _ = stream.write(line.as_bytes());
    }
    let _ = stream.write(b"\n");
    let _ = stream.write(&res.body);
    println!("{} - - \"{} {} {}\" {}", req.address, req.method, req.uri, req.version, res.status_code);
}
