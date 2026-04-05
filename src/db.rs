use crate::items::Database;
use std::fs;

pub fn load() -> Database {
    let contents = match fs::read_to_string("dishes.toml") {
        Ok(s) => s,
        Err(_) => return Database { dishes: vec![] },
    };
    toml::from_str(&contents).expect("dishes.toml is fucked!")
}

pub fn save(db: &Database) {
    let contents = toml::to_string(db).expect("failed to serialize...");
    fs::write("dishes.toml", contents).expect("failed to write file...")
}
