#CTF #picoCTF #Binary #ProcessHandling

# plumbing Writeup

## 概要

本問題は、標準出力へ大量の不要出力を行うプログラムに対して、外部からその出力をパイプ処理し、フラグを抽出する課題である。プロセス出力をファイルに落とさずに検索する必要がある点が特徴である。

## 問題文要点

* 実行サーバへ `nc <host> <port>` で接続すると、膨大な "Not a flag" 系の文字列が標準出力に流れる。
* その中にフラグが混在している。
* ヒントとして「Sometimes you need to handle process data outside of a file」が示されており、ファイル保存せずにパイプ処理を行うことが意図されている。

## 解法

### パイプと `grep` の利用

標準出力をリアルタイムにフィルタリングするため、以下のようにパイプと `grep` を使用することで、出力を保存せずにフラグ部分のみを抽出できる。

```bash
nc fickle-tempest.picoctf.net 58620 | grep picoCTF
```

### 得られたフラグ

```
picoCTF{digital_plumb3r_0BAc587E}
```

## 使用技術ポイント

* パイプによるプロセス間標準出力の接続
* `grep` によるフィルタリング
* ファイルを介さずにストリームデータ処理を行う UNIX 哲学の基本要素

## まとめ

| 項目   | 内容                                  |
| ---- | ----------------------------------- |
| 分類   | ストリーム処理 / UNIX 基本操作                 |
| 主な操作 | パイプ、`grep`                          |
| 難易度  | 低                                   |
| フラグ  | `picoCTF{digital_plumb3r_0BAc587E}` |
