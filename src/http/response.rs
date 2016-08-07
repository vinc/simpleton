extern crate time;

use std::io::prelude::*;
use std::net::TcpStream;

use http::headers::Headers;

#[derive(Clone)]
pub struct Response {
    pub status_code: u16,
    pub status_message: String,
    pub date: String,
    pub body: Vec<u8>,
    pub headers: Headers,
    head_sent: bool,
    head: Vec<u8>
}

impl Response {
    pub fn new() -> Response {
        let time = time::now();
        let date = time::strftime("%a, %d %b %y %T %Z", &time).unwrap();

        Response {
            status_code: 200,
            status_message: "Ok".into(),
            date: date,
            head_sent: false,
            head: Vec::new(),
            body: Vec::new(),
            headers: Headers::new()
        }
    }

    pub fn send_head(&mut self, mut stream: &TcpStream) {
        // Status line
        let version = "HTTP/1.1";
        let code = self.status_code;
        let message = self.status_message.clone();
        let line = format!("{} {} {}\n", version, code, message);
        self.head.extend(line.as_bytes().iter().cloned());

        // Headers
        if !self.headers.contains_key("content-length") {
            let content_length = self.body.len().to_string();
            self.headers.set("content-length", &content_length);
        }
        let date = self.date.clone();
        self.headers.set("server", "SimpletonHTTP/0.0.0");
        self.headers.set("date", &date);
        self.headers.set("connection", "close");
        for (name, value) in &self.headers {
            let line = format!("{}: {}\n", name, value);
            self.head.extend(line.as_bytes().iter().cloned());
        }

        let _ = stream.write(&self.head);
        let _ = stream.write(b"\n");

        self.head_sent = true;
    }

    pub fn send(&mut self, mut stream: &TcpStream) {
        if !self.head_sent {
            self.send_head(&stream);
        }
        let _ = stream.write(&self.body);
    }
}
