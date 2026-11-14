use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
};

pub fn set_command(
    key: &str,
    value: &str,
    db: &Arc<RwLock<HashMap<String, String>>>,
) -> Option<String> {
    match db.write() {
        Ok(mut map) => map.insert(key.to_string(), value.to_string()),
        Err(err) => {
            println!("Failed to acquire write lock: {}", err);
            None
        }
    }
}

pub fn get_command<'a>(key: &str, db: &Arc<RwLock<HashMap<String, String>>>) -> Option<String> {
    match db.read() {
        Ok(map) => map.get(key).cloned(),
        Err(err) => {
            println!("Failed to acquire read lock: {}", err);
            None
        }
    }
}