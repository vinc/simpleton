Simpleton
=========

Simpleton HTTP
--------------

This is very much a work in progress.

```rust
extern crate simpleton;

use simpleton::http;

fn main() {
    let mut server = http::Server::new();

    server.configure_from_args(std::env::args().collect());

    server.add_handler(my_handler);
    server.add_handler(http::handlers::serve_static::handler);
    server.add_handler(http::handlers::print_log::handler);

    println!("{}", server.name);
    println!("Listening on {}:{}", server.address, server.port);

    server.listen();
}

fn my_handler(req: Request, res: Response) -> Response {
    res.send("Hello, World!".as_bytes());
}
```
