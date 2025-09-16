use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s: String = input.trim().to_string();
    let digits: Vec<i64> = s.chars().map(|c| c.to_digit(10).unwrap() as i64).collect();

    let mut sum = 0;
    let n = digits.len();
    for i in (0..n).rev() {
        let v = digits[i];
        let u = if i < n - 1 { digits[i + 1] } else { 0 };
        let b = (10 + v - u) % 10;
        sum += b;
    }

    let ans = sum + n as i64;
    println!("{}", ans);
}
