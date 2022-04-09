use std::{path::Path, fs::{read_to_string, File, self}, collections::HashMap, env};
use std::io::Write;

use walkdir::WalkDir;

use crate::engine::{php::{read_source, extract_source_mappings}, util::get_index_of_line};

use super::php::SourceMapping;
use super::mixin::MixinTypes;
use super::interpreter::Interpreter;
use super::config::Config;

pub fn compile(config: &Config) {
    println!("Warming up Interpreter");
    // Retrive the paths 
    let origin = Path::new(&config.origin);
    let injections_path = origin.join(Path::new(&config.injections));
    let src_path = origin.join(Path::new(&config.src));
    let cache_path = origin.join(Path::new(&config.cache));
    // Instanciate the interpreter
    let mut god = Interpreter::default();
    let walk_dir = WalkDir::new(injections_path.clone());
    for entry in walk_dir {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            println!("> Interpreting {}", entry.path().to_string_lossy());
            let contents = read_to_string(entry.path()).unwrap();
            god.interpret(&contents, entry.path());
        }
    }
    println!("Mapping Sources");
    let mut files: HashMap<String, String> = HashMap::new();
    let mut source_mappings: HashMap<String, SourceMapping> = HashMap::new();
    let walk_dir = WalkDir::new(src_path.clone());
    for entry in walk_dir {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.file_name().to_str().unwrap().ends_with(".php") {
            let path = entry.path();
            println!("> Mapping {}", path.to_string_lossy());
            let contents = read_to_string(entry.path()).unwrap();
            // Take out the "src" from config file
            let mut relative_path = path.clone().to_str().unwrap();
            let src = src_path.clone();
            let src_path = src.to_str().unwrap();
            if relative_path.contains(src_path.clone()) {
                relative_path = &relative_path[src_path.len()..];
            } 
            files.insert(relative_path.clone().to_string(), contents.to_string());
            let relative_path = &Path::new(relative_path.clone());
            let mappings = extract_source_mappings(&contents, relative_path);
            source_mappings.extend(mappings);
        }
    }
    println!("Compiling Mixins");
    let mut injections: Vec<(String, String, String, String)> = vec![];
    for mixin in &god.mixins {
        println!("> Caching {}:{} on {}", mixin.namespace, mixin.name, mixin.target);
        let target = mixin.target.clone();
        if target.ends_with(".php") {
            // File mixins
            continue;
        }
        // Function mixins
        match &mixin.at {
            MixinTypes::Head(injection) => {
                let divider = target.rfind('/').expect("Could not extract target function");
                let path = &target[0..divider];
                injections.push((
                    mixin.path.to_owned(), 
                    path.to_string(), 
                    mixin.namespace.to_string(),
                    mixin.name.to_string(),
                ));
                let clit = &target[(divider + 1)..(target.len())];
                let target_file = extract_src_map_from_target(&target);
                let mut src = source_mappings
                    .get_mut(clit)
                    .expect("Could not get source mapping");
                let mut file = files
                    .get_mut(target_file)
                    .expect("Could not get source");
                src.to += 3;
                let function_statement = format!("\n{}(); #mixin call {} from {}", mixin.name, mixin.name, mixin.path);
                let from = src.from as i32 + injection.offset;
                let from  = from as usize;
                let index1 = get_index_of_line(file, from);
                file.insert_str(index1, &function_statement);
                // TODO: AFTER INSERT, UPDATE SOURCE MAPPINGS......

            },
            _ => {},
        }
    }
    println!("Adding imports");
    let current_dir = env::current_dir().unwrap();
    for injection in &injections {
        println!("> Injecting requires {} into {}", injection.0, injection.1);
        let src_path = injection.1.clone();
        let garbage = injection.0.clone();
        let injection_path = Path::new(&garbage);
        let mut contents = files.get_mut(&src_path).expect("Error injecting?");
        let mut prepend = "";
        if config.use_document_root {
            prepend = "$_SERVER['DOCUMENT_ROOT'] . ";
        }
        let namespaced = format!("{}/{}", injection.2, injection.3);
        let map_back = format!("\n#mixin {} from {}\n", namespaced, injection_path.clone().to_string_lossy());
        let use_statement = format!("use function {};\n", namespaced);
        let require_statement = format!("require_once {}\"/{}\";\n", prepend, injection_path.to_str().unwrap());
        // Insert after "<?php"
        contents.insert_str(5, &use_statement);
        contents.insert_str(5, &require_statement);
        contents.insert_str(5, &map_back);
    }
    println!("Writing Cache");
    for file in files {
        let path = cache_path.clone().join(&file.0);
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        let mut handle = File::create(path.clone()).unwrap();
        write!(handle, "{}", file.1).unwrap();
        println!("> Wrote: {}", path.clone().to_string_lossy());
    }
    println!("Done!");
}

fn extract_target_mapping(tag: &str) -> Vec<String> {
    let mut mapping: Vec<String> = vec![];
    let mut tag = tag.clone();
    while tag.len() > 0 {
        if !tag.starts_with("$") {
            panic!("Could not find target tags");
        }
        let untagged = &tag[2..tag.len()];
        let current_type = &tag[0..2];
        let next_tag = untagged.find("$").unwrap_or(untagged.len());
        let current_tag = &untagged[0..next_tag];
        let current = format!("{}{}", current_type, current_tag);
        tag = &untagged[next_tag..untagged.len()];
        if current_type == "$F" {
            mapping.push(current.clone());
        }
        if current_type == "$C" {
            mapping.push(current.clone());
        }
    }
    return mapping;
}

fn extract_src_map_from_target(target: &str) -> &str {
    let file = target.rfind("/").unwrap();
    return &target[0..file];
}

fn move_mappings(
    source_mapping: &mut HashMap<String, SourceMapping>, 
    start_index: usize,
    move_amount: usize,
) {
}