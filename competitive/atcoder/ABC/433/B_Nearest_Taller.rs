use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = Vec::new();
    for _ in 0..n {
        let ai: usize = iter.next().unwrap().parse().unwrap();
        a.push(ai);
    }

    println!("-1");
    for i in 1..n {
        let mut found = -1;
        for j in 0..i {
            if a[i] < a[j] {
                found = (j + 1) as isize;
            }
        }
        println!("{} ", found);
    }
}
