use std::{collections::HashMap, hash::Hash, io::stdin};

#[derive(Debug, Hash, PartialEq, Clone)]
enum MapValue {
    String(String),
    Integer(i64),
    List(Vec<String>),
    Nil,
}

#[derive(Debug)]
struct KeyValueStore(HashMap<String, MapValue>);

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: MapValue) {
        let res = self.0.insert(key, value);
        match res {
            Some(val) => println!("UPDATED; old value: {:?}", val),
            None => println!("INSERTED"),
        }
    }

    pub fn get_value(&self, key: &str) -> &MapValue {
        let res = self.0.get(key);
        match res {
            Some(val) => val,
            None => &MapValue::Nil,
        }
    }

    pub fn get_type(&self, key: &str) -> &'static str {
        match self.get_value(key) {
            MapValue::Integer(_) => "Integer",
            MapValue::String(_) => "String",
            MapValue::List(_) => "List",
            MapValue::Nil => "Nil",
        }
    }

    pub fn incr_val(&mut self, key: &str) {
        self.0
            .entry(key.to_string())
            .and_modify(|f| {
                if let MapValue::Integer(v) = f {
                    *v += 1;
                }
            })
            .or_insert(MapValue::Integer(1));
    }

    pub fn lpush(&mut self, key: String, value: String) {
        self.0
            .entry(key)
            .and_modify(|mv| {
                if let MapValue::List(ls) = mv {
                    ls.insert(0, value.clone());
                }
            })
            .or_insert(MapValue::List(vec![value.clone()]));
    }

    pub fn lrange(&self, key: &str, start: usize, stop: usize) {
        match self.get_value(key) {
            MapValue::List(ls) => {
                println!("{:?}", ls.get(start..=stop));
            },
            MapValue::Integer(_) | MapValue::String(_) => {
                eprintln!("Value not subscriptable");
            },
            MapValue::Nil => {
                eprintln!("Key not found");
            }
        };
    }

    pub fn remove(&mut self, key: &str) {
        let res = self.0.remove(key);
        match res {
            Some(val) => println!("REMOVED; value: {:?}", val),
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

                map.insert(
                    parts[1].to_string(),
                    match parts[2].parse::<i64>() {
                        Ok(val) => MapValue::Integer(val),
                        Err(_) => MapValue::String(parts[2].trim().to_string()),
                    },
                );
            }
            "INCR" => {
                if parts.len() != 2 {
                    eprintln!("ERROR: provide exactly 2 args; INCR key");
                    continue;
                }
                match map.get_type(parts[1]) {
                    "Integer" | "Nil" => {
                        map.incr_val(parts[1]);
                    }
                    _other_types => {
                        eprintln!("{} not supported for Increment", _other_types)
                    }
                }
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
                println!("{:?}", map.get_value(parts[1]));
            }
            "DIS" => println!("{:?}", map),
            "LPUSH" => {
                if parts.len() != 3 {
                    eprintln!("ERROR: provide exactly 3 args; LPUSH key item");
                    continue;
                }

                map.lpush(parts[1].to_string(), parts[2].to_string());
            }
            "LRANGE" => {
                if parts.len() != 4 {
                    eprintln!("ERROR: provide exactly 4 args; LRANGE key start stop");
                    continue;
                }

                let start = match parts[2].parse::<usize>() {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        continue;
                    }
                };

                let stop = match parts[3].parse::<usize>() {
                    Ok(s) => s,
                    Err(err) => {
                        eprintln!("Error: {}", err);
                        continue;
                    }
                };
                
                map.lrange(parts[1], start, stop)
                
            }
            "TYPE" => {
                if parts.len() != 2 {
                    eprintln!("ERROR: provide exactly 2 args; TYPE key");
                    continue;
                }
                println!("{:?}", map.get_type(parts[1]));
            }
            "EXIT" => break,
            _ => {
                eprintln!("ERROR: invalid operation; valid operations SET, GET, DEL, DIS and EXIT")
            }
        }

        inp.clear();
    }
}
