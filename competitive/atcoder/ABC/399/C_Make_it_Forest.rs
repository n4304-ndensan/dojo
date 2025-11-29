use std::io::{self, Read};
use std::collections::HashSet;

#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>, // 各ノードの親を格納
    size: Vec<usize>,   // 各集合のサイズ（木の高さを抑えるため）
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n], 
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return; 
        }

        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
    }

    fn count_groups(&mut self) -> usize {
        let mut roots = HashSet::new();
        for i in 0..self.parent.len() {
            roots.insert(self.find(i));
        }
        roots.len()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut iter = input.split_whitespace();

    let n: usize = iter.next().unwrap().parse().unwrap();
    let m: usize = iter.next().unwrap().parse().unwrap();

    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        let u: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        let v: usize = iter.next().unwrap().parse::<usize>().unwrap() - 1;
        uf.unite(u, v);
    }

    let groups = uf.count_groups();
    let ans = m - (n - groups);

    println!("{}", ans);
}
