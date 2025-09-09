# Blame Game

## 問題情報

* **Author:** Jeffery John
* **Description:**
  Someone's commits seem to be preventing the program from working. Who is it?
  You can download the challenge files here:

  ```
  challenge.zip
  ```

---

## 解法

`git` で管理されているフォルダ内に **`message.py`** が存在していた。
このファイルの履歴を調べると、あるコミットがプログラムの動作を妨げていることが分かった。

`git blame message.py` を実行すると、問題の行を編集した **コミットユーザー名** が特定できた。
このユーザー名が **flag** となっていた。

---

## 関連技術

* Git
* `git blame` コマンドによる履歴調査

---