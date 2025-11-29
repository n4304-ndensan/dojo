use std::collections::HashMap;
use std::io::{self, Read};

fn pow10(k: usize) -> usize {
    (0..k).fold(1, |acc, _| acc * 10)
}

fn digits(x: usize) -> usize {
    if x == 0 {
        1
    } else {
        (x as f64).log10().floor() as usize + 1
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<usize> = Vec::with_capacity(n);
    let mut count_by_len: Vec<HashMap<usize, usize>> = vec![HashMap::new(); 11];

    for _ in 0..n {
        let x: usize = iter.next().unwrap().parse().unwrap();
        let l: usize = digits(x);
        a.push(x);
        *count_by_len[l].entry(x % m).or_insert(0) += 1;
    }

    let mut ans: usize = 0;
    for i in 0..n {
        for j in 1..=10 {
            let target_mod = (m - ((a[i] * pow10(j)) % m)) % m;
            ans += count_by_len[j].get(&target_mod).unwrap_or(&0);
        }
    }

    println!("{}", ans);
}
