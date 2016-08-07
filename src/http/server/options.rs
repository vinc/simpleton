#[derive(Clone)]
pub struct Options<'a> {
    pub root_path: &'a str,
    pub name: &'a str,
    pub address: &'a str,
    pub port: u16,
    pub debug: bool,
    pub allow_trace: bool,
    pub directory_indexes: Vec<String>
}

impl<'a> Options<'a> {
    pub fn from_args(args: Vec<String>) -> Options<'a> {
        let mut options = Options {
            root_path: ".",
            name: "Simpleton HTTP Server",
            address: "127.0.0.1",
            port: 3000,
            debug: false,
            allow_trace: false,
            directory_indexes: vec!["index.htm".into(), "index.html".into()]
        };

        let args: Vec<_> = args.iter().filter(|&arg| {
            if arg == "--debug" {
                options.debug = true;
            }

            if arg == "--allow-trace" {
                options.allow_trace = true;
            }

            !arg.starts_with("--")
        }).collect();

        if args.len() > 1 {
            //options.root_path = args[1]; // FIXME
        }

        options
    }
}
