# Rust_fixme_3

## 問題情報

- **Author:** TAYLOR MCCAMPBELL
- **Description:**  
  Have you heard of Rust? Fix the syntax errors in this Rust file to print the flag!  
  Download the Rust code here.

---

## 解法

1. 配布された Rust コードには `std::slice::from_raw_parts` を使う箇所があり、  
   これは **unsafe 関数** であるため通常のままではコンパイルエラーになる。
2. Rust では、生ポインタを参照外しするときに無効なポインタであれば **未定義動作 (UB)** になる危険があるため、  
   この操作は **`unsafe` ブロック** の中でしか許されない。
3. 今回は `Vec<u8>` から `as_ptr()` と `len()` を取得しており、必ず有効な領域であることが保証されている。  
   そのため `unsafe` ブロックで `from_raw_parts` を呼び出すことで安全にスライスを作成できる。
4. 復号処理を通して flag が出力された。

---

## 関連技術

- `Rust unsafe`（`unsafe` ブロック、参照外し）
- `Vec<u8>` と `&[u8]`（スライス）
- `String::from_utf8_lossy` によるバイト列から文字列への変換
