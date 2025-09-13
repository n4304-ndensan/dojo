use regex::Regex;
use std::{env, fs};
use walkdir::WalkDir;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: grep <pattern> [path]");
        std::process::exit(1);
    }

    let pattern = &args[1];
    let path = if args.len() > 2 { &args[2] } else { "." };

    // 正規表現をコンパイル
    let re = Regex::new(pattern).unwrap_or_else(|e| {
        eprintln!("Invalid regex pattern: {}", e);
        std::process::exit(1);
    });

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.file_type().is_file() {
            let filename = entry.path().file_name().unwrap().to_string_lossy();

            // 正規表現で判定
            if re.is_match(&filename) {
                if let Ok(content) = fs::read_to_string(entry.path()) {
                    for (num, line) in content.lines().enumerate() {
                        println!("{}:{}: {}", entry.path().display(), num + 1, line);
                    }
                }
            }
        }
    }
}
