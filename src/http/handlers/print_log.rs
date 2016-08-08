use std::net::TcpStream;

use http::request::Request;
use http::response::Response;
use http::server::Server;

pub fn handler(req: Request, mut res: Response, stream: TcpStream, server: Server) {
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
