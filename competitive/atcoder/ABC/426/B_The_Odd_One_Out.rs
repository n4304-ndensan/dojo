use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let s: &str = iter.next().unwrap();
    let mut map: HashMap<String, usize> = HashMap::new();

    for c in s.chars() {
        *map.entry(c.to_string()).or_insert(0) += 1;
    }

    for (_key, value) in map.iter() {
        if *value == 1 {
            println!("{}", _key);
            return;
        }
    }
}
