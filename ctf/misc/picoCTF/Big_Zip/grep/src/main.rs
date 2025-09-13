use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    let pattern = &args[1];
    let path = if args.len() > 2 { &args[2] } else { "." };

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            if let Ok(content) = fs::read_to_string(entry.path()) {
                for (num, line) in content.lines().enumerate() {
                    if line.contains(pattern) {
                        println!("{}:{}: {}", entry.path().display(), num + 1, line);
                    }
                }
            }
        }
    }
}
