use std::io::{self, Read};

fn find_little_endian(word: &str) -> String {
    let bytes = word.as_bytes();
    let mut result = String::with_capacity(bytes.len() * 2);

    for &b in bytes.iter().rev() {
        // 各バイトを大文字16進に変換
        result.push_str(&format!("{:02X}", b));
    }

    result
}

fn find_big_endian(word: &str) -> String {
    let bytes = word.as_bytes();
    let mut result = String::with_capacity(bytes.len() * 2);

    for &b in bytes {
        result.push_str(&format!("{:02X}", b));
    }

    result
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let word = iter.next().unwrap();

    let little = find_little_endian(word);
    let big = find_big_endian(word);

    println!("word = {}", word);
    println!("little endian = {}", little); // "4241"
    println!("big endian    = {}", big); // "4142"
}
