use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use regex::Regex;
use std::collections::HashMap;

// Recursively retrieves all files with a given extension starting at a certain folder
fn recurse(extension: &str, path: &Path, mut data: Vec<PathBuf>) -> Vec<PathBuf> {
    for entry in path.read_dir().expect("Path is not a directory") {
        if let Ok(entry) = entry {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                data = recurse(extension, &entry_path, data);
            } else if entry_path.is_file() {
                if let Some(ext) = entry_path.extension() {
                    if let Some(ext) = ext.to_str() {
                        if ext == extension {
                            data.push(entry_path);
                        }
                    }
                }
            }
        }
    }
    return data;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Invalid arguments. Expected only two. Found {}.", args.len() - 1);
        eprintln!("Usage: ./executable [path_to_folder] [file_extension]");
        return;
    }

    let folder_path = Path::new(&args[1]);
    let ext = args[2].clone();

    if folder_path.is_file() {
        eprintln!("Invalid path. Expected a folder, found file.");
        return;
    }

    let data = recurse(&ext, folder_path, Vec::with_capacity(1000));

    let mut total  =  0;
    let mut mapping = HashMap::new();
    let match_re = Regex::new(r"\b(unsafe)\b").unwrap();
    let comment_re = Regex::new(r"\s*//.*").unwrap();

    for path in data.iter() {
        if let Ok(file_str) = fs::read_to_string(path) {
            // Remove all comments before matching
            let no_comments = comment_re.replace_all(&file_str, "");
            if let Some(str) = path.to_str() {
                let file_total = match_re.captures_iter(&no_comments).count();
                mapping.insert(str.to_string(), file_total);
                total += file_total;
            }
        }
    }

    let mut sorted = mapping
        .into_iter()
        .collect::<Vec<(String, usize)>>();
    sorted.sort_by(|lhs, rhs| lhs.1.cmp(&rhs.1));

    for pair in sorted.iter() {
        println!("{:>5} : {}", pair.1, pair.0);
    }

    println!();
    println!("Path          : {:?}", folder_path);
    println!("Ext           : {}", ext);
    println!("Total Files   : {:>5}", sorted.len());
    println!("Total matches : {:>5}", total);
}
