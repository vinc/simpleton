extern crate time;

use std::fmt;
use std::io::prelude::*;
use std::net::TcpStream;

use http::headers::Headers;
use http::server::Server;

/// HTTP response message
#[derive(Clone)]
pub struct Response {
    /// The Status-Code element is a 3-digit integer result code of the
    /// attempt to understand and satisfy the request.
    pub status_code: u16,

    /// The Reason-Phrase is intended to give a short
    /// textual description of the Status-Code.
    pub status_message: String,

    /// HTTP/1.1 clients and servers MUST only generate the RFC 1123
    /// format for representing HTTP-date values in header fields.
    pub date: String,

    /// The message-body (if any) of an HTTP message is used to carry the
    /// entity-body associated with the request or response.
    pub body: Vec<u8>,

    /// The response-header fields allow the server to pass additional
    /// information about the response which cannot be placed in the
    /// Status-Line. These header fields give information about the server
    /// and about further access to the resource identified by the Request-URI.
    pub headers: Headers,

    /// Boolean indicating if the message head (status-line + headers) has
    /// been sent.
    head_sent: bool,

    pub server: Server
}

impl Response {
    /// Create an HTTP message response.
    pub fn new(server: Server) -> Response {
        let time = time::now();
        let date = time::strftime("%a, %d %b %y %T %Z", &time).unwrap();

        Response {
            status_code: 200,
            status_message: "Ok".into(),
            date: date,
            head_sent: false,
            body: Vec::new(),
            headers: Headers::new(),
            server: server
        }
    }

    /// Send the status-line and the headers of the response to the client
    /// through a `TcpStream`.
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

    /// Send the response to the client through a `TcpStream`.
    /// 
    /// This method will first send the status-line and the headers if they
    /// have not been already sent, then it will send the message body.
    pub fn send(&mut self, mut stream: &TcpStream) {
        if !self.head_sent {
            self.send_head(&stream);
        }
        let _ = stream.write(&self.body);
    }
}

impl fmt::Display for Response {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

        write!(f, "{}", lines.join("\n"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use http::server::Server;

    #[test]
    fn test_new() {
        let server = Server::new();
        let res = Response::new(server);

        assert_eq!(res.status_code, 200);
    }

    #[test]
    fn test_to_string() {
        let server = Server::new();
        let res = Response::new(server);

        assert!(res.to_string().starts_with("HTTP/1.1 200 Ok\n"));
    }
}
