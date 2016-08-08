use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::{Path, PathBuf, Component};

use http::headers::Headers;

/// HTTP request message
#[derive(Clone)]
pub struct Request {
    /// The Method token indicates the method to be performed on the
    /// resource identified by the Request-URI. The method is case-sensitive.
    pub method: String,

    /// The Request-URI is a Uniform Resource Identifier and identifies
    /// the resource upon which to apply the request.
    pub uri: String,

    /// HTTP Version: `HTTP/<major>.<minor>`.
    pub version: String,

    /// The Request-Header fields allow the client to pass additional
    /// information about the request, and about the client itself, to
    /// the server.
    pub headers: Headers
}

impl Request {
    /// Create an HTTP message request.
    pub fn new(method: &str, host: &str, uri: &str) -> Request {
        let user_agent = "SimpletonHTTP/0.0.0";
        let version = "HTTP/1.1";
        let mut req = Request {
            method:  method.into(),
            uri:     uri.into(),
            version: version.into(),
            headers: Headers::new()
        };
        req.headers.set("host".into(), host.into());
        req.headers.set("user-agent".into(), user_agent.into());
        req.headers.set("accept".into(), "*/*".into());

        req
    }

    /// Create a `Request` from a raw HTTP request message.
    pub fn from_str(message: &str) -> Result<Request, String> {
        let mut lines = message.lines();

        // Parse the request line
        let req_line = match lines.next() {
            None       => return Err("Could not read request line".into()),
            Some(line) => line
        };
        let req_line_fields: Vec<&str> = req_line.split_whitespace().collect();
        if req_line_fields.len() != 3 {
            return Err("Could not parse request line".into());
        }
        let mut req = Request {
            method:  req_line_fields[0].into(),
            uri:     req_line_fields[1].into(),
            version: req_line_fields[2].into(),
            headers: Headers::new()
        };

        // Parse the headers
        for line in lines {
            let mut fields = line.splitn(2, ":");
            if let Some(field_name) = fields.next() {
                if let Some(field_value) = fields.next() {
                    let name = field_name.trim();
                    let value = field_value.trim();
                    req.headers.set(name, value);
                }
            }
            if line == "" {
                break; // End of headers
            }
        }
        
        Ok(req)
    }

    /// Get the normalized URI of a `Request`.
    pub fn get_uri(&self) -> String {
        let mut components = vec![];

        // Rebuild URL to prevent path traversory attack
        for component in Path::new(&self.uri).components() {
            match component {
                Component::ParentDir => { components.pop(); },
                Component::Normal(s) => { components.push(s.to_str().unwrap()); },
                _                    => { }
            }
        }

        let mut path = PathBuf::from("/");
        for component in components {
            path.push(component);
        }
        path.to_str().unwrap().to_string()
    }

    /// Send the request to the server through a `TcpStream`.
    pub fn send(&mut self, mut stream: &TcpStream) {
        let _ = stream.write(&self.to_string().into_bytes());
    }
}

impl fmt::Display for Request {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut lines = vec![];

        // Request line
        lines.push(format!("{} {} {}", self.method, self.uri, self.version));

        // Headers
        for (name, value) in &self.headers {
            lines.push(format!("{}: {}", name, value));
        }

        // End of head
        lines.push("\n".into());

        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let req = Request::new("GET", "example.com", "/");

        assert_eq!(req.method, String::from("GET"));
        assert_eq!(req.headers.get("host"), Some(&"example.com".into()));
        assert_eq!(req.uri, String::from("/"));
    }

    #[test]
    fn test_to_string() {
        let req = Request::new("GET", "example.com", "/");

        assert!(req.to_string().starts_with("GET / HTTP/1.1\n"));
    }
}
