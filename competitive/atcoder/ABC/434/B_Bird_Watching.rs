use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut dic: Vec<Vec<f64>> = vec![vec![0.0; 2]; m];
    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: f64 = iter.next().unwrap().parse().unwrap();
        dic[a][0] += 1.0;
        dic[a][1] += b;
    }

    for i in 0..m {
        let ans: f64 = dic[i][1] / dic[i][0];
        println!("{:.10}", ans);
    }
}