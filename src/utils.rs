use std::{fs, path::PathBuf};

use regex::Regex;

use crate::config::Config;

pub fn search_directory(path: &PathBuf, pattern: &str) -> Vec<PathBuf> {
    let mut items = vec![];
    inner_search_directory(path, pattern, &mut items);

    items
}

fn inner_search_directory(path: &PathBuf, pattern: &str, items: &mut Vec<PathBuf>) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let entry_path = entry.path();
                let entry_name = entry.file_name();

                let file_name_str = entry_name.to_string_lossy();
                let re = Regex::new(&pattern).unwrap();

                if re.is_match(&file_name_str) && Config::is_accepted_file(&entry) {
                    items.push(PathBuf::from(&entry_path))
                }

                if entry_path.is_dir() {
                    inner_search_directory(&entry_path, pattern, items);
                }
            }
        }
    }
}
