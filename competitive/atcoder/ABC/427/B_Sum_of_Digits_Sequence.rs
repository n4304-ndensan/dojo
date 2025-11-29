use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut a: Vec<usize> = vec![0; n + 1];
    a[0] = 1;
    a[1] = 1;

    for i in 2..=n {
        a[i] = a[i-1] + digit_sum(a[i-1]);
    }
    println!("{}", a[n]);
}

fn digit_sum(mut x: usize) -> usize {
    let mut sum = 0;
    while x > 0 {
        sum += x % 10;
        x /= 10;
    }
    sum
}
