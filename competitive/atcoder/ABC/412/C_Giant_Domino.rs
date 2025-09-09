use std::io::{self, Read};

fn solve(iter: &mut std::str::SplitWhitespace) -> i64 {
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let mut s: Vec<i64> = vec![];
    for _ in 0..n {
        let x: i64 = iter.next().unwrap().parse().unwrap();
        s.push(x);
    }
    let first = s[0];
    let last = s[n as usize - 1];
    if first * 2 >= last {
        return 2;
    }

    let mut ns: Vec<i64> = vec![];
    ns.push(first);
    for i in 0..n {
        if first < s[i as usize] && s[i as usize] < last {
            ns.push(s[i as usize]);
        }
    }
    ns.push(last);
    ns.sort();

    for i in 0..ns.len() - 1 {
        if ns[i] * 2 < ns[i + 1] {
            return -1;
        }
    }

    let mut ans = 2;
    let mut last = first;
    for i in 0..ns.len() {
        if last * 2 < ns[i] {
            last = ns[i as usize - 1];
            ans += 1;
        }
    }

    ans
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: i64 = iter.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let ans = solve(&mut iter);
        println!("{}", ans);
    }
}
