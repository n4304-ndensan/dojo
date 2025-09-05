# Time Machine

## 問題情報

- **Author:** Jeffery John
- **Description:**
  What was I last working on? I remember writing a note to help me remember...
  You can download the challenge files here:

  ```plaintext
  challenge.zip
  ```

---

## 解法

配布されたアーカイブを展開すると、`.git` ディレクトリと `message` ファイルが含まれていた。
`message` ファイルには「コミットメッセージを確認すれば何かがわかる」という趣旨の内容が記載されていた。

そこで `git log` コマンドを実行したところ、1 つのコミットが存在しており、そのコミットメッセージ中にフラグが記載されていた。

---

## 関連技術

- Git の基本操作

  - `git log` による履歴の確認
  - `.git` ディレクトリの解析

---
