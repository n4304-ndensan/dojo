use std::io::{self, Read};

/// 文字列が回文かどうか判定
fn is_palindrome(s: &str) -> bool {
    let chars: Vec<char> = s.chars().collect();
    let len = chars.len();
    for i in 0..len / 2 {
        if chars[i] != chars[len - 1 - i] {
            return false;
        }
    }
    true
}

/// 整数を任意基数の文字列表現に変換
fn to_base_n(mut number: u64, base: u32) -> String {
    if number == 0 {
        return "0".to_string();
    }

    let digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut result = String::new();

    while number > 0 {
        let rem = (number % base as u64) as usize;
        result.push(digits.chars().nth(rem).unwrap());
        number /= base as u64;
    }

    result.chars().rev().collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let a: u32 = iter.next().unwrap().parse().unwrap(); // 基数
    let n: u64 = iter.next().unwrap().parse().unwrap(); // 上限

    let mut ans: i64 = 0;
    let n_str = n.to_string(); // 桁数比較用

    let mut len = 1;
    loop {
        let half_len = (len + 1) / 2;
        let start = 10u64.pow((half_len - 1) as u32);
        let end = 10u64.pow(half_len as u32);

        if start > n {
            break; // これ以上大きな回文は n を超えるので終了
        }

        for half in start..end {
            let mut s = half.to_string();
            let rev: String = if len % 2 == 0 {
                s.chars().rev().collect()
            } else {
                s.chars().rev().skip(1).collect()
            };
            s.push_str(&rev);

            // 桁数チェック → n より大きな桁数なら終了
            if s.len() > n_str.len() {
                break;
            }
            // 桁数が同じなら文字列比較で n を超えていないか判定
            if s.len() == n_str.len() && s > n_str {
                break;
            }

            let pal = s.parse::<u64>().unwrap();

            // 基数 a で回文チェック
            let base_str = to_base_n(pal, a);
            if is_palindrome(&base_str) {
                ans += pal as i64;
            }
        }
        len += 1;
    }

    println!("{}", ans);
}
