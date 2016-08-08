extern crate time;
extern crate simpleton;

use simpleton::http;
use simpleton::http::Server;

fn main() {
    let mut server = Server::new();
    server.configure_from_args(std::env::args().collect());
    server.add_handler(http::handlers::serve_static::handler);
    server.add_handler(http::handlers::print_log::handler);

    println!("{}", server.name);
    println!("Listening on {}:{}", server.address, server.port);

    server.listen();
}
