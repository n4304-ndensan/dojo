use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();

    let mut h: Vec<usize> = Vec::new();
    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        h.push(a);
    }
    let mut b: Vec<usize> = Vec::new();
    for _ in 0..m {
        let a: usize = iter.next().unwrap().parse().unwrap();
        b.push(a);
    }

    h.sort_by(|a, b| b.cmp(a));
    b.sort_by(|a, b| b.cmp(a));

    let mut ans = 0;
    let mut h_idx = 0;
    let mut b_idx = 0;
    while h_idx < h.len() && b_idx < b.len() {
        if h[h_idx] <= b[b_idx] {
            ans += 1;
            h_idx += 1;
            b_idx += 1;
        } else {
            h_idx += 1;
        }
    }

    if ans >= k {
        println!("Yes");
    } else {
        println!("No");
    }
}
