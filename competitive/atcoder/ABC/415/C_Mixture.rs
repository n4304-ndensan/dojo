use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let t: usize = iter.next().unwrap().parse().unwrap();

    for _ in 0..t {
        let n: usize = iter.next().unwrap().parse().unwrap();
        let s: String = "0".to_string() + iter.next().unwrap();
        let si = s.chars().collect::<Vec<char>>();

        let mut ok = vec![false; 1 << n];
        ok[0] = true;
        for i in 0..(1 << n) {
            if !ok[i] {
                continue;
            }
            for j in 0..n {
                if (i & (1 << j)) != 0 {
                    continue;
                }
                let next = i | (1 << j);
                if si[next] == '0' {
                    ok[next] = true;
                }
            }
        }
        if ok[(1 << n) - 1] {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
