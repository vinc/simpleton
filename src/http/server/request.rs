use std::collections::BTreeMap;
use std::path::{Path, PathBuf, Component};

#[derive(Copy, Clone)]
pub struct Request<'a> {
    pub method: &'a str,
    pub uri: &'a str,
    pub version: &'a str
}

impl<'a> Request<'a> {
    pub fn from_str(message: &'a str) -> Result<Request, String> {
        let mut lines = message.lines();

        // Parse the request line
        let req_line = lines.next().unwrap();
        let req_line_fields: Vec<&str> = req_line.split_whitespace().collect();
        if req_line_fields.len() != 3 {
            return Err("Could not parse request line".into());
        }
        let req = Request {
            method:  req_line_fields[0],
            uri:     req_line_fields[1],
            version: req_line_fields[2],
        };

        // Parse the headers
        let mut req_headers = BTreeMap::new();
        for line in lines {
            let mut fields = line.splitn(2, ":");
            if let Some(field_name) = fields.next() {
                if let Some(field_value) = fields.next() {
                    let name = field_name.trim();
                    let value = field_value.trim();
                    req_headers.insert(name, value);
                }
            }
            if line == "" {
                break; // End of headers
            }
        }
        
        Ok(req)
    }
    pub fn get_uri(&self) -> String {
        let mut components = vec![];

        // Rebuild URL to prevent path traversory attack
        for component in Path::new(self.uri).components() {
            match component {
                Component::ParentDir => { components.pop(); },
                Component::Normal(s) => { components.push(s.to_str().unwrap()); },
                _                    => { }
            }
        }

        let mut path = PathBuf::from("/");
        for component in components {
            path.push(component);
        }
        path.to_str().unwrap().to_string()
    }
}
