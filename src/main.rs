mod watcher;
mod cloud;
mod config;

fn main() {
    let cfg = config::load();
    watcher::start(cfg.watch_path, cfg.cloud_key);
}
