pub struct Cache {
    // current_page: String,
    // player_name: String,
    // player_id: i32,
}

pub fn new() -> Cache {
    let cache = Cache {
        // current_page: String::from(""),
        // player_name: String::from(""),
        // player_id: 0,
    };
    cache
}

// pub fn insert(map: &mut HashMap<String, Cache>, key: String, value: Cache) {
//     map.insert(key, value);
// }

// pub fn get<'a>(map: &'a HashMap<String, Cache>, key: &String) -> Option<&'a Cache> {
//     map.get(key)
// }

// pub fn save(map: &HashMap<String, Cache>) -> std::io::Result<()> {
//     let file = File::create("cache.json")?;
//     let json = serde_json::to_string(map)?;
//     file.write_all(json.as_bytes())?;
//     Ok(())
// }

// pub fn load() -> std::io::Result<HashMap<String, Cache>> {
//     let mut file = File::open("cache.json")?;
//     let mut json = String::new();
//     file.read_to_string(&mut json)?;
//     let map = serde_json::from_str(&json)?;
//     Ok(map)
// }
