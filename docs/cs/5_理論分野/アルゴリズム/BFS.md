# BFS（Breadth-First Search：幅優先探索）

## 概要

**BFS（幅優先探索）**は、グラフ探索の基本アルゴリズムであり、探索を**層ごと（距離順）**に行う。キュー構造を用いる。

## 特徴

- 最短経路を求める際に有効（無重みグラフ）。
    
- 各頂点を1回ずつ訪問する。
    

## 擬似コード

```pseudo
BFS(s):
    queue = [s]
    visited[s] = true
    dist[s] = 0
    while queue not empty:
        v = queue.pop_front()
        for u in G[v]:
            if not visited[u]:
                visited[u] = true
                dist[u] = dist[v] + 1
                queue.push_back(u)
```

## 用途

- 無重み最短路探索
    
- 連結成分数の計算
    
- 二部グラフ判定
    
- 木の距離計算
    

## 計算量

- 時間計算量：O(V+E)O(V + E)
    
- 空間計算量：O(V)O(V)
    

## 例

```
1 — 2 — 3
|         \
4 — 5 ————
```

BFS(1): 1 → 2 → 4 → 3 → 5

## 注意

- 再帰ではなく**キューで実装**する。
    
- 隣接行列よりも**隣接リスト表現**が高速。