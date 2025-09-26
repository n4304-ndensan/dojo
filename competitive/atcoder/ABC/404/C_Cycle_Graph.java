import java.util.*;

public class Main {
    public static void main(String[] args) {
        Scanner sc = new Scanner(System.in);
        int n = sc.nextInt();
        int m = sc.nextInt();

        Map<Integer, Integer> nodeMap = new HashMap<>();
        for (int i = 1; i <= n; i++) {
            nodeMap.put(i, 0);
        }

        UnionFind uf = new UnionFind(n);

        for (int i = 0; i < m; i++) {
            int source = sc.nextInt();
            int dest = sc.nextInt();
            nodeMap.put(source, nodeMap.getOrDefault(source, 0) + 1);
            nodeMap.put(dest, nodeMap.getOrDefault(dest, 0) + 1);
            uf.union(source, dest);
        }

        boolean ans = true;

        // 各ノードの次数が2であるかチェック
        for (int i = 1; i <= n; i++) {
            if (nodeMap.getOrDefault(i, 0) != 2) {
                ans = false;
                break;
            }
        }

        // 連結かどうかを確認（すべて同じ親か？）
        if (ans) {
            int root = uf.find(1);
            for (int i = 2; i <= n; i++) {
                if (uf.find(i) != root) {
                    ans = false;
                    break;
                }
            }
        }

        System.out.println(ans ? "Yes" : "No");
    }
}

class UnionFind {
    private int[] parent, rank;

    UnionFind(int n) {
        parent = new int[n + 1];
        rank = new int[n + 1];
        for (int i = 1; i <= n; i++) {
            parent[i] = i;
            rank[i] = 0;
        }
    }

    int find(int x) {
        if (parent[x] != x) {
            parent[x] = find(parent[x]);
        }
        return parent[x];
    }

    void union(int x, int y) {
        int rx = find(x), ry = find(y);
        if (rx == ry) return;
        if (rank[rx] < rank[ry]) {
            parent[rx] = ry;
        } else if (rank[rx] > rank[ry]) {
            parent[ry] = rx;
        } else {
            parent[ry] = rx;
            rank[rx]++;
        }
    }

    boolean connected(int x, int y) {
        return find(x) == find(y);
    }
}
