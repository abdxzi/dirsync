use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::time::Duration;


pub fn start(path: String, api_key: String) {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();
    watcher
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .unwrap();

    let mut last_sync_time = std::time::Instant::now();

    for event in rx {
        match event {
            Ok(Event { kind, paths, .. }) => {
                if matches!(
                    kind,
                    EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_)
                ) {
                    for path in paths {
                        // Ignore if multiple events occur in a short period (e.g., 2 sec)
                        if last_sync_time.elapsed() < Duration::from_secs(2) {
                            continue;
                        }

                        crate::cloud::sync_file(&path, &api_key, kind.clone());

                        // Update last sync time
                        last_sync_time = std::time::Instant::now();
                    }
                }
            }
            Err(e) => eprintln!("watch error: {:?}", e),
        }
    }
}
