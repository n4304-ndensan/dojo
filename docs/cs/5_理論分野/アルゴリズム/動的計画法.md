# DP（Dynamic Programming：動的計画法）

## 概要

**動的計画法（Dynamic Programming, DP）** は、問題を部分問題に分解し、その結果を再利用して効率的に最適解を求める手法。

## 基本原理

1. **最適部分構造 (Optimal Substructure)**：部分問題の最適解から全体最適解が導ける。
    
2. **重複部分問題 (Overlapping Subproblems)**：同じ部分問題を何度も解く必要がある。
    

これらを満たすとき、DPが有効。

## 実装パターン

### 1. メモ化再帰（トップダウン）

```python
from functools import lru_cache

@lru_cache(None)
def f(n):
    if n <= 1:
        return 1
    return f(n-1) + f(n-2)
```

### 2. テーブル法（ボトムアップ）

```python
dp = [0]*(n+1)
dp[0], dp[1] = 1, 1
for i in range(2, n+1):
    dp[i] = dp[i-1] + dp[i-2]
```

## 応用分野

- ナップサック問題
    
- 最長共通部分列（LCS）
    
- 区間DP・木DP・bitDP
    
- 経路探索・コスト最小化
    

## 計算量

- 通常：O(N)O(N) ～ O(N2)O(N^2)
    
- 状態・遷移の設計で高速化可能。
    

## 注意点

- 配列初期化・遷移式の境界条件に注意。
    
- メモリ制約が厳しい場合は**ローリング配列**で最適化可能。