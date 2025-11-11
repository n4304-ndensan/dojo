use std::convert::TryInto;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut sum_b: i64 = 0;
    let mut sum_w: i64 = 0;
    let mut list: Vec<(usize, usize)> = Vec::new();
    for _ in 0..n {
        let w: usize = iter.next().unwrap().parse().unwrap();
        let h: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        sum_b += b as i64;
        sum_w += w as i64;
        if h > b {
            list.push((w, h - b));
        }
    }

    /* 典型のDP
    let list_size = list.len();
    let limit: usize = (sum_w / 2).try_into().unwrap();
    const INF: i64 = -10_i64.pow(10);
    let mut dp: Vec<Vec<i64>> = vec![vec![INF; limit + 1]; list_size + 1];
    dp[0][0] = 0;

    for i in 0..list_size {
        let (w, gain) = list[i];
        for j in 0..=limit {
            if dp[i][j] == INF {
                continue;
            }
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
            if w + j <= limit {
                dp[i + 1][w + j] = dp[i + 1][w + j].max(dp[i][j] + gain as i64);
            }
        }
    }

    let mut ans = 0;
    for i in 0..=limit {
        ans = ans.max(dp[list_size][i]);
    }

    ans += sum_b;
    println!("{}", ans);
    */

    /*
    let limit: usize = (sum_w / 2).try_into().unwrap();
    let list_size = list.len();
    let mut dp: Vec<Vec<i64>> = vec![vec![0; limit + 1]; list_size + 1];

    for i in 0..list_size {
        let (w, gain) = list[i];
        for j in 0..=limit {
            if w + j <= limit {
                dp[i + 1][w + j] = dp[i + 1][w + j].max(dp[i][j] + gain as i64);
            }
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j]);
        }
    }

    let ans = dp[list_size][limit] + sum_b;
    println!("{}", ans);
    */

    // 一次元のDP効率的に 200ms => 20ms
    let limit: usize = (sum_w / 2).try_into().unwrap();
    let mut dp: Vec<i64> = vec![0; limit + 1];

    for (w, gain) in list {
        if w > limit {
            continue;
        }

        for j in (0..=limit - w).rev() {
            dp[j + w] = dp[j + w].max(dp[j] + gain as i64);
        }
    }

    let ans = sum_b + dp[limit];
    println!("{}", ans);
}
