use std::{path::{Path, PathBuf}, sync::mpsc::channel, time::Duration};

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use super::{config::Config, compiler::compile};

pub fn watch(config: &Config) {
    let origin = Path::new(&config.origin);
    let injections_path = Path::new(&config.injections);
    let path = origin.join(injections_path);
    watch_files(path, &config).unwrap();
}

fn watch_files(path: PathBuf, config: &Config) -> notify::Result<()> {
    // Async channel to transmit an recieve watch events
    let (tx, rx) = channel();
    // Instance a Watcher with the dir in recursive mode
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(path.clone(), RecursiveMode::Recursive).unwrap();
    println!("Watching files in {}", path.clone().to_string_lossy());
    // Start listening for changes
    loop {
        match rx.recv().unwrap() {
            DebouncedEvent::Create(_) => compile(config),
            DebouncedEvent::Write(_) => compile(config),
            DebouncedEvent::Remove(_) => compile(config),
            DebouncedEvent::Rename(_, _) => compile(config),
            _ => {}
        }
    }
}