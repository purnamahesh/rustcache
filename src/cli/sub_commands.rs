use crate::map_model::{KeyValueStore, Value};

pub fn lrange_command_handler(parts: &Vec<&str>, map: &mut KeyValueStore) {
    if parts.len() != 4 {
        eprintln!("ERROR: provide exactly 4 args; LRANGE key start stop");
        return;
    }

    let start = match parts[2].parse::<usize>() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    let stop = match parts[3].parse::<usize>() {
        Ok(s) => s,
        Err(err) => {
            eprintln!("Error: {}", err);
            return;
        }
    };

    map.lrange(parts[1], start, stop)
}

pub fn expire_command_handler(parts: &Vec<&str>, map: &mut KeyValueStore) {
    if parts.len() != 3 {
        eprintln!("ERROR: provide exactly 3 args; EXPIRE key seconds");
        return;
    }

    let ttl_sec = match parts[2].parse::<i64>() {
        Ok(val) => {
            if val <= 0 {
                eprintln!("expire seconds should be greater than 0");
                return;
            }
            val
        }
        Err(err) => {
            eprintln!("ERROR: parsing error {}", err);
            return;
        }
    };

    println!("{}", map.set_ttl(parts[1], ttl_sec));
}

pub fn increment_command_handler(parts: &Vec<&str>, map: &mut KeyValueStore) {
    if parts.len() != 2 {
        eprintln!("ERROR: provide exactly 2 args; INCR key");
        return;
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

pub fn set_command_handler(parts: &Vec<&str>, map: &mut KeyValueStore) {
    if !(parts.len() == 3 || parts.len() == 5) {
        eprintln!("ERROR: SET expects 3 or 5 args; SET key val [EX seconds]");
        return;
    }

    let ttl = if parts.len() == 5 {
        if !parts[3].eq("EX") {
            eprint!("ERROR: incorrect syntax; SET key value EX seconds");
            return;
        }
        match parts[4].parse::<i64>() {
            Ok(sec) => {
                if sec <= 1 {
                    eprintln!("expiry seconds should be greater than or equal to 1 second");
                    return;
                }
                Some(sec)
            }
            Err(err) => {
                eprintln!("ERROR: parsing {}, {:?}", parts[4], err);
                return;
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
