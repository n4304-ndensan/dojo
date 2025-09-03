use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    // N, Q
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let q: i64 = iter.next().unwrap().parse().unwrap();

    // Ai
    let mut a: Vec<i64> = Vec::with_capacity(n as usize);
    for _ in 0..n {
        a.push(iter.next().unwrap().parse().unwrap());
    }

    let a_max = *a.iter().max().unwrap();

    // max value (問題制約に合わせる)
    let maxvalue = 1_000_000; // 問題制約に合わせる
    let mut sum_list = vec![0i64; (maxvalue + 1) as usize];
    let mut cnt_list = vec![0i64; (maxvalue + 1) as usize];

    // 出現数カウント
    for &val in &a {
        sum_list[val as usize] += val;
        cnt_list[val as usize] += 1;
    }
    // 累積和
    for i in 1..=maxvalue {
        sum_list[i as usize] += sum_list[(i - 1) as usize];
        cnt_list[i as usize] += cnt_list[(i - 1) as usize];
    }

    // クエリ処理
    for _ in 0..q {
        let b: i64 = iter.next().unwrap().parse().unwrap();

        if b <= 0 {
            continue;
        }
        if b > a_max {
            println!("-1");
            continue;
        }

        let ans = 1 + sum_list[(b - 1) as usize] + (n - cnt_list[(b - 1) as usize]) * (b - 1);

        println!("{}", ans);
    }
}
