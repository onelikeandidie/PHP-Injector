use std::{path::{Path, PathBuf}, sync::mpsc::channel, time::Duration};

use notify::{RecommendedWatcher, Watcher, RecursiveMode, DebouncedEvent};

use crate::engine::php::walk_src_mappings;

use super::{config::Config, compiler::compile};

pub fn watch(config: &Config) {
    let origin = Path::new(&config.origin);
    let injections_path = Path::new(&config.injections);
    let injections_path = origin.join(injections_path);
    let src_path = origin.join(Path::new(&config.src));
    watch_files(injections_path, src_path, &config).unwrap();
}

fn watch_files(injections_path: PathBuf, src_path: PathBuf, config: &Config) -> notify::Result<()> {
    // Async channel to transmit an recieve watch events
    let (tx, rx) = channel();
    // Instance a Watcher with the dir in recursive mode
    let mut watcher: RecommendedWatcher = Watcher::new(tx, Duration::from_secs(2)).unwrap();
    watcher.watch(injections_path.clone(), RecursiveMode::Recursive).unwrap();
    println!("Watching files in {}", injections_path.clone().to_string_lossy());
    let (files, src) = walk_src_mappings(src_path);
    let compile = || {
        compile(config, Some((files.clone(), src.clone())));
    };
    // Start listening for changes
    loop {
        match rx.recv().unwrap() {
            DebouncedEvent::Create(_) => {compile();},
            DebouncedEvent::Write(_) => {compile();},
            DebouncedEvent::Remove(_) => {compile();},
            DebouncedEvent::Rename(_, _) => {compile();},
            _ => {}
        }
    }
}