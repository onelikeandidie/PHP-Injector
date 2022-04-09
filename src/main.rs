use std::env;

use php_injector::engine::compiler::compile;
use php_injector::engine::config::{Config, extract_config};
use php_injector::engine::watcher::watch;

fn main() {
    // Collect command arguments
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    if args.len() <= 1 {
        // Show the help string
        println!("Run with `--config \"path/to/config\" --watch` to watch files for changes");
    }

    let config: Config;
    match extract_config(&args) {
        Ok(imported_config) => config = imported_config,
        Err(error) => panic!("{}", error),
    }

    println!("{:?}", config);

    if args.contains(&"--watch".to_owned()) {
        watch(&config);
    } else {
        compile(&config);
    }
}
