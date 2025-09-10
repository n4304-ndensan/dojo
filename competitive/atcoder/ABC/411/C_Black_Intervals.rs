use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();

    // マスの色を管理する配列（0:白, 1:黒）
    // 番兵として 0 と N+1 も追加しておく
    let mut a = vec![0; n + 2];
    let mut diff_count = 0; // 色が異なる境目の数

    for _ in 0..q {
        let x: usize = iter.next().unwrap().parse().unwrap();

        // 左側の境界 x-1 と x の関係を更新
        if a[x - 1] != a[x] {
            diff_count -= 1;
        } else {
            diff_count += 1;
        }

        // 右側の境界 x と x+1 の関係を更新
        if a[x] != a[x + 1] {
            diff_count -= 1;
        } else {
            diff_count += 1;
        }

        // マス x を反転
        a[x] ^= 1;

        // 黒区間の数 = diff_count / 2
        println!("{}", diff_count / 2);
    }
}
