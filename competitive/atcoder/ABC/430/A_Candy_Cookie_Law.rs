use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let c: usize = iter.next().unwrap().parse().unwrap();
    let d: usize = iter.next().unwrap().parse().unwrap();

    if a > c {
        println!("No");
    } else if d >= b {
        println!("No");
    } else {
        println!("Yes");
    }
}
