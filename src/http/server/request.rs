use std::collections::BTreeMap;
use std::io::prelude::*;
use std::net::TcpStream;
use std::path::{Path, PathBuf, Component};

#[derive(Clone)]
pub struct Request {
    pub method: String,
    pub uri: String,
    pub version: String,
    head: Vec<u8>,
    headers: BTreeMap<String, String>
}

impl Request {
    pub fn new(method: &str, host: &str, uri: &str) -> Request {
        let user_agent = "SimpletonHTTP/0.0.0";
        let version = "HTTP/1.1";
        let mut req = Request {
            method:  method.into(),
            uri:     uri.into(),
            version: version.into(),
            head: Vec::new(),
            headers: BTreeMap::new()
        };
        req.set_header("host".into(), host.into());
        req.set_header("user-agent".into(), user_agent.into());
        req.set_header("accept".into(), "*/*".into());

        req
    }

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
            head: Vec::new(),
            headers: BTreeMap::new()
        };

        // Parse the headers
        for line in lines {
            let mut fields = line.splitn(2, ":");
            if let Some(field_name) = fields.next() {
                if let Some(field_value) = fields.next() {
                    let name = field_name.trim();
                    let value = field_value.trim();
                    req.set_header(name, value);
                }
            }
            if line == "" {
                break; // End of headers
            }
        }
        
        Ok(req)
    }

    // TODO: this code is duplicated in `response.rs`
    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.into());
    }

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

    pub fn send(&mut self, mut stream: &TcpStream) {
        let uri = self.uri.clone();
        let method = self.method.clone();
        let version = self.version.clone();
        let line = format!("{} {} {}\n", method, uri, version);
        self.head.extend(line.as_bytes().iter().cloned());

        for (name, value) in &self.headers {
            let line = format!("{}: {}\n", name, value);
            self.head.extend(line.as_bytes().iter().cloned());
        }
        self.head.push(b'\n');

        let _ = stream.write(&self.head);
    }

    pub fn to_string(&self) -> String {
        let head = self.head.clone();
        String::from_utf8(head).unwrap()
    }
}
