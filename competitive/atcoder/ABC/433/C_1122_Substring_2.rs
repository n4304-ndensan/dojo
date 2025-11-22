use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let num_str = iter.next().unwrap().to_string();

    let digits: Vec<usize> = num_str
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut a: Vec<usize> = Vec::new();
    for i in 0..(digits.len() - 1) {
        if digits[i] + 1 == digits[i + 1] {
            a.push(i + 1);
        }
    }

    let mut ans = 0;
    for ai in a {
        let mut left: isize = ai as isize - 1;
        let mut right: isize = ai as isize;

        let original_left = digits[left as usize];
        let original_right = digits[right as usize];
        while left >= 0
            && right < digits.len() as isize
            && digits[left as usize] == original_left
            && digits[right as usize] == original_right
        {
            ans += 1;
            left -= 1;
            right += 1;
        }
    }

    println!("{}", ans);
}
