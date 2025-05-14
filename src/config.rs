use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct WatchEntry {
    pub path: String,
    pub api_key: String,
}

fn config_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("dirsync")
        .join("config.json")
}

pub fn add_watch(path: String, api_key: String) {
    let mut entries = load_config();
    entries.push(WatchEntry { path, api_key });
    save_config(&entries);
    println!("✅ Watch added.");
}

pub fn load_config() -> Vec<WatchEntry> {
    let path = config_path();
    if path.exists() {
        let data = fs::read_to_string(path).unwrap();
        serde_json::from_str(&data).unwrap()
    } else {
        Vec::new()
    }
}

fn save_config(entries: &Vec<WatchEntry>) {
    let path = config_path();
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    fs::write(path, serde_json::to_string_pretty(entries).unwrap()).unwrap();
}