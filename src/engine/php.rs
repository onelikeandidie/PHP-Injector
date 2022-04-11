use std::{collections::HashMap, path::Path, fs};

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
    let mut parent = "".to_owned();
    let mut is_in_function = false;
    let mut last_function_depth = 0;
    let mut last_function_start = 0;
    let mut last_function_mapping = "".to_owned();
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
        if line.contains("function") {
            last_function_mapping = parent.clone() + &"$F".to_owned() + extract_function_name(line);
            is_in_function = true;
            last_function_depth = bracket_lvl;
            // TODO: Fix alt line functions
            // There's a problem here, devs that like to put their function
            // brackets on the next line will run this part of the code since
            // it assumes the function starts on the same line as the function
            // name...
            last_function_start = v_cursor;
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
        bracket_lvl += depth_change;
        if is_in_function && bracket_lvl == last_function_depth {
            map.insert(last_function_mapping.clone(), SourceMapping {
                path: path.to_str().unwrap().to_string(),
                mapping: last_function_mapping.clone(),
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

#[derive(Debug)]
pub struct SourceMapping {
    pub path: String,
    pub mapping: String,
    pub from: usize,
    pub to: usize,
}