use std::{collections::HashMap, path::Path, fs::{self, read_to_string}};

use regex::Regex;
use lazy_static::lazy_static;
use walkdir::WalkDir;

lazy_static! {
    static ref FUNCTION_ARGS_REGEX: Regex = Regex::new(r"\$\w+").unwrap();
}

pub fn extract_namespace(line: &str) -> &str {
    // namespace and a space has 10 chars (also remove the trailing ';')
    let name = &line[10..(line.len() - 1)];
    return name;
}

pub fn extract_function_name(line: &str) -> &str {
    let line = line.trim();
    let function_start_index = line.find("function").unwrap();
    let parameter_start_index = line.find("(").unwrap();
    // function and a space has 9 chars
    let name = &line[(function_start_index + 9)..parameter_start_index];
    return name;
}

pub fn extract_function_params(line: &str) -> Vec<String> {
    let line = line.trim();
    let regex = &FUNCTION_ARGS_REGEX;
    let mut args = vec![]; 
    for capture in regex.captures_iter(line) {
        args.push(capture[0].to_string());
    };
    return args;
}

pub fn extract_class_name(line: &str) -> &str {
    let line = line.trim();
    let bracket_start_index = line.find("{").unwrap_or(line.len());
    // class and a space has 6 chars
    let name = &line[6..bracket_start_index].trim();
    return name;
}

pub fn read_source(path: &Path) -> HashMap<String, SourceMapping> {
    // Get source files
    let text = fs::read_to_string(path).expect("Could not read target file");
    // Extract source mappings
    return extract_source_mappings(&text, &path);
}

pub fn extract_source_mappings(php_content: &String, path: &Path) -> HashMap<String, SourceMapping> {
    let mut map = HashMap::new();
    let mut lines = php_content.lines();
    let mut is_in_multiline_comment = false;
    let mut is_in_function = false;
    let mut is_in_multiline_params = false;
    let mut find_last_function_start = false;
    let mut parent = "".to_owned();
    let mut last_function_mapping = "".to_owned();
    let mut last_function_args: Vec<String> = vec![];
    let mut last_function_depth = 0;
    let mut last_function_start = 0;
    let mut bracket_lvl = 0;
    let mut v_cursor = 0;
    while let Some(line) = lines.next() {
        v_cursor += 1;
        let line = line.trim();
        let mut depth_change = 0;
        if line.starts_with("#") || line.starts_with("//") {
            continue;
        }
        if line.starts_with("/*") || line.starts_with("/**") {
            is_in_multiline_comment = true;
        }
        if line.starts_with("*/") {
            is_in_multiline_comment = false;
        }
        if is_in_multiline_comment {
            continue;
        }
        // If we don't search for "function " it'll also match calls to
        // functions like "function_exists". So for now, adding a space
        // fixes this issue
        if line.contains("function ") {
            last_function_mapping = parent.clone() + &"$F".to_owned() + extract_function_name(line);
            last_function_depth = bracket_lvl;
            last_function_args = vec![];
            if line.trim_end().ends_with("{") {
                is_in_function = true;
                last_function_start = v_cursor;
            } else {
                find_last_function_start = true;
            }
            // Check for 1 line params
            if line.contains("(") && line.contains(")") {
                last_function_args = extract_function_params(line);
            }
            if line.contains("(") {
                is_in_multiline_params = true;
            }
        }
        if line.starts_with("class") {
            parent = "$C".to_owned() + extract_class_name(line);
        }
        if line.starts_with("{") || line.ends_with("{") {
            depth_change += 1;
        }
        if line.starts_with("}") || line.ends_with("}") {
            depth_change -= 1;
        }
        if is_in_multiline_params {
            let args = extract_function_params(line);
            last_function_args.to_vec().extend(args);
            if line.starts_with(")") || line.ends_with(")") {
                is_in_multiline_params = false;
            }
        }
        if find_last_function_start && depth_change > 0 {
            last_function_start = v_cursor;
            find_last_function_start = false;
            is_in_function = true;
        }
        bracket_lvl += depth_change;
        if is_in_function && bracket_lvl == last_function_depth {
            map.insert(last_function_mapping.clone(), SourceMapping {
                path: path.to_str().unwrap().to_string(),
                mapping: last_function_mapping.clone(),
                args: last_function_args.to_vec(),
                from: last_function_start,
                to: v_cursor,
            });
            is_in_function = !is_in_function;
        }
        if bracket_lvl <= 0 {
            if parent.len() > 0 {
                parent = "".to_owned();
            }
        }
    }
    // Check if it is still in a function or a class
    if is_in_function || parent.len() > 0 {
        panic!("Could not create source mappings");
    }
    return map;
}

pub fn walk_src_mappings(src_path: std::path::PathBuf) -> (HashMap<String, String>, HashMap<String, SourceMapping>) {
    let mut files: HashMap<String, String> = HashMap::new();
    let mut source_mappings: HashMap<String, SourceMapping> = HashMap::new();
    let walk_dir = WalkDir::new(src_path.clone());
    for entry in walk_dir {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.file_name().to_str().unwrap().ends_with(".php") {
            let path = entry.path();
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
    (files, source_mappings)
}

#[derive(Debug, Clone)]
pub struct SourceMapping {
    pub path: String,
    pub mapping: String,
    pub args: Vec<String>,
    pub from: usize,
    pub to: usize,
}