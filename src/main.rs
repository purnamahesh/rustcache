mod map_model;
mod repl;

use map_model::KeyValueStore;

fn main() {
    let map = KeyValueStore::new();

    repl::run(map);
}
