extern crate getopts;
extern crate simpleton;

use std::env;

use getopts::Options;
use simpleton::http;

fn print_usage(program: &str, opts: Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let mut opts = Options::new();
    opts.optopt("a", "address", "Bind to HOST address (default: 0.0.0.0)", "HOST");
    opts.optopt("p", "port", "Use PORT (default: 3000)", "HOST");
    opts.optflag("h", "help", "Show this message");
    let matches = match opts.parse(&args[1..]) {
        Ok(m)  => { m }
        Err(_) => { return print_usage(&program, opts); }
    };

    if matches.opt_present("h") {
        return print_usage(&program, opts);
    }

    let address = matches.opt_str("a").unwrap_or("0.0.0.0".into());
    let port = matches.opt_str("p").unwrap_or("3000".into());

    let binding = vec![address, port].join(":");

    let mut server = http::Server::new();

    server.add_handler(http::handlers::serve_static::handler);
    server.add_handler(http::handlers::print_log::handler);

    println!("{}", server.name);
    println!("Listening on {}", binding);

    server.listen(&binding);
}
