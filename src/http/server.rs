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
    pub middlewares: Vec<fn(Request, Response, TcpStream, Server)>,
    pub root_path: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub debug: bool,
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
            middlewares: Vec::new(),
            root_path: ".".into(),
            name: "Simpleton HTTP Server".into(),
            address: "127.0.0.1".into(),
            port: 3000,
            debug: false,
            allow_trace: false,
            directory_indexes: vec!["index.htm".into(), "index.html".into()],
            content_types: content_types
        }
    }

    pub fn add_middleware(&mut self, f: fn(Request, Response, TcpStream, Server)) {
        self.middlewares.push(f);
    }

    pub fn configure_from_args(&mut self, args: Vec<String>) {
        let args: Vec<_> = args.iter().filter(|&arg| {
            if arg == "--debug" {
                self.debug = true;
            }

            if arg == "--allow-trace" {
                self.allow_trace = true;
            }

            !arg.starts_with("--")
        }).collect();

        if args.len() > 1 {
            //self.root_path = args[1]; // FIXME
        }
    }

    /*
    fn create(f: fn(Request, Response)) -> Server {
        Server {
            handle: f
        }
    }
    fn listen(self) {
        let req = Request::new("/");
        let res = Response::new();
        (self.handle)(req, res);

        let req = Request::new("/yooo");
        let res = Response::new();
        (self.handle)(req, res);
    }
    */
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
        // Read the request message
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

        let req = match Request::from_str(&request_message) {
            Err(_)  => return,
            Ok(req) => req
        };

        let res = Response::new();

        for middleware in &server.middlewares {
            let server = server.clone();
            let req = req.clone();
            let res = res.clone();
            match stream.try_clone() {
                Ok(stream) => {
                    middleware(req, res, stream, server);
                },
                Err(e) => { panic!("{}", e) }
            }
        }
    }

#[cfg(test)]
mod tests {
    use super::*;
    
    use http::request::Request;
    use http::response::Response;

    fn handle_client(req: Request, res: Response) { }

    #[test]
    fn test_new() {
        let server = Server::new(handle_client);

        assert_eq!(server.port, 3000);
    }

    #[test]
    fn test_configure_from_args() {
        let mut server = Server::new(handle_client);

        assert_eq!(server.debug, false);
        server.configure_from_args(vec!["--debug".into()]);
        assert_eq!(server.debug, true);
    }
}
