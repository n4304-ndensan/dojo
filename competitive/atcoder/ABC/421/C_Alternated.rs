use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<char> = iter.next().unwrap().chars().collect();

    let mut a_positions: Vec<usize> = Vec::new();
    for (i, &c) in s.iter().enumerate() {
        if c == 'A' {
            a_positions.push(i);
        }
    }

    // ABAB...
    let mut target1: Vec<usize> = Vec::new();
    for i in 0..n {
        target1.push(2 * i);
    }

    // BABA...
    let mut target2: Vec<usize> = Vec::new();
    for i in 0..n {
        target2.push(2 * i + 1);
    }

    let cost1: usize = a_positions
        .iter()
        .zip(target1.iter())
        .map(|(a, t)| (*a as isize - *t as isize).abs() as usize)
        .sum();

    let cost2: usize = a_positions
        .iter()
        .zip(target2.iter())
        .map(|(a, t)| (*a as isize - *t as isize).abs() as usize)
        .sum();

    let ans = cost1.min(cost2);
    println!("{}", ans);
}
