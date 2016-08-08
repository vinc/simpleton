use std::net::TcpStream;

use http::request::Request;
use http::response::Response;

/// Run after every other handlers to print a log of the request
/// and its response to stdout.
pub fn handler(req: Request, res: Response, stream: TcpStream) -> Response {
    let address = match stream.peer_addr() {
        Err(_)        => return res,
        Ok(peer_addr) => peer_addr.ip()
    };

    println!(
        "{} - - [{}] \"{} {} {}\" {} -",
        address,
        res.date,
        req.method,
        req.uri,
        req.version,
        res.status_code
    );

    res
}
