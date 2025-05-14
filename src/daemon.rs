use crate::config::load_config;

pub fn start_all() {
    let entries = load_config();
    for entry in entries {
        std::thread::spawn(move || {
            crate::watcher::start(entry.path, entry.api_key);
        });
    }

    loop {
        std::thread::park(); // keep main thread alive
    }
}
