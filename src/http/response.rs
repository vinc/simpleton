extern crate time;

use std::io::prelude::*;
use std::net::TcpStream;

use http::headers::Headers;

/// HTTP response message
#[derive(Clone)]
pub struct Response {
    pub status_code: u16,
    pub status_message: String,
    pub date: String,
    pub body: Vec<u8>,
    pub headers: Headers,
    head_sent: bool
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
            body: Vec::new(),
            headers: Headers::new()
        }
    }

    pub fn to_string(&self) -> String {
        let mut lines = vec![];

        // Status line
        let version = "HTTP/1.1";
        let code = self.status_code;
        let message = self.status_message.clone();
        lines.push(format!("{} {} {}", version, code, message));

        // Headers
        for (name, value) in &self.headers {
            lines.push(format!("{}: {}", name, value));
        }

        // End of head
        lines.push("\n".into());

        lines.join("\n")
    }

    pub fn send_head(&mut self, mut stream: &TcpStream) {
        // Set headers
        if !self.headers.contains_key("content-length") {
            let content_length = self.body.len().to_string();
            self.headers.set("content-length", &content_length);
        }
        let date = self.date.clone();
        self.headers.set("server", "SimpletonHTTP/0.0.0");
        self.headers.set("date", &date);
        self.headers.set("connection", "close");

        // Send head
        let _ = stream.write(&self.to_string().into_bytes());
        self.head_sent = true;
    }

    pub fn send(&mut self, mut stream: &TcpStream) {
        if !self.head_sent {
            self.send_head(&stream);
        }
        let _ = stream.write(&self.body);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let res = Response::new();

        assert_eq!(res.status_code, 200);
    }

    #[test]
    fn test_to_string() {
        let res = Response::new();

        assert!(res.to_string().starts_with("HTTP/1.1 200 Ok\n"));
    }
}
