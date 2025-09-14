use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = vec![0; n];
    for _ in 0..m {
        let l: usize = iter.next().unwrap().parse().unwrap();
        let r: usize = iter.next().unwrap().parse().unwrap();
        a[l - 1] += 1;
        if r < n {
            a[r] -= 1;
        }
    }

    for i in 1..n {
        a[i] += a[i - 1];
    }

    let ans = a.iter().min().unwrap();

    println!("{}", ans);
}
