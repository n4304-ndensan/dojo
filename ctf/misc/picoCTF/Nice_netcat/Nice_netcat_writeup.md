# Nice_netcat Writeup (picoCTF)

## 概要

* **問題名**: Nice netcat
* **作者**: syreal
* **接続先**: `nc mercury.picoctf.net 22342`
* **状況**: サーバが数値列を返し、UTF-8として解釈すると意味のある文字列が得られる。英語ではないが、数値→文字列に変換すれば読み取れる。

---

## 目標

サーバから得られた「数字列」を適切にバイト/コードポイントとしてUTF-8にデコードして可読なテキストにする。汎用的に同様の問題に対応できる手順を残す。

---

## 環境

* Kali / Debian 系 Linux
* 標準コマンド: `awk`, `xxd`, `perl`, `python3`, `nc` (netcat)

---

## 解法（要点）

1. 受け取った数字列が **バイト列(0–255)** を表すか、**Unicodeコードポイント** を表すかを判別する。

   * 例: `72 101 108 108 111` はバイト列 → "Hello"。
   * 例: `12354` のように 255 を超える数字があればコードポイントとして扱う。
2. バイト列なら `pack` 相当でバイナリに変換し、そのバイナリを UTF-8 として表示する。
3. コードポイントなら `chr()` を使って各コードポイントを文字に変換して結合する。

---

## 実際に使ったコマンド

### 1) あなたが提示した方法（スペース区切りのバイト列をそのまま表示）

```bash
# 入力をスペース区切りで渡す
echo "72 101 108 108 111" \
  | awk '{for(i=1;i<=NF;i++) printf "%02x", $i; print ""}' \
  | xxd -r -p
```

* 解説: `awk` で各数値を 2 桁 16 進に変換し（`%02x`）、連結した 16 進文字列を `xxd -r -p` でバイナリに戻して出力する。

### 2) 汎用的な Python ワンライナー（バイト列として扱う場合）

```bash
python3 -c 'import sys; b=bytes(map(int,sys.stdin.read().split())); print(b.decode("utf-8",errors="strict"))' <<< "72 101 108 108 111"
```

* `errors="strict"` は不正な UTF-8 バイトがあれば例外にする。`replace` にすれば置換表示される。

### 3) Python（Unicode コードポイントとして扱う場合）

```bash
python3 -c 'import sys; print("".join(chr(int(x)) for x in sys.stdin.read().split()))' <<< "12354 12418"
```

### 4) Perl（バイト列）

```bash
perl -e 'print pack("C*", split(/ +/, <STDIN>))' <<< "72 101 108 108 111"
```

### 5) netcat とパイプで接続する例

```bash
nc mercury.picoctf.net 22342 | awk '{for(i=1;i<=NF;i++) printf "%02x", $i; print ""}' | xxd -r -p
```

または

```bash
nc mercury.picoctf.net 22342 | python3 -c 'import sys; b=bytes(map(int,sys.stdin.read().split())); print(b.decode("utf-8",errors="replace"))'
```

---

## 補足・トラブルシュート

* **数値に 256 以上が含まれる場合**: バイト変換（0–255）を使うとエラーになる。コードポイント（`chr`）で扱う。
* **改行・カンマ・カッコが混じる場合**: `tr -d '(),'` や `sed
