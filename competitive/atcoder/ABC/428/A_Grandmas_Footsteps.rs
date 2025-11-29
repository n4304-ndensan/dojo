use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let s: usize = iter.next().unwrap().parse().unwrap();
    let a: usize = iter.next().unwrap().parse().unwrap();
    let b: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();

    let interval = a + b;

    let full_cycles = x / interval;

    let remainder = x % interval;

    let run_time = full_cycles * a + remainder.min(a);

    let distance = s * run_time;

    println!("{}", distance);
}
