# PW Crack 1 - writeup

## 問題
- **Author:** LT 'syreal' Jones  
- **Description:**  
    Can you crack the password to get the flag?
    Download the password checker here and you'll need the 
    encrypted flag in the same directory too.

---

## 解法

1. **ファイルの確認**  
   提供された `python` プログラムと暗号化ファイルを確認。  
   プログラムを読むと、入力された文字列と暗号化ファイルの内容に対して **XOR（排他的論理和）** を取ることで復号処理をしていることが分かった。

2. **特定の入力文字列**  
   コード内の `if` 文で、特定の文字列を入力したときに復号処理に入る仕組みになっていた。  
   その文字列は **8713** で記述されており、人間がそのまま入力できる形式であった。

出力された文字列をパスワードとしてプログラムに入力すると、暗号化ファイルが復号されフラグが得られた。

---

## 学び・関連技術

* **XOR (排他的論理和)**

  * 「同じなら0、違えば1」となる演算。
  * 暗号や難読化でよく使われ、`cipher = plain ⊕ key` で暗号化、`cipher ⊕ key = plain` で復号できる。

---
