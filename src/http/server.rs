use std::collections::HashMap;

#[derive(Clone)]
pub struct Server {
    pub root_path: String,
    pub name: String,
    pub address: String,
    pub port: u16,
    pub debug: bool,
    pub allow_trace: bool,
    pub directory_indexes: Vec<String>,
    pub content_types: HashMap<String, String>
}

impl Server {
    pub fn new() -> Server {
        let mut server = Server {
            root_path: ".".into(),
            name: "Simpleton HTTP Server".into(),
            address: "127.0.0.1".into(),
            port: 3000,
            debug: false,
            allow_trace: false,
            directory_indexes: vec!["index.htm".into(), "index.html".into()],
            content_types: HashMap::new()
        };
        server.content_types.insert("html".into(), "text/html".into());
        server.content_types.insert("txt".into(),  "text/plain".into());

        server
    }

    pub fn configure_from_args(&mut self, args: Vec<String>) {
        let args: Vec<_> = args.iter().filter(|&arg| {
            if arg == "--debug" {
                self.debug = true;
            }

            if arg == "--allow-trace" {
                self.allow_trace = true;
            }

            !arg.starts_with("--")
        }).collect();

        if args.len() > 1 {
            //self.root_path = args[1]; // FIXME
        }
    }
}