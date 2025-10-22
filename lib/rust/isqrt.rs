// n の平方根の「切り捨て値（floor）」を整数で返す関数
fn isqrt(n: u128) -> u128 {
    let (mut lo, mut hi) = (0u128, 1u128);
    // √n を超える最小の 2^k を探す
    while hi.saturating_mul(hi) <= n {
        hi <<= 1;
    }
    // 探索開始区間を [hi/2, hi) にする
    lo = hi >> 1;
    // 二分探索
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
    println("{}", isqrt(9)); // 3
    println("{}", isqrt(10)); // 3
    println("{}", isqrt(15)); // 3
    println("{}", isqrt(16)); // 4
    println("{}", isqrt(17)); // 4
}