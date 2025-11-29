# `strings`

## 概要

バイナリに埋め込まれた**可読文字列**を抽出。CTF の初手確認に最適。

## まずはこれ（クイック）

```bash
# 全体を対象、6文字以上、16進オフセット付きで閲覧
strings -a -n 6 -t x target.bin | less

# よくあるキーワードだけ素早く
strings -a target.bin | egrep -i "flag|pico|ctf|password|secret|token"
```

## 主なオプション（よく使う順）

* `-a, --all` : ファイル**全体**を対象に検索
* `-n N` : **N 文字以上**を対象（ノイズ減）
* `-t {x,d,o}` : 文字列の**オフセット**を表示（x=16進, d=10進, o=8進）
* `-e {l,b}` : **エンディアン指定**で広めに探索（`-el`=UTF-16LEっぽい、`-eb`=UTF-16BEっぽい）
* `-f, --print-file-name` : 出力行に**ファイル名を付ける**（複数ファイル時に便利）

### 例

```bash
# UTF-16LE の文字列も拾いたい
strings -a -n 6 -t x -el target.bin

# 見つかった位置をバイナリで再確認
xxd -s 0x6e0 -l 128 target.bin
```
