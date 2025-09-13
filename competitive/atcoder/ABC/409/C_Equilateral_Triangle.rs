use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let l: usize = iter.next().unwrap().parse().unwrap();

    let mut d: Vec<usize> = vec![0; l + 1];
    let mut last = 1;
    d[last] += 1;

    for _ in 1..n {
        let di: usize = iter.next().unwrap().parse().unwrap();
        last += di;
        last %= l;
        if last == 0 {
            last = l;
        }
        d[last] += 1;
    }

    if l % 3 != 0 {
        println!("0");
        return;
    }

    let mut ans = 0;
    let c = l / 3;
    for i in 1..=c {
        ans += d[i] * d[i + c] * d[i + 2 * c];
    }

    println!("{}", ans);
}
