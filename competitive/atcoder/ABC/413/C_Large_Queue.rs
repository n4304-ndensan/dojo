use std::collections::VecDeque;
use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let q: usize = iter.next().unwrap().parse().unwrap();
    let mut queue: VecDeque<(i64, i64)> = VecDeque::new();

    for _ in 0..q {
        let type_num: u8 = iter.next().unwrap().parse().unwrap();
        if type_num == 1 {
            let count: i64 = iter.next().unwrap().parse().unwrap();
            let number: i64 = iter.next().unwrap().parse().unwrap();
            queue.push_back((number, count));
        } else {
            let k: i64 = iter.next().unwrap().parse().unwrap();
            let mut sum = 0;
            let mut remain = k;
            while remain > 0 {
                if let Some((n, cnt)) = queue.front_mut() {
                    let take = remain.min(*cnt);
                    sum += *n * take;
                    *cnt -= take;
                    remain -= take;
                    if *cnt == 0 {
                        queue.pop_front();
                    }
                } else {
                    break;
                }
            }
            println!("{}", sum);
        }
    }
}
