use rustcache::cli;
use rustcache::map_model::KeyValueStore;

fn main() {
    let map = KeyValueStore::new();

    cli::run(map);
}
