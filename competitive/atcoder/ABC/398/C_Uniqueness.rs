use std::io::{self, Read};
use std::collections::{HashMap, HashSet};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let mut m: HashMap<usize, usize> = HashMap::new();
    let mut s: HashSet<usize> = HashSet::new();
    for i in 0..n {
        let a: usize = iter.next().unwrap().parse().unwrap();
        if s.contains(&a) {
            m.remove(&a);
        }
        else {
            s.insert(a);
            m.insert(a, i+1);
        }
    }

    if m.len() == 0 {
        println!("{}", -1);
        return;
    }

    let mut max = 0;
    for &k in m.keys() {
        if k > max {
            max = k;
        }
    } 

    let ans = m[&max];

    println!("{}", ans);
}