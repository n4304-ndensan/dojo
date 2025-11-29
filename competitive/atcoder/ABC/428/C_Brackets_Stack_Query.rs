use std::collections::VecDeque;
use std::io::{self, Read};

const MIN_INF: i32 = -1_000_000_000;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut stack: VecDeque<i32> = VecDeque::new();

    for _ in 0..n {
        let q: usize = iter.next().unwrap().parse().unwrap();
        if q == 1 {
            let x: char = iter.next().unwrap().chars().next().unwrap();
            if x == '(' {
                // 最後の値を参照する +1
                if let Some(&last) = stack.back() {
                    stack.push_back(last + 1);
                } else {
                    stack.push_back(1);
                }
            } else {
                if let Some(&last) = stack.back() {
                    if last == 0 {
                        stack.push_back(MIN_INF);
                    } else {
                        stack.push_back(last -1);
                    }
                } else {
                    stack.push_back(MIN_INF);
                }
            }
        }
        else {
            // 削除
            stack.pop_back();
        }

        if let Some(&last) = stack.back() {
            if last == 0 {
                println!("Yes");
            } else {
                println!("No");
            }
        } else {
            println!("Yes");
        }
    }
}