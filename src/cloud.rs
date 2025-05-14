use std::path::Path;

use notify::EventKind;

pub fn sync_file(path: &Path, api_key: &str, e: EventKind) {
    println!("Syncing {:?} file: {:?} with API key: {}", e, path, api_key);
    // implement actual cloud logic
}
