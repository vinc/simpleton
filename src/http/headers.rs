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

/*
impl Iterator for Headers {
    type Item = (String, String);

    fn next(&mut self) -> Option<(String, String)> {
        self.headers.next()
    }
}
*/
