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

pub fn list_watches() {
    let entries = load_config();
    if entries.is_empty() {
        println!("📭 No watches found.");
        return;
    }
    for (i, entry) in entries.iter().enumerate() {
        println!("{} | Path: {} | API Key: {}", i + 1, entry.path, entry.api_key);
    }
}

pub fn update_watch(path: String, new_api_key: String) {
    let mut entries = load_config();
    let mut found = false;

    for entry in &mut entries {
        if entry.path == path {
            entry.api_key = new_api_key.clone();
            found = true;
        }
    }

    if found {
        save_config(&entries);
        println!("✅ Updated API key for path: {}", path);
    } else {
        println!("❌ Path not found: {}", path);
    }
}

pub fn delete_watch(path: String) {
    let mut entries = load_config();
    let initial_len = entries.len();
    entries.retain(|entry| entry.path != path);

    if entries.len() != initial_len {
        save_config(&entries);
        println!("🗑️ Deleted watch for path: {}", path);
    } else {
        println!("❌ Path not found: {}", path);
    }
}