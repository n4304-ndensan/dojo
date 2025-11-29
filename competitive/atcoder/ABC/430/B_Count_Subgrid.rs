use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    // n行n列の文字列を受け取る場合の例
    let s: Vec<Vec<char>> = (0..n)
        .map(|_| iter.next().unwrap().chars().collect())
        .collect();
    // set 集合
    let mut set = std::collections::HashSet::new();

    for i in 0..=(n - m) {
        for j in 0..=(n - m) {
            //　文字列連結してく箱
            let mut strinbuf = String::new();
            for x in i..(i + m) {
                for y in j..(j + m) {
                    if s[x][y] == '#' {
                        strinbuf.push('1');
                    } else {
                        strinbuf.push('0');
                    }
                }
            }
            set.insert(strinbuf);
        }
    }

    println!("{}", set.len());
}
