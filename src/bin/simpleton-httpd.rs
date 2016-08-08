extern crate time;
extern crate simpleton;

use std::fs::File;
use std::io::prelude::*;
use std::net::IpAddr;
use std::net::TcpStream;
use std::path::PathBuf;
use std::str;

use simpleton::http::{Server, Request, Response};

fn main() {
    let mut server = Server::new(handle_connection);

    server.configure_from_args(std::env::args().collect());

    println!("{}", server.name);
    println!("Listening on {}:{}", server.address, server.port);

    server.listen();
}

fn handle_connection(req: Request, mut res: Response, stream: TcpStream, server: Server) {
    let address = match stream.peer_addr() {
        Err(_)        => return,
        Ok(peer_addr) => peer_addr.ip()
    };

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
        res.body = req.to_string().as_bytes().to_vec();
        res.send(&stream);
        print_log(address, req, res);
        return;
    }

    // Build local file path from URI
    let req_path = String::from(server.root_path) + &req.canonicalized_uri();
    let mut path = PathBuf::from(&req_path);

    if path.is_dir() {
        // Trailing slash redirect
        if !req.uri.ends_with("/") {
            let redirect_uri = req.uri.clone() + "/";
            res.status_code = 301;
            res.status_message = "Moved Permanently".into();
            res.headers.set("location", &redirect_uri);
            res.send(&stream);
            print_log(address, req, res);
            return;
        }

        // Directory index file
        for index in &server.directory_indexes {
            if path.join(index).is_file() {
                path.push(index);
                break;
            }
        }
    }

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
