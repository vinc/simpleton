extern crate time;

use std::collections::HashMap;
use std::io::prelude::*;
use std::net::TcpStream;

#[derive(Clone)]
pub struct Response<'a> {
    pub status_code: u16,
    pub status_message: &'a str,
    pub date: String,
    pub head: Vec<u8>,
    pub body: Vec<u8>,
    pub size: usize,
    headers: HashMap<String, String>
}

impl<'a> Response<'a> {
    pub fn new() -> Response<'a> {
        let time = time::now();
        let date = time::strftime("%a, %d %b %y %T %Z", &time).unwrap();

        Response {
            status_code: 200,
            status_message: "Ok",
            date: date,
            head: Vec::new(),
            body: Vec::new(),
            size: 0,
            headers: HashMap::new()
        }
    }

    pub fn get_header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }

    pub fn set_header(&mut self, name: &str, value: &str) {
        self.headers.insert(name.into(), value.into());
    }

    pub fn send(&mut self, mut stream: &TcpStream) {
        // Status line
        let version = "HTTP/1.1";
        let code = self.status_code;
        let message = self.status_message;
        let line = format!("{} {} {}\n", version, code, message);
        self.head.extend(line.as_bytes().iter().cloned());

        // Headers
        let content_length = self.body.len().to_string();
        let date = self.date.clone();
        self.set_header("server", "SimpletonHTTP/0.0.0");
        self.set_header("content-length", &content_length);
        self.set_header("date", &date);
        for (name, value) in &self.headers {
            let line = format!("{}: {}\n", name, value);
            self.head.extend(line.as_bytes().iter().cloned());
        }

        let _ = stream.write(&self.head);
        let _ = stream.write(b"\n");
        let _ = stream.write(&self.body);
    }
}
