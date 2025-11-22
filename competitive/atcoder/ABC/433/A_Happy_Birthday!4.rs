use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let x: i32 = iter.next().unwrap().parse().unwrap();
    let y: i32 = iter.next().unwrap().parse().unwrap();
    let z: i32 = iter.next().unwrap().parse().unwrap();

    if (x - z * y) < 0 {
        println!("No");
        return;
    }

    let ans = (x - z * y) % (z - 1) == 0;

    if ans {
        println!("Yes");
    } else {
        println!("No");
    }
}
