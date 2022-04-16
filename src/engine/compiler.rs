use std::{path::Path, fs::{read_to_string, File, self}, collections::HashMap};
use std::io::Write;

use walkdir::WalkDir;

use crate::engine::{php::walk_src_mappings, util::get_index_of_line};

use super::php::SourceMapping;
use super::mixin::MixinTypes;
use super::interpreter::Interpreter;
use super::config::Config;

pub fn compile(
    config: &Config, 
    source: Option<(HashMap<String, String>, HashMap<String, SourceMapping>)>
) -> (usize, usize) {
    println!("Warming up Interpreter");
    // Retrive the paths 
    let origin = Path::new(&config.origin);
    let injections_path = origin.join(Path::new(&config.injections));
    let src_path = origin.join(Path::new(&config.src));
    let cache_path = origin.join(Path::new(&config.cache));
    // Copy other files if required
    if config.copy_other {
        println!("Copy option provided, copying sources to cache dir before compiling");
        copy_src(&src_path, &cache_path);
    }
    // Instanciate the interpreter
    let mut god = Interpreter::default();
    let walk_dir = WalkDir::new(injections_path.clone());
    for entry in walk_dir {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            if config.debbuging { println!("> Interpreting {}", entry.path().to_string_lossy()); }
            let contents = read_to_string(entry.path()).unwrap();
            god.interpret(&contents, entry.path());
        }
    }
    println!("Mapping Sources");
    let (mut files, mut source_mappings);
    if let Some(mappings) = source {
        // This is for when the "watcher" has a cache of the source files
        files = mappings.0; 
        source_mappings = mappings.1;
    } else {
        let mappings = walk_src_mappings(src_path);
        files = mappings.0; 
        source_mappings = mappings.1;
    }
    let file_count = files.len();
    println!("Compiling Mixins");
    let mut injections: Vec<(String, String, String, String)> = vec![];
    for mixin in &god.mixins {
        if config.debbuging { println!("> Caching {}:{} on {}", mixin.namespace, mixin.name, mixin.target); }
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
        let target_file = extract_src_map_from_target(&target);
        let src = source_mappings
            .get_mut(&target)
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
            // Replaces a section of the target
            MixinTypes::Slice(injection) => {
                let function_statement = create_mixin_call_string(mixin);
                let from = src.from + injection.from as usize;
                let index1 = get_index_of_line(file, from);
                let to = src.from + injection.to as usize;
                let index2 = get_index_of_line(file, to);
                let size = to - from;
                let pre = file[0..index1].to_string();
                let ap = file[index2..file.len()].to_string();
                let result_file = &mut format!("{}{}", pre, ap);
                move_mappings(&mut source_mappings, &path, from, size as i32);
                result_file.insert_str(index1, &function_statement);
                move_mappings(&mut source_mappings, &path, to, -1);
                file.clear();
                file.insert_str(0, &result_file);
            }
            _ => {},
        }
    }
    let mixin_count = injections.len();
    println!("Adding use statements");
    // First is src and second is injection
    let mut files_imported: Vec<(String, String)> = vec![];
    for injection in &injections {
        if config.debbuging { println!("> Injecting requires {} into {}", injection.0, injection.1); }
        let src_path = injection.1.clone();
        let garbage = injection.0.clone();
        let injection_path = Path::new(&garbage);
        let contents = files.get_mut(&src_path).expect("Error injecting?");
        let namespaced = format!("{}\\{}", injection.2, injection.3);
        let map_back = format!("\n#mixin {} from {}\n", namespaced, injection_path.clone().to_string_lossy());
        let use_statement = format!("use function {};\n", namespaced);
        // Insert after "<?php"
        contents.insert_str(5, &use_statement);
        contents.insert_str(5, &map_back);
        // Check if the require statement has been added before
        let p = injection_path.to_str().unwrap().to_string();
        if let None = files_imported.iter().find(|v| {v.1 == p && v.0 == src_path}) {
            files_imported.push((src_path, p));
        }
    }
    println!("Adding imports");
    for import in files_imported {
        let mut prepend = "";
        if config.use_document_root {
            prepend = "$_SERVER['DOCUMENT_ROOT'] . ";
        }
        let src_path = import.0;
        let injection = import.1;
        let contents = files.get_mut(&src_path).expect("Error injecting?");
        let require_statement = format!("\nrequire_once {}\"/{}\";\n", prepend, injection);
        contents.insert_str(5, &require_statement);
    }
    println!("Writing Cache");
    for file in files {
        let path = cache_path.clone().join(&file.0);
        let parent = path.parent().unwrap();
        fs::create_dir_all(parent).unwrap();
        let mut handle = File::create(path.clone()).unwrap();
        write!(handle, "{}", file.1).unwrap();
        if config.debbuging { println!("> Wrote: {}", path.clone().to_string_lossy()); }
    }
    println!("Done! {} Mixins and {} Source Files written", mixin_count, file_count);
    return (mixin_count, file_count);

}

fn create_mixin_call_string(mixin: &super::mixin::Mixin) -> String {
    return format!("\n{}({}); #mixin call {} from {}", 
        mixin.name, 
        mixin.args.join(", "), 
        mixin.name, 
        mixin.path
    );
}

fn copy_src(src_path: &Path, cache_path: &Path) {
    let walk_dir = WalkDir::new(src_path.clone());
    for entry in walk_dir {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            let from = entry.clone().into_path();
            let stripped = from.to_str().unwrap().replace(src_path.to_str().unwrap(), "");
            let to = Path::new(cache_path);
            let to = to.join(stripped);
            let parent_dir = to.parent().unwrap();
            fs::create_dir_all(parent_dir).expect("Could not create directories for source files");
            fs::copy(entry.into_path(), to).expect("Could not copy other source files");
        }
    }
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
