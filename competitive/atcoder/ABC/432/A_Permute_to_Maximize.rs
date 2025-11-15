use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let mut n: Vec<usize> = Vec::new();
    for _ in 0..3 {
        let a: usize = iter.next().unwrap().parse().unwrap();
        n.push(a);
    }
    // 降順にソート
    n.sort_by(|a, b| b.cmp(a));

    for nn in n {
        print!("{}", nn);
    }
}
