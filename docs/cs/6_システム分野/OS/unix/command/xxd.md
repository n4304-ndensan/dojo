# `xxd` コマンド ドキュメント

`xxd` は Linux/Unix 系で使えるバイナリ⇔16進ダンプ変換ツール。CTF・リバースエンジニアリング・バイナリ解析でよく使う。Kali に標準搭載。

---

## 1. 基本概要

* `xxd` はファイルや標準入力を 16 進数表記（hexdump）に変換する。
* `-r` オプションを使うと 16 進数ダンプをバイナリに戻せる。
* 出力フォーマットやオフセットの表示を細かく制御できる。

---

## 2. 基本構文

```bash
xxd [options] [infile [outfile]]
```

* `infile`: 入力ファイル、省略すると標準入力を読む。
* `outfile`: 出力ファイル、省略すると標準出力に書く。

---

## 3. よく使うオプション

### 基本

* `xxd file.bin` → バイナリを 16 進数 + ASCII 表記で表示。
* `xxd -p file.bin` → 連続した 16進数だけを表示（ASCII 無し）。
* `xxd -r -p hex.txt` → プレーンな16進ダンプをバイナリに戻す。

### 出力制御

* `-c cols` : 1 行に表示するバイト数 (デフォルト 16)。

  ```bash
  xxd -c 8 file.bin
  ```
* `-g bytes` : グループ化サイズを指定。

  ```bash
  xxd -g 2 file.bin   # 2バイト単位で区切る
  ```
* `-s [+]seek` : 入力の読み取り開始位置を指定。

  ```bash
  xxd -s 0x100 file.bin  # 0x100 バイト目から表示
  ```
* `-l len` : 出力するバイト数を指定。

  ```bash
  xxd -l 64 file.bin   # 先頭64バイトだけ
  ```

### バイナリ復元

* `xxd -r` : 通常の `xxd` 出力をバイナリに戻す。
* `xxd -r -p` : `-p` 形式（連続16進数）の場合にバイナリに戻す。

### 例

```bash
# バイナリを hexdump してファイル保存
xxd file.bin > dump.txt

# dump から復元
xxd -r dump.txt > restored.bin

# プレーンな16進（例: 41424344 → ABCD）をバイナリ化
echo "41424344" | xxd -r -p
```

---

## 4. 使用例（CTF で役立つパターン）

### 文字コード変換の補助

```bash
# 数値列を 16進にして xxd -r -p で文字列に
awk '{for(i=1;i<=NF;i++) printf "%02x", $i; print ""}' nums.txt | xxd -r -p
```

### 部分抽出

```bash
# バイナリの 0x200 オフセットから 16 バイトだけ取り出す
xxd -s 0x200 -l 16 file.bin
```

### ファイル比較（md5sum と組み合わせ）

```bash
xxd file1.bin | md5sum
xxd file2.bin | md5sum
```

### バイナリパッチ

```bash
# hexdump をテキストエディタで修正 → xxd -r でバイナリに戻す
xxd file.bin > patch.txt
vim patch.txt
xxd -r patch.txt > patched.bin
```

---

## 5. 応用テクニック

### ネットワークとの組み合わせ

```bash
nc host port | xxd  # 通信データを hex 表示
```

### Base64 / Hex 相互変換

```bash
# Hex → Base64
echo "48656c6c6f" | xxd -r -p | base64

# Base64 → Hex
echo "SGVsbG8=" | base64 -d | xxd -p
```

### シェル芸

```bash
# ASCII 文字列を Hex に
printf "Hello" | xxd -p
# => 48656c6c6f

# Hex を ASCII に
echo 48656c6c6f | xxd -r -p
# => Hello
```

---

## 6. ガイドライン（基礎→応用の使い方）

### 基礎

* バイナリを読むときは **xxd file**。
* Hex 表記だけ欲しいなら **xxd -p file**。
* バイナリに戻すときは **xxd -r** または **xxd -r -p**。

### 中級

* 部分解析には **-s** (オフセット) と **-l** (長さ) を併用。
* **-c** (列数) や **-g** (グループ化) で読みやすく調整。

### 応用

* `awk`, `sed`, `tr` と組み合わせてパイプ処理。
* `nc` と組み合わせて通信解析。
* テキスト編集でパッチを作り `xxd -r` で反映。
* 他ツール (`base64`, `md5sum`, `strings`) と組み合わせて活用。

---

## まとめ

* `xxd` は **バイナリ↔16進数変換の基本ツール**。
* 基本3パターン: `xxd`, `xxd -p`, `xxd -r` を覚える。
* `-s`, `-l` で範囲指定、`-c`, `-g` で見やすさ調整。
* 応用では通信解析・パッチ作成・他コマンドとの組み合わせで強力に使える。
