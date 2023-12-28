use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Write};

pub struct Cache<K, V> {
    map: HashMap<K, V>,
}

impl<K: std::hash::Hash + std::cmp::Eq, V> Cache<K, V> {
    pub fn new() -> Cache<K, V> {
        Cache {
            map: HashMap::new(),
        }
    }

    pub fn get(&self, key: &K) -> Option<&V> {
        self.map.get(key)
    }

    pub fn insert(&mut self, key: K, value: V) {
        self.map.insert(key, value);
    }
}

// save cache in a file
pub fn save_cache(cache: &Cache<String, i32>) -> std::io::Result<()> {
    let file = File::create("cache.json")?;
    let json = serde_json::to_string(cache)?;
    file.write_all(json.as_bytes())?;
    Ok(())
}

pub fn load_cache() -> std::io::Result<Cache<String, i32>> {
    let mut file = File::open("cache.json")?;
    let mut json = String::new();
    file.read_to_string(&mut json)?;
    let cache = serde_json::from_str(&json)?;
    Ok(cache)
}
