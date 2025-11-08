use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let h: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();

    if h > b {
        println!("{}", h - b);
    } else {
        println!("0");
    }
}
