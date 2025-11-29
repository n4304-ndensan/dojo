use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();

    let mut map: HashMap<usize, usize> = HashMap::new();
    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        *map.entry(a).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (&_, &val) in map.iter() {
        ans += (val * (val - 1) / 2) * (n - val);
    }

    println!("{}", ans);
}
