use std::collections::HashMap;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str;
use std::thread;

use http::request::Request;
use http::response::Response;

/// HTTP server
#[derive(Clone)]
pub struct Server {
    pub name: String,
    pub handlers: Vec<fn(Request, Response) -> Response>,

    // TODO: All of that could be in `serve_static` handler
    pub root_path: String,
    pub allow_trace: bool,
    pub directory_indexes: Vec<String>,
    pub content_types: HashMap<String, String>
}

impl Server {
    pub fn new() -> Server {
        let mut content_types = HashMap::new();
        content_types.insert("html".into(), "text/html".into());
        content_types.insert("txt".into(),  "text/plain".into());

        Server {
            handlers: Vec::new(),
            root_path: ".".into(),
            name: "Simpleton HTTP Server".into(),
            allow_trace: false,
            directory_indexes: vec!["index.htm".into(), "index.html".into()],
            content_types: content_types
        }
    }

    pub fn add_handler(&mut self, f: fn(Request, Response) -> Response) {
        self.handlers.push(f);
    }

    pub fn listen(self, binding: &str) {
        let listener = match TcpListener::bind(binding) {
            Err(e)       => { println!("Error: {}", e); return }
            Ok(listener) => listener
        };

        for stream in listener.incoming() {
            match stream {
                Err(e)     => {
                    println!("Error: {}", e);
                    return
                },
                Ok(stream) => {
                    let server = self.clone();
                    thread::spawn(move|| {
                        handle_client(stream, server)
                    });
                }
            }
        }

        drop(listener);
    }

}

fn handle_client(stream: TcpStream, server: Server) {
    // Read raw request message
    let mut lines = vec![];
    let mut reader = BufReader::new(&stream);
    for line in reader.by_ref().lines() {
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

    // Create Request message
    let mut req = match Request::from_str(&request_message) {
        Err(_)  => return,
        Ok(req) => req
    };

    // Set the IP address of the client in Request
    let ip = match stream.peer_addr() {
        Err(_)        => return,
        Ok(peer_addr) => peer_addr.ip()
    };
    req.ip = ip.to_string();


    // Create Response message
    let mut res = Response::new(server.clone());

    // Call all handlers
    for handler in &server.handlers {
        match stream.try_clone() {
            Ok(stream) => {
                res = handler(req.clone(), res.clone());
                res.write(&stream);
            },
            Err(e) => { panic!("{}", e) }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let server = Server::new();

        assert!(server.handlers.is_empty());
    }
}
