use std::fs;
use std::env;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::util::get_index;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub injections: String,
    pub src: String,
    pub cache: String,
    pub use_document_root: Option<bool>,
    pub copy_other: Option<bool>
}

#[derive(Debug)]
pub struct Config {
    pub injections: String,
    pub src: String,
    pub cache: String,
    pub origin: String,
    pub use_document_root: bool,
    pub copy_other: bool,
    pub debbuging: bool
}

pub fn extract_config(args: &Vec<String>) -> Result<Config, String> {
    // Check if the --config flag was passed
    // And find the file indicated
    if args.contains(&"--config".to_string()) {
        // Get the index of the --config flag in the arguments
        let index_of_flag = get_index(&args, "--config");
        if index_of_flag < 0 {
            return Err("Config given, attempting to load".to_string());
        }
        // Check if there is a path after the config flag
        if let Some(path) = args.get((index_of_flag + 1) as usize) {
            return load_config(path);
        } else {
            println!("Config path not passed after flag, please use `--config \"path/to/config\"`");
        }
    }

    // Check if instead, the arguments were passed
    let has_src         = args.contains(&"--src".to_string());
    let has_injections  = args.contains(&"--injections".to_string());
    let has_cache       = args.contains(&"--cache".to_string());
    if has_src && has_injections && has_cache {
        let index_of_src        = (get_index(&args, "--src")        + 1) as usize;
        let index_of_injections = (get_index(&args, "--injections") + 1) as usize;
        let index_of_cache      = (get_index(&args, "--cache")      + 1) as usize;
        let src = args.get(index_of_src).unwrap_or(&"./src".to_string()).to_owned();
        let inj = args.get(index_of_injections).unwrap_or(&"./injections".to_string()).to_owned();
        let cac = args.get(index_of_cache).unwrap_or(&"./cache".to_string()).to_owned();
        return Ok(Config {
            injections: inj,
            src: src,
            cache: cac,
            origin: ".".to_string(),
            use_document_root: true,
            debbuging: false,
            copy_other: false
        });
    }

    // Maybe the config is in the root where this was called
    let current_dir = env::current_dir().unwrap();
    let current_dir_string = current_dir.to_str().unwrap().to_owned();
    let possible_config_path_string = current_dir_string + "/php-injector.json";
    let possible_config_path = Path::new(&possible_config_path_string);
    let config_exists = possible_config_path.exists();
    if config_exists {
        return load_config(&possible_config_path_string);
    }

    return Err("No configuration provided!".to_string());
}

fn load_config(path: &str) -> Result<Config, String> {
    let contents = fs::read_to_string(path);
    let json;
    match contents {
        Ok(txt) => {
            json = serde_json::from_str::<ConfigFile>(&txt);
        },
        Err(_) => {
            let error_msg = format!("Could not retrieve file contents! {}", path);
            return Err(error_msg);
        },
    }
    match json {
        Ok(config) => {
            // Attach origin config parent dir path
            let path_obj = Path::new(path).parent().unwrap();
            return Ok(Config {
                injections: config.injections,
                src: config.src,
                cache: config.cache,
                origin: path_obj.to_str().unwrap().to_string(),
                use_document_root: config.use_document_root.unwrap_or(true),
                copy_other: config.use_document_root.unwrap_or(false),
                debbuging: false
            });
        },
        Err(_) => return Err("Could not parse config file!".to_string()),
    }
}