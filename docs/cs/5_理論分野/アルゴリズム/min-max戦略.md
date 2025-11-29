# min-max戦略（Minimax Strategy）

## 概要

**min-max戦略**は、主に**二人零和ゲーム（Zero-sum Game）**で用いられる探索・意思決定アルゴリズムである。相手が最善手を取ることを前提に、自分の損失を最小化（min）しつつ、利益を最大化（max）する戦略を選択する。

## 基本原理

- 自分：最大化プレイヤー（Max player）
    
- 相手：最小化プレイヤー（Min player）
    
- 末端ノードに評価値を設定し、木を逆方向にたどって最善手を決定する。
    

### 例：ゲーム木探索

```pseudo
def minimax(node, depth, isMax):
    if node is terminal or depth == 0:
        return evaluate(node)

    if isMax:
        best = -INF
        for child in node.children:
            best = max(best, minimax(child, depth-1, False))
        return best
    else:
        best = INF
        for child in node.children:
            best = min(best, minimax(child, depth-1, True))
        return best
```

## 改良手法

- **α-β枝刈り（Alpha-Beta Pruning）**：不要な探索を省略して計算量を削減（平均的に O(bd/2)O(b^{d/2}) に短縮）。
    
- **深さ制限 + 評価関数**：全探索が困難な場合に近似評価を導入。
    

## 応用例

- オセロ、チェス、将棋、三目並べなどのAI戦略。
    
- 経済モデル、戦略的意思決定の最悪シナリオ分析。
    

## 計算量

- 深さ dd、分岐数 bb の場合：O(bd)O(b^d)
    
- α-β枝刈りを使えば探索木の大部分をスキップ可能。