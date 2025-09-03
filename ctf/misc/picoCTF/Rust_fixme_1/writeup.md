# Rust_fixme_1

## 問題情報

- **Author:** TAYLOR MCCAMPBELL
- **Description:**  
  Have you heard of Rust? Fix the syntax errors in this Rust file to print the flag!  
  Download the Rust code here.

---

## 解法

配布された Rust ファイルは複数の **構文エラー** によりコンパイルが通らなかった。  
以下の点を修正することで、正常にコンパイル・実行ができるようになった。

1. **文末にセミコロンが必要**

   - `let key = String::from("CSUCKS")`  
     → `let key = String::from("CSUCKS");`

2. **不要な記述の修正**

   - `re;`  
     → `return;`

3. **`println!` のフォーマット指定の誤り**

   - `println!(":?", var);`  
     → `println!("{:?}", var);`

   Rust の `println!` マクロでは、値を表示する場合に

   - `{} ` : Display トレイト
   - `{:?}` : Debug トレイト  
     を使う必要がある。

4. **全体の修正例**
   ```rust
   println!(
       "{:?}", // How do we print out a variable in the println function?
       String::from_utf8_lossy(&decrypted_buffer)
   );
   ```

---

## 関連技術

- **Rust の文末セミコロン**（式と文の区別）
- **`return` の書き方**（Rust では `return;` または式をそのまま返す）
- **`println!` マクロのフォーマット指定**

  - `{}` : 通常表示（Display）
  - `{:?}` : デバッグ表示（Debug）

---
