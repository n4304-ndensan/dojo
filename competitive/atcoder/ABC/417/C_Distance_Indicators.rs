use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    let mut freq: HashMap<i64, i64> = HashMap::new();
    let mut ans: i64 = 0;

    for (i, &ai) in a.iter().enumerate() {
        let idx = (i + 1) as i64;

        let left = idx - ai;

        if let Some(&cnt) = freq.get(&left) {
            ans += cnt;
        }

        let right = idx + ai;
        *freq.entry(right).or_insert(0) += 1;
    }

    println!("{}", ans);
}
