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

    println!("Path : {:?}\nExt  : {}", folder_path, ext);

    let data = recurse(&ext, folder_path, Vec::with_capacity(1000));

    let mut total  =  0;
    let mut mapping = HashMap::new();
    let re = Regex::new(r"\b(unsafe)\b").unwrap(); // TODO ignore unsafe in comments with both '//' and '///'

    for path in data.iter() {
        if let Ok(file) = fs::read_to_string(path) {
            let file_total = re.captures_iter(&file).count();
            if let Some(str) = path.to_str() {
                mapping.insert(str.to_string(), file_total);
                total += file_total;
            }
        }
    }

    println!("\nTotal {}", total);

    for (key, val) in mapping.iter() {
        println!("{} : {}", key, val);
    }
}
