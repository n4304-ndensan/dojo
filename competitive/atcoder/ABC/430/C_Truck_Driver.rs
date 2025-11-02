use std::io::{self, Read};

fn main() {
    // 入力処理
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let a_need: usize = iter.next().unwrap().parse().unwrap();
    let b_limit: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<char> = iter.next().unwrap().chars().collect();

    // 累積和
    let mut a_sum = vec![0; n + 1];
    let mut b_sum = vec![0; n + 1];
    for i in 0..n {
        a_sum[i + 1] = a_sum[i] + if s[i] == 'a' { 1 } else { 0 };
        b_sum[i + 1] = b_sum[i] + if s[i] == 'b' { 1 } else { 0 };
    }

    let mut ans = 0usize;

    // 各左端 l を固定して探索
    for l in 0..n {
        // 'a' 条件: a_sum[r] - a_sum[l] >= a_need を初めて満たす r
        let ar = lower_bound(l + 1, n + 1, |r| a_sum[r] - a_sum[l] >= a_need);

        // 'b' 条件: b_sum[r] - b_sum[l] >= b_limit を初めて満たす r
        let br = lower_bound(l + 1, n + 1, |r| b_sum[r] - b_sum[l] >= b_limit);

        if ar <= n {
            let end = if br == n + 1 { n } else { br - 1 };
            if end >= ar {
                ans += end - ar + 1;
            }
        }
    }

    println!("{}", ans);
}

/// 境界探索：条件を満たす最初の位置を返す
fn lower_bound<F>(mut lo: usize, mut hi: usize, f: F) -> usize
where
    F: Fn(usize) -> bool,
{
    while lo < hi {
        let mid = (lo + hi) / 2;
        if f(mid) {
            hi = mid;
        } else {
            lo = mid + 1;
        }
    }
    lo
}
