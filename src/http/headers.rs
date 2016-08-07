/*
 * NOTE: headers are stored in a BTreeMap. It would be faster to use a HashMap
 * instead but the order in which they are displayed would become
 * non-deterministic.
 */
use std::collections::BTreeMap;
use std::collections::btree_map::IntoIter;

#[derive(Clone)]
pub struct Headers {
    headers: BTreeMap<String, String>
}

impl Headers {
    pub fn new() -> Headers {
        Headers {
            headers: BTreeMap::new()
        }
    }

    pub fn get(&self, name: &str) -> Option<&String> {
        self.headers.get(&name.to_lowercase())
    }

    pub fn set(&mut self, name: &str, value: &str) {
        self.headers.insert(name.to_lowercase(), value.into());
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.headers.contains_key(&name.to_lowercase())
    }
}

impl<'a> IntoIterator for &'a Headers {
    type Item = (String, String);
    type IntoIter = IntoIter<String, String>;

    fn into_iter(self) -> Self::IntoIter {
        self.headers.clone().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set() {
        let mut headers = Headers::new();

        headers.set("content-type".into(), "text/html".into());
        assert_eq!(headers.get("content-type"), Some(&"text/html".into()));

        // The name of the header is case insensitive
        headers.set("Content-Type".into(), "text/plain".into());
        assert_eq!(headers.get("content-type"), Some(&"text/plain".into()));
    }

    #[test]
    fn test_get() {
        let mut headers = Headers::new();

        assert_eq!(headers.get("not-set"), None);

        headers.set("content-type".into(), "text/html".into());
        assert_eq!(headers.get("content-type"), Some(&"text/html".into()));

        // The name of the header is case insensitive
        assert_eq!(headers.get("Content-Type"), Some(&"text/html".into()));
    }
}
