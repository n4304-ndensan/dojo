use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let r: usize = iter.next().unwrap().parse().unwrap();

    let s: Vec<usize> = (0..n).map(|_| iter.next().unwrap().parse().unwrap()).collect();

    let mut left = n;
    let mut right = 0;

    for i in 0..n {
        if s[i] == 0 {
            left = i;
            break;
        }
    }

    for i in (0..n).rev() {
        if s[i] == 0 {
            right = i;
            break;
        }
    }

    let mut open = 0;
    for i in 0..n {
        if s[i] == 0 {
            open += 1;
        }
    }

    let mut close = 0;
    if left <= r - 1 {
        for i in (left + 1)..r {
            if s[i] == 1 {
                close += 1;
            }
        }
    }

    if right >= r {
        for i in r..right {
            if s[i] == 1 {
                close += 1;
            }
        }
    }


    let ans = 2 * close + open;

    println!("{}", ans);
}
