use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = (1..=n).collect();
    let mut offset = 0;

    for _ in 0..q {
        let query_type: usize = iter.next().unwrap().parse().unwrap();

        match query_type {
            1 => {
                let p: usize = iter.next().unwrap().parse().unwrap();
                let x: usize = iter.next().unwrap().parse().unwrap();

                let idx = (offset + p - 1) % n;
                a[idx] = x;
            }
            2 => {
                let p: usize = iter.next().unwrap().parse().unwrap();

                let idx = (offset + p - 1) % n;
                println!("{}", a[idx]);
            }
            3 => {
                let k: usize = iter.next().unwrap().parse().unwrap();
                offset = (offset + k) % n;
            }
            _ => unreachable!(),
        }
    }
}
