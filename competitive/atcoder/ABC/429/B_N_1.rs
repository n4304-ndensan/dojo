use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut arr: Vec<usize> = Vec::with_capacity(n);

    for _ in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        arr.push(a);
    }

    let sum: usize = arr.iter().sum();
    let mut ok = false;

    for &a in &arr {
        if sum - a == m {
            ok = true;
            break;
        }
    }

    if ok {
        println!("Yes");
    } else {
        println!("No");
    }
}
