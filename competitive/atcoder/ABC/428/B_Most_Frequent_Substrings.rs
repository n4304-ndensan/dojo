use std::collections::HashMap;
use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let s: String = iter.next().unwrap().to_string();

    let mut map: HashMap<String, usize> = HashMap::new();

    // 長さKの部分文字列をすべて抽出してカウント
    for i in 0..=n - k {
        let sub = &s[i..i + k];
        *map.entry(sub.to_string()).or_insert(0) += 1;
    }

    // 出現回数の最大値を求める
    let max_count = map.values().copied().max().unwrap();

    // 出現回数が最大の部分文字列を辞書順にソート
    let mut result: Vec<_> = map
        .iter()
        .filter(|(_, &v)| v == max_count)
        .map(|(k, _)| k.clone())
        .collect();
    result.sort();

    // 出力
    println!("{}", max_count);
    println!("{}", result.join(" "));
}
