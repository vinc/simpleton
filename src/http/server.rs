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
    pub address: String,
    pub port: u16,
    pub handlers: Vec<fn(Request, Response, TcpStream) -> Response>,

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
            address: "127.0.0.1".into(),
            port: 3000,
            allow_trace: false,
            directory_indexes: vec!["index.htm".into(), "index.html".into()],
            content_types: content_types
        }
    }

    pub fn add_handler(&mut self, f: fn(Request, Response, TcpStream) -> Response) {
        self.handlers.push(f);
    }

    pub fn configure_from_args(&mut self, args: Vec<String>) {
        let args: Vec<_> = args.iter().filter(|&arg| {
            if arg == "--allow-trace" {
                self.allow_trace = true;
            }

            !arg.starts_with("--")
        }).collect();

        if args.len() > 1 {
            //self.root_path = args[1]; // FIXME
        }
    }

    pub fn listen(self) {
        let binding = (self.address.as_str(), self.port);

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
    let req = match Request::from_str(&request_message) {
        Err(_)  => return,
        Ok(req) => req
    };

    // Create Response message
    let mut res = Response::new(server.clone());

    // Call all handlers
    for handler in &server.handlers {
        match stream.try_clone() {
            Ok(stream) => {
                res = handler(req.clone(), res.clone(), stream);
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

        assert_eq!(server.port, 3000);
    }

    #[test]
    fn test_configure_from_args() {
        let mut server = Server::new();

        assert_eq!(server.allow_trace, false);
        server.configure_from_args(vec!["--allow-trace".into()]);
        assert_eq!(server.allow_trace, true);
    }
}
