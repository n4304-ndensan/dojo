use std::io::{self, Read};

fn main() {
    // 標準入力の読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    // 分割
    let mut vec_n: Vec<usize> = n
        .to_string()
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();

    vec_n.sort();

    for i in 0..vec_n.len() {
        if vec_n[i] != 0 {
            let temp = vec_n.remove(i);
            vec_n.insert(0, temp);
            break;
        }
    }

    for nn in vec_n {
        print!("{}", nn);
    }
}
