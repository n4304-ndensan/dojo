use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let s: String = iter.next().unwrap().to_string();
    let center_pos = s.len() / 2;

    let ans = s[..center_pos].to_string() + &s[center_pos + 1..];
    println!("{}", ans);
}
