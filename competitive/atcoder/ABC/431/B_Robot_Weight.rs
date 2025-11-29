use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let x: usize = iter.next().unwrap().parse().unwrap();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut w: Vec<usize> = Vec::new();
    let mut ww: Vec<usize> = vec![0; n + 1];
    ww[0] = x;
    w.push(x);
    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        w.push(a);
    }

    let q: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..q {
        let p: usize = iter.next().unwrap().parse().unwrap();
        if ww[p] == 0 {
            ww[p] = w[p];
        } else {
            ww[p] = 0;
        }
        println!("{}", ww.iter().sum::<usize>());
    }
}
