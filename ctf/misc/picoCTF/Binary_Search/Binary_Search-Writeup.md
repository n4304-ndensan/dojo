# Binary Search

## 問題情報

- **Author:** Jeffery John
- **Description:**
  Want to play a game? As you use more of the shell, you might be interested in how they work! Binary search is a classic algorithm used to quickly find an item in a sorted list. Can you find the flag? You'll have 1000 possibilities and only 10 guesses. Cyber security often has a huge amount of data to look through - from logs, vulnerability reports, and forensics. Practicing the fundamentals manually might help you in the future when you have to write your own tools!
  You can download the challenge files here: **challenge.zip**

  Additional details will be available after launching your challenge instance.

---

## 問題

0〜999 のランダムな整数を、シェル上で対話的に当てる。
試行回数は最大 10 回。

---

## 解法

各回答に対して「大きい／小さい」（higher/lower）のヒントが返るため、**二分探索**を用いれば 10 回以内に必ず当てられる。

- 初期範囲：`low = 0`, `high = 999`
- 毎回 `mid = (low + high) // 2` を答える
- 返答が **higher** なら `low = mid + 1`
  **lower** なら `high = mid - 1`
  **correct** で終了

根拠：`⌈log2(1000)⌉ = 10` なので、1000 通りは 10 回で探索可能。

---

## 手順（例）

1. チャレンジを起動し、指示に従ってシェルへ接続。
2. ゲーム（またはバイナリ）を実行。
3. 初手は `mid = 499` または `500` を入力。
4. 返答に応じて範囲を半分に絞り、合計 10 回以内に的中させる。
5. 正解するとフラグが表示される（出力形式は環境に依存）。

---

## 関連技術

- 二分探索（Binary Search）
- （環境によっては）JSON 出力からの抽出に `jq` を使用する場合あり

---
