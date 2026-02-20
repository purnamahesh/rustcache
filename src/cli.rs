mod sub_commands;

use crate::cli::sub_commands::*;
use crate::map_model::KeyValueStore;
use std::{io::stdin, process::exit};

pub fn command_handler(parts: &Vec<&str>, map: &mut KeyValueStore) {
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "SET" => {
            set_command_handler(parts, map);
        }
        "INCR" => {
            increment_command_handler(parts, map);
        }
        "DEL" => {
            if parts.len() != 2 {
                eprintln!("ERROR: provide exactly 2 args; DEL key");
                return;
            }
            map.remove(parts[1]);
        }
        "GET" => {
            if parts.len() != 2 {
                eprintln!("ERROR: provide exactly 2 args; GET key");
                return;
            }
            println!("{:?}", map.get_value(parts[1]));
        }
        "TTL" => {
            if parts.len() != 2 {
                eprintln!("ERROR: provide exactly 2 args; TTL key");
                return;
            }
            println!("{:?}", map.get_ttl(parts[1]));
        }
        "EXPIRE" => {
            expire_command_handler(parts, map);
        }
        "LPUSH" => {
            if parts.len() != 3 {
                eprintln!("ERROR: provide exactly 3 args; LPUSH key item");
                return;
            }

            map.lpush(parts[1].to_string(), parts[2].to_string());
        }
        "LRANGE" => {}
        "TYPE" => {
            if parts.len() != 2 {
                eprintln!("ERROR: provide exactly 2 args; TYPE key");
                return;
            }
            println!("{:?}", map.get_type(parts[1]));
        }
        "DIS" => println!("{:?}", map),
        "EXIT" => exit(0),
        _ => {
            eprintln!("ERROR: invalid operation; valid operations SET, GET, DEL, DIS and EXIT")
        }
    }
}

pub fn run(mut map: KeyValueStore) {
    let mut inp = String::new();

    loop {
        stdin().read_line(&mut inp).unwrap_or_else(|err| {
            eprintln!("Error: {:?}", err);
            0_usize
        });

        let parts: Vec<&str> = inp.trim().split(" ").filter(|x| !x.is_empty()).collect();

        command_handler(&parts, &mut map);

        inp.clear();
    }
}
