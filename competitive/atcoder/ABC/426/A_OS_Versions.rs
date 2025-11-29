use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let versions: HashMap<&str, usize> = [("Ocelot", 1), ("Serval", 2), ("Lynx", 3)]
        .iter()
        .cloned()
        .collect();

    let x: &str = iter.next().unwrap();
    let y: &str = iter.next().unwrap();

    if versions[x] >= versions[y] {
        println!("Yes");
    } else {
        println!("No");
    }
}
