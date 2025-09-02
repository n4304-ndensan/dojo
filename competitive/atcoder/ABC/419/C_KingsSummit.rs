use std::cmp::max;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // N
    let n: usize = iter.next().unwrap().parse().unwrap();

    // x, y を Vec にする
    let mut x: Vec<i64> = Vec::with_capacity(n);
    let mut y: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        x.push(iter.next().unwrap().parse().unwrap());
        y.push(iter.next().unwrap().parse().unwrap());
    }

    let mx: i64 = *x.iter().max().unwrap() - *x.iter().min().unwrap();
    let my: i64 = *y.iter().max().unwrap() - *y.iter().min().unwrap();

    let ans: i64 = (max(mx, my) + 1) / 2;

    println!("{}", ans);
}
