# Rust_fixme_2

## 問題情報

- **Author:** TAYLOR MCCAMPBELL
- **Description:**  
  The Rust saga continues?  
  I ask you, can I borrow that, pleeeeeaaaasseeeee?  
  Download the Rust code here.

---

## 解法

1. 配布された Rust コードでは、`decrypt` 関数の第二引数 `borrowed_string` が `&String`（不変参照）として宣言されていた。
2. しかし関数内部で `push_str` を用いて `borrowed_string` を変更しようとしており、不変参照ではコンパイルエラーになる。
3. 修正方法は **引数を `&mut String` に変更**し、呼び出し側も `&mut` を付けて渡すこと。  
   また、`borrowed_string` を宣言する際に `mut` を付ける必要がある。
4. この修正により、可変参照として安全に文字列を更新でき、プログラムが正しくコンパイル・実行される。

---

## 関連技術

- **可変参照 (`&mut`)**  
  Rust ではデフォルトが不変参照。値を変更するには可変参照を明示する必要がある。
- **所有権と借用**
  - 所有権：変数が値を完全に持つ
  - 不変参照：読み取り専用で借りる
  - 可変参照：書き換え可能に借りる（ただし同時に 1 つだけ）

---
