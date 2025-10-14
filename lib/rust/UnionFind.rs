#[derive(Debug)]
struct UnionFind {
    parent: Vec<usize>, // 各ノードの親を格納
    size: Vec<usize>,   // 各集合のサイズ（木の高さを抑えるため）
}

impl UnionFind {
    /// n個の要素で初期化
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(), // 自分自身が親
            size: vec![1; n],         // 初期サイズは1
        }
    }

    /// 要素xの根を返す（経路圧縮付き）
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            // 再帰的に親をたどり、経路圧縮する
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    /// xとyを同じ集合にする（union by size）
    fn unite(&mut self, x: usize, y: usize) {
        let mut root_x = self.find(x);
        let mut root_y = self.find(y);
        if root_x == root_y {
            return; // すでに同じ集合
        }

        // 小さい木を大きい木にぶら下げる
        if self.size[root_x] < self.size[root_y] {
            std::mem::swap(&mut root_x, &mut root_y);
        }

        self.parent[root_y] = root_x;
        self.size[root_x] += self.size[root_y];
    }

    /// xとyが同じ集合に属するか判定
    fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    /// 集合のサイズを取得
    fn get_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root]
    }
}

fn main() {
    let mut uf = UnionFind::new(6);

    uf.unite(0, 1);
    uf.unite(1, 2);
    uf.unite(3, 4);

    println!("0と2は同じグループ？ {}", uf.same(0, 2)); // true
    println!("0と3は同じグループ？ {}", uf.same(0, 3)); // false
    println!("グループサイズ(0): {}", uf.get_size(0));   // 3
    println!("グループサイズ(3): {}", uf.get_size(3));   // 2
}
