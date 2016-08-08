use std::net::TcpStream;

use http::request::Request;
use http::response::Response;

pub fn handler(req: Request, res: Response, stream: TcpStream) {
    let address = match stream.peer_addr() {
        Err(_)        => return,
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
}
