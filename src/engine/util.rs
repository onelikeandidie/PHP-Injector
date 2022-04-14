pub fn get_index(vector: &Vec<String>, key: &str) -> i32 {
    let lookup = vector.iter().position(|v| v == key);
    match lookup {
        Some(index) => return index as i32,
        None => return -1
    }
}

pub fn get_index_of_line(txt: &str, index: usize) -> usize {
    let lines_slice = &txt
        .split("\n")
        .collect::<Vec<&str>>()
        [0..index];
    let lines = lines_slice
        .into_iter()
        .map(|line| {line.len()})
        .collect::<Vec<usize>>();
    // Count chars of lines before index
    let result_index = lines
        .into_iter()
        .reduce(|accum, line_len| {
            // Don't forget the \n char
            return accum + line_len + 1;
        })
        .unwrap();
    return result_index;
}

pub fn count_occurences_not_in_string(txt: &str, pat: char) -> usize {
    let mut count = 0 as usize;
    let mut is_in_string = false;
    let mut is_escaped = false;
    let mut chars = txt.chars();
    while let Some(character) = chars.next() {
        if is_escaped {
            is_escaped = false;
            continue;
        }
        if character == '\\' {
            is_escaped = true;
            continue;
        }
        if character == pat && !is_in_string {
            count += 1;
        }
        if character == '"' || character == '\''{
            is_in_string = !is_in_string;
        }
    }
    return count;
}