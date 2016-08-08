extern crate time;
extern crate simpleton;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::IpAddr;
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use std::str;
use std::thread;

use simpleton::http::{Server, Request, Response};

fn main() {
    let mut server = Server::new();

    server.configure_from_args(std::env::args().collect());

    println!("{}", server.name);

    let binding = (server.address.as_str(), server.port);
    let listener = match TcpListener::bind(binding) {
        Err(e)       => { println!("Error: {}", e); return }
        Ok(listener) => listener
    };
    println!("Listening on {}:{}", server.address, server.port);

    for stream in listener.incoming() {
        match stream {
            Err(e)     => {
                println!("Error: {}", e);
                return
            },
            Ok(stream) => {
                let server = server.clone();
                thread::spawn(move|| {
                    handle_client(stream, server)
                });
            }
        }
    }

    drop(listener);
}

fn handle_client(stream: TcpStream, server: Server) {
    let address = match stream.peer_addr() {
        Err(_)        => return,
        Ok(peer_addr) => peer_addr.ip()
    };

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
    let request_message = lines.join("\n");

    let req = match Request::from_str(&request_message) {
        Err(_)  => return,
        Ok(req) => req
    };

    let mut res = Response::new();

    // Check HTTP method
    let mut methods = vec!["GET", "HEAD"];
    if server.allow_trace {
        methods.push("TRACE");
    }
    if let None = methods.iter().find(|&&method| method == req.method) {
        res.status_code = 501;
        res.status_message = "Not Implemented".into();
        res.send(&stream);
        print_log(address, req, res);
        return;
    }

    if req.method == "TRACE" {
        // The TRACE method is used to invoke a remote, application-layer
        // loop-back of the request message. The final recipient of the
        // request SHOULD reflect the message received back to the client
        // as the entity-body of a 200 (OK) response.
        //
        // If the request is valid, the response SHOULD contain the entire
        // request message in the entity-body, with a Content-Type of
        // "message/http".
        //
        // (RFC 2616 9.8)
        res.headers.set("content-type", "message/http");
        res.body = request_message.as_bytes().to_vec();
        res.send(&stream);
        print_log(address, req, res);
        return;
    }

    // Build local file path from URI
    let p = String::from(server.root_path) + &req.get_uri();
    let mut path = PathBuf::from(p);

    // Look for directory index file if requested
    if path.is_dir() {
        for index in &server.directory_indexes {
            if path.join(index).is_file() {
                path.push(index);
                break;
            }
        }
    } // NOTE: we could check 404 here with `else if !path.is_file()`

    // Set content-type header based on file extension
    if let Some(extension) = path.extension() {
        let extension = extension.to_str().unwrap();
        if let Some(content_type) = server.content_types.get(extension) {
             res.headers.set("content-type", content_type);
        }
    }

    // Read file
    if let Err(_) = read_file(path.to_str().unwrap(), &mut res.body) {
        res.status_code = 404;
        res.status_message = "Not Found".into();
        res.send(&stream);
        print_log(address, req, res);
        return;
    }

    if req.method == "HEAD" {
        // The HEAD method is identical to GET except that the server MUST NOT
        // return a message-body in the response.
        //
        // (RFC 2616 9.4)
        res.send_head(&stream);
    } else { // GET method
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
        "{} - - [{}] \"{} {} {}\" {} -",
        address,
        res.date,
        req.method,
        req.uri,
        req.version,
        res.status_code
    );
}
