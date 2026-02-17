use std::collections::HashMap;

#[derive(Debug)]
struct KeyValueStore(HashMap<String, String>);

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.0.insert(key, value);
    }

    pub fn get_value(&self, key: &str) -> Option<&String> {
        self.0.get(key)
    }

    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.0.remove(key)
    }
}

fn main() {
    let mut map = KeyValueStore::new();

    map.insert("name".to_string(), "Mahesh".to_string());

    println!("{:?}", map.get_value(&"name"));

    map.remove(&"name");
}
