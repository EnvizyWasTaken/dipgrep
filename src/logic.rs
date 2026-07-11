// --- Imports ---
use regex::Regex;
use std::fs::read_to_string;
use std::io;

// --- Read path function ---

pub fn read_path(path: &str) -> Result<String, io::Error> {
    let contents = read_to_string(path)?;
    Ok(contents)
}

// --- pattern functions ---

pub fn match_pattern(pattern: &str, contents: &str) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.contains(pattern) {
            result.push((i, line.to_string()))
        }
    }
    result
}

pub fn match_pattern_exact(pattern: &str, contents: &str) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.split_whitespace().any(|w| w == pattern) {
            result.push((i, line.to_string()))
        }
    }
    result
}

pub fn match_pattern_insensitive(pattern: &str, contents: &str) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    for (i, line) in contents.lines().enumerate() {
        if line.to_lowercase().contains(&pattern.to_lowercase()) {
            result.push((i, line.to_string()))
        }
    }
    result
}

pub fn match_pattern_regex(pattern: &str, contents: &str) -> Vec<(usize, String)> {
    let mut result: Vec<(usize, String)> = Vec::new();
    let re = match Regex::new(pattern) {
        Ok(r) => r,
        Err(_) => return vec![],
    };
    for (i, line) in contents.lines().enumerate() {
        if re.is_match(line) {
            result.push((i, line.to_string()))
        }
    }
    result
}

// --- Search function ---

pub fn search_directory(
    path: &str,
    pattern: &str,
    recursive: bool,
    algorithm: fn(&str, &str) -> Vec<(usize, String)>,
) -> Vec<(String, usize, String)> {
    let mut results: Vec<(String, usize, String)> = Vec::new();

    let entries = match std::fs::read_dir(path) {
        Ok(dir) => dir,
        Err(_) => return vec![],
    };

    for entry in entries {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };
        let entry_path = entry.path();

        if entry_path.is_dir() {
            if recursive {
                let sub_results = search_directory(
                    entry_path.to_str().unwrap_or(""),
                    pattern,
                    recursive,
                    algorithm,
                );
                results.extend(sub_results);
            }
        } else {
            let contents = match read_path(entry_path.to_str().unwrap_or("")) {
                Ok(c) => c,
                Err(_) => continue,
            };
            let filename = entry_path.to_str().unwrap_or("unknown").to_string();
            let matches = algorithm(pattern, &contents);
            for (line_num, line_content) in matches {
                results.push((filename.clone(), line_num, line_content));
            }
        }
    }
    results
}
