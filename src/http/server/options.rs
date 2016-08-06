#[derive(Copy, Clone)]
pub struct Options<'a> {
    pub root_path: &'a str,
    pub name: &'a str,
    pub address: &'a str,
    pub port: u16,
    pub debug: bool
}

impl<'a> Options<'a> {
    pub fn from_args(args: Vec<String>) -> Options<'a> {
        let mut opts = Options {
            root_path: ".",
            name: "Simpleton HTTP Server",
            address: "127.0.0.1",
            port: 3000,
            debug: false
        };

        let args: Vec<_> = args.iter().filter(|&arg| {
            if arg == "--debug" {
                opts.debug = true;
            }

            !arg.starts_with("--")
        }).collect();

        if args.len() > 1 {
            //opts.root_path = args[1]; // FIXME
        }

        opts
    }
}
