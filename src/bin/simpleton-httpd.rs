extern crate simpleton;

use simpleton::http;

fn main() {
    let mut server = http::Server::new();
    server.configure_from_args(std::env::args().collect()); // TODO: move that back here
    server.add_handler(http::handlers::serve_static::handler);
    server.add_handler(http::handlers::print_log::handler);

    println!("{}", server.name);
    println!("Listening on {}:{}", server.address, server.port);

    server.listen();
}
