use crate::map_model::{KeyValueStore, Value};
use std::io::stdin;

pub fn run(mut map: KeyValueStore) {
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
                if !(parts.len() == 3 || parts.len() == 5) {
                    eprintln!("ERROR: SET expects 3 or 5 args; SET key val [EX seconds]");
                    continue;
                }

                let ttl = if parts.len() == 5 {
                    if !parts[3].eq("EX") {
                        eprint!("ERROR: incorrect syntax; SET key value EX seconds");
                        continue;
                    }
                    match parts[4].parse::<i64>() {
                        Ok(sec) => {
                            if sec <= 1 {
                                eprintln!(
                                    "expiry seconds should be greater than or equal to 1 second"
                                );
                                continue;
                            }
                            Some(sec)
                        }
                        Err(err) => {
                            eprintln!("ERROR: parsing {}, {:?}", parts[4], err);
                            continue;
                        }
                    }
                } else {
                    None
                };

                map.insert(
                    parts[1].to_string(),
                    match parts[2].parse::<i64>() {
                        Ok(val) => Value::Integer(val),
                        Err(_) => Value::String(parts[2].trim().to_string()),
                    },
                    ttl,
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
            "TTL" => {
                if parts.len() != 2 {
                    eprintln!("ERROR: provide exactly 2 args; TTL key");
                    continue;
                }
                println!("{:?}", map.get_ttl(parts[1]));
            }
            "EXPIRE" => {
                if parts.len() != 3 {
                    eprintln!("ERROR: provide exactly 3 args; EXPIRE key seconds");
                    continue;
                }

                
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
