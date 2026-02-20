use std::collections::HashMap;

use chrono::Utc;

#[derive(Debug, Hash, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    List(Vec<String>),
    Nil,
}

#[derive(Debug, Hash, PartialEq, Clone)]
pub struct MapValue {
    value: Value,
    ttl: Option<i64>,
}

#[derive(Debug)]
pub struct KeyValueStore(HashMap<String, MapValue>);

impl KeyValueStore {
    pub fn new() -> Self {
        KeyValueStore(HashMap::new())
    }

    pub fn insert(&mut self, key: String, value: Value, ttl_secs: Option<i64>) {
        let res = self.0.insert(
            key,
            MapValue {
                value,
                ttl: ttl_secs.map(|sec| Utc::now().timestamp() + sec),
            },
        );
        match res {
            Some(val) => println!("UPDATED; old value: {:?}", val),
            None => println!("INSERTED"),
        }
    }

    pub fn get_value(&self, key: &str) -> &Value {
        let res = self.0.get(key);
        match res {
            Some(val) => &val.value,
            None => &Value::Nil,
        }
    }

    pub fn get_type(&self, key: &str) -> &'static str {
        match self.get_value(key) {
            Value::Integer(_) => "Integer",
            Value::String(_) => "String",
            Value::List(_) => "List",
            Value::Nil => "Nil",
        }
    }

    pub fn incr_val(&mut self, key: &str) {
        self.0
            .entry(key.to_string())
            .and_modify(|f| {
                if let Value::Integer(v) = &mut f.value {
                    *v += 1;
                }
            })
            .or_insert(MapValue {
                value: Value::Integer(1),
                ttl: None,
            });
    }

    pub fn lpush(&mut self, key: String, value: String) {
        self.0
            .entry(key)
            .and_modify(|mv| {
                if let Value::List(ls) = &mut mv.value {
                    ls.insert(0, value.clone());
                }
            })
            .or_insert(MapValue {
                value: Value::List(vec![value.clone()]),
                ttl: None,
            });
    }

    pub fn lrange(&self, key: &str, start: usize, stop: usize) {
        match self.get_value(key) {
            Value::List(ls) => {
                println!("{:?}", ls.get(start..=stop));
            }
            Value::Integer(_) | Value::String(_) => {
                eprintln!("Value not subscriptable");
            }
            Value::Nil => {
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

    pub fn get_ttl(&self, key: &str) -> i32 {
        match self.0.get(key) {
            Some(x) => match x.ttl {
                Some(ttl) => {
                    let remaining = ttl - Utc::now().timestamp();
                    if remaining < 0 { 0 } else { remaining as i32 }
                }
                None => -1,
            },
            None => -2,
        }
    }

    pub fn set_ttl(&mut self, key: &str, ttl_sec: i64) -> i8 {
        match self.0.get_mut(key) {
            Some(mv) => {
                if let Some(ttl) = &mut mv.ttl {
                    *ttl = (Utc::now().timestamp() + ttl_sec) as i64;
                    -1
                } else {
                    mv.ttl = Some((Utc::now().timestamp() + ttl_sec) as i64);
                    0
                }
            }
            None => -2,
        }
    }
}
