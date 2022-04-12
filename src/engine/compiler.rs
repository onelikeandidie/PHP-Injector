use std::{path::Path, fs::{read_to_string, File, self}, collections::HashMap};
use std::io::Write;

use walkdir::WalkDir;

use crate::engine::{php::{extract_source_mappings}, util::get_index_of_line};

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
        if mixin.at == MixinTypes::None {
            continue;
        }
        // Get the source mapping for this mixin
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
        let file = files
            .get_mut(target_file)
            .expect("Could not get source");
        // Function mixins
        match &mixin.at {
            // Inserts at the start of target
            MixinTypes::Head(injection) => {
                let function_statement = create_mixin_call_string(mixin);
                let from = src.from as i32 + injection.offset;
                let from  = from as usize;
                let index1 = get_index_of_line(file, from);
                file.insert_str(index1, &function_statement);
                move_mappings(&mut source_mappings, &path, from, 1);
            },
            // Inserts at the end of target
            MixinTypes::Tail(injection) => {
                let function_statement = create_mixin_call_string(mixin);
                let to = src.to as i32 - injection.offset;
                let to  = to as usize;
                let index1 = get_index_of_line(file, to);
                file.insert_str(index1, &function_statement);
                move_mappings(&mut source_mappings, &path, to, -1);
            },
            _ => {},
        }
    }
    println!("Adding imports");
    for injection in &injections {
        println!("> Injecting requires {} into {}", injection.0, injection.1);
        let src_path = injection.1.clone();
        let garbage = injection.0.clone();
        let injection_path = Path::new(&garbage);
        let contents = files.get_mut(&src_path).expect("Error injecting?");
        let mut prepend = "";
        if config.use_document_root {
            prepend = "$_SERVER['DOCUMENT_ROOT'] . ";
        }
        let namespaced = format!("{}\\{}", injection.2, injection.3);
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

fn create_mixin_call_string(mixin: &super::mixin::Mixin) -> String {
    return format!("\n{}({}); #mixin call {} from {}", 
        mixin.name, 
        mixin.args.join(", "), 
        mixin.name, 
        mixin.path
    );
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
    file_name: &str,
    start_index: usize,
    move_amount: i32,
) {
    for ele in source_mapping {
        let mut mapping = ele.1;
        // Find the source mapping with the same path
        if mapping.path != file_name {
            continue;
        }
        // If the "to" and "from" are after the start_index
        // Then move them by the required amount
        if mapping.from > start_index {
            // WARN: This should throw a warning in the future
            // Converting from i32 to usize can be unsafe
            mapping.from = (mapping.from as i32 + move_amount) as usize;
        }
        if mapping.to > start_index {
            mapping.to = (mapping.to as i32 + move_amount) as usize;
        }
    }
}