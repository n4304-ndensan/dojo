# useless Writeup

## 概要

この writeup は、`useless` スクリプトの解析手順と発見したフラグをまとめたものです。環境は picoCTF のチャレンジ環境（`saturn.picoctf.net:49748`）で、ユーザー `picoplayer` として操作しました。

---

## 実行と観察

1. スクリプトの中身を `cat useless` で確認した。先頭に `#!/bin/bash` があり、`[[ ... ]]` を使う Bash 専用の構文であることを確認。
2. `sh useless ...` で実行すると `/bin/sh`（dash）が用いられ、`[[` がサポートされずエラーが出ることを確認した（`[[: not found`）。
3. `bash useless add 4 5` や `chmod +x useless` の後 `./useless mul 5 5` として実行すると期待どおり計算結果が得られることを確認した。

---

## マニュアルの確認

* `man useless` を実行したところ、スクリプトの説明（SYNOPSIS / DESCRIPTION / Examples / Authors）が表示された。
* `man` ページの Authors セクションにフラグが記載されており、これが今回の課題で求められるフラグであると判断した。

---

## 発見したフラグ

```
picoCTF{us3l3ss_ch4ll3ng3_3xpl0it3d_5136}
```

---

## 考察

* 今回の問題はスクリプト本体だけでなく、付随するマニュアル（`man` ページ）や補助ドキュメントにフラグが隠されているパターンである。
* `sh` と `bash` の違い（特に `[[` と `]]` のサポート）を把握していることが重要。
* `strings` コマンドが利用できない環境もあるため、まずは `cat` や `man`、`grep` 等でテキストを探索するのが有効である。

---
