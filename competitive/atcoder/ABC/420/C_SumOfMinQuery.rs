use std::cmp::min;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // N, Q
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    // A, B
    let mut a: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();
    let mut b: Vec<i64> = (0..n)
        .map(|_| iter.next().unwrap().parse().unwrap())
        .collect();

    // クエリ：(対象, インデックス, 変換値)
    let queries: Vec<(String, usize, i64)> = (0..q)
        .map(|_| {
            let ci: String = iter.next().unwrap().parse().unwrap();
            let xi: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
            let vi: i64 = iter.next().unwrap().parse().unwrap();
            (ci, xi, vi)
        })
        .collect();

    let mut min_list: Vec<i64> = (0..n).map(|i| min(a[i], b[i])).collect();
    let mut ans: i64 = min_list.iter().sum();

    // クエリ処理
    for (ci, idx, val) in queries {
        if ci == "A" {
            a[idx] = val;
        } else {
            b[idx] = val;
        }

        // 差分を反映
        let new_min = min(a[idx], b[idx]);
        ans += new_min - min_list[idx];
        min_list[idx] = new_min;

        println!("{}", ans);
    }
}
