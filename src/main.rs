mod map_model;
mod repl;

use map_model::KeyValueStore;

// use std::{thread::sleep, time::Duration};

// use chrono::Utc;

fn main() {
    let map = KeyValueStore::new();

    repl::run(map);

    // println!("{}", Utc::now().timestamp());

    // sleep(Duration::from_secs(5));

    // println!("{}", Utc::now().timestamp());
}
