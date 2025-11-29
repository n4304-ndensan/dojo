use std::io::{self, Read};

fn isqrt(n: u128) -> u128 {
    let (mut lo, mut hi) = (0u128, 1u128);
    // 上限を指数探索で広げる
    while hi.saturating_mul(hi) <= n {
        hi <<= 1;
    }
    lo = hi >> 1;
    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        if mid.saturating_mul(mid) <= n {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    lo
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let n: u128 = input.trim().parse().unwrap();

    let ans = isqrt(n / 2) + isqrt(n / 4);
    println!("{}", ans);
}
