use std::path::{Path, PathBuf, Component};

#[derive(Copy, Clone)]
pub struct Request<'a> {
    pub method: &'a str,
    pub uri: &'a str,
    pub version: &'a str
}

impl<'a> Request<'a> {
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
