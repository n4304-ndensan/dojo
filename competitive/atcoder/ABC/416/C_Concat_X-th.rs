use std::io::{self, Read};

fn main() {
    // 入力全体を読み込み
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let k: usize = iter.next().unwrap().parse().unwrap();
    let x: usize = iter.next().unwrap().parse().unwrap();
    let s: Vec<String> = (0..n).map(|_| iter.next().unwrap().to_string()).collect();

    let mut ans = Vec::new();

    // 再帰DFS
    fn dfs(crr: String, count: usize, k: usize, s: &Vec<String>, ans: &mut Vec<String>) {
        if count == k {
            ans.push(crr);
            return;
        }
        for si in s {
            let mut next = crr.clone();
            next.push_str(si);
            dfs(next, count + 1, k, s, ans);
        }
    }

    dfs(String::new(), 0, k, &s, &mut ans);

    ans.sort();
    println!("{}", ans[x - 1]);
}
