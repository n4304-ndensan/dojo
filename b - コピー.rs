use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut list_a = Vec::new();

    list_a.push(0); // 1-indexed
    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        list_a.push(a);
    }

    let mut max = 2;
    let mut ans = 0;
    for i in 1..=n {
        if i >= max {
            break;
        }
        if max < i + list_a[i] {
            max = i + list_a[i];
        }
        ans += 1;
    }

    println!("{}", ans);
}
