use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let w: usize = iter.next().unwrap().parse::<usize>().unwrap() * 1000;
    let b: usize = iter.next().unwrap().parse().unwrap();

    let ans = w / b + 1;
    println!("{}", ans);
}