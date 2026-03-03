use rustcache::cli::command_handler;
use rustcache::map_model::KeyValueStore;

#[test]
pub fn test_set_string() {
    let parts = vec!["SET", "name", "mahesh"];

    let mut map = KeyValueStore::new();

    command_handler(&parts, &mut map);
}
