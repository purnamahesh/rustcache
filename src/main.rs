use std::{collections::HashMap, io::stdin};

#[derive(Debug)]
struct KeyValueStore(HashMap<String, String>);

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: String) {
        let res = self.0.insert(key, value);
        match res {
            Some(val) => println!("UPDATED; old value: {}", val),
            None => println!("INSERTED"),
        }
    }

    pub fn get_value(&self, key: &str) {
        let res = self.0.get(key);
        match res {
            Some(val) => println!("FETCHED: {}", val),
            None => eprintln!("KEY NOT FOUND"),
        }
    }

    pub fn remove(&mut self, key: &str) {
        let res = self.0.remove(key);
        match res {
            Some(val) => println!("REMOVED; value: {}", val),
            None => eprintln!("KEY NOT FOUND"),
        }
    }
}

fn main() {
    let mut map = KeyValueStore::new();

    let mut inp = String::new();

    loop {
        stdin().read_line(&mut inp).unwrap_or_else(|err| {
            eprintln!("Error: {:?}", err);
            0_usize
        });

        let parts: Vec<&str> = inp.trim().split(" ").filter(|x| !x.is_empty()).collect();

        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "SET" => {
                if parts.len() != 3 {
                    eprintln!("ERROR: provide exactly 3 args; SET key val");
                    continue;
                }

                // map.insert(*&parts[0].to_string(), *&parts[1].to_string()); // cannot move out of shared referance
                map.insert(parts[1].to_string(), parts[2].to_string());
            }
            "DEL" => {
                if parts.len() != 2 {
                    eprintln!("ERROR: provide exactly 2 args; DEL key");
                    continue;
                }
                map.remove(parts[1]);
            }
            "GET" => {
                if parts.len() != 2 {
                    eprintln!("ERROR: provide exactly 2 args; GET key");
                    continue;
                }
                map.get_value(parts[1]);
            }
            "DIS" => {
                println!("{:?}", map);
            }
            _ => {
                eprintln!("ERROR: invalid operation; valid operations SET, GET, DEL and DIS")
            }
        }

        inp.clear();
    }
}
