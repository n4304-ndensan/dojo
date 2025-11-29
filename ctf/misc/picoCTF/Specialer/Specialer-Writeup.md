# Specialer - Writeup

## 概要

**Challenge Name:** Specialer
**Author:** LT 'syreal' Jones, et al.
**Category:** General Skills / Shell Environment
**Difficulty:** Easy
**CTF Platform:** picoCTF
**Connection Info:**

```
ssh -p 61263 ctf-player@saturn.picoctf.net
Password: af86add3
```

---

## 説明

「Specialer」は、`Special` というシェル環境の改良（という名の制限版）をテーマにした問題である。通常のLinuxコマンドが利用できず、非常に制限された環境でフラグを探索することが目的である。

---

## 調査過程

### 1. 接続

まずSSHで接続する：

```bash
ssh -p 61263 ctf-player@saturn.picoctf.net
```

パスワードを入力してログインすると、プロンプトが `Specialer$` となる。

### 2. 環境の確認

`ls`, `dir`, `cat` などの基本的なコマンドが使用不能である：

```
-bash: ls: command not found
-bash: cat: command not found
```

`pwd` は使用可能であり、ルート (`/`) にいることが分かる。

```
Specialer$ pwd
/
```

`cd` は使用可能であるため、手動でディレクトリを探索することにした。

---

## 3. ディレクトリ探索

ホームディレクトリに移動：

```
Specialer$ cd home/ctf-player/
```

中身を確認（`ls`が使えないため後述の代替策を使用）：

```
.hushlogin  .profile  abra/  ala/  sim/
```

`.profile` に環境設定がある：

```bash
export PS1='Specialer$ '
```

---

## 4. コマンド代替策の構築

### `ls` の代用

シェルの `alias` 機能が有効なため、次のように定義する：

```bash
alias ls='printf "%s\n" .* && printf "%s\n" * '
```

これにより、カレントディレクトリの内容を確認できるようになった。

### `cat` の代用

同様に、`cat` コマンドの代用を定義する：

```bash
alias cat='while IFS= read -r line || [[ -n $line ]]; do echo "$line"; done < '
```

この定義により、ファイル内容を出力できるようになった。

---

## 5. 各ディレクトリの確認

### `abra/`

```
cadabra.txt
cadaniel.txt
```

中身を確認したが、アクセスできなかった。

### `ala/`

```
kazam.txt
mode.txt
```

`kazam.txt` を確認すると、フラグが含まれていた：

```bash
Specialer$ cat kazam.txt
return 0 picoCTF{y0u_d0n7_4ppr3c1473_wh47_w3r3_d01ng_h3r3_a8567b6f}
```

### `sim/`

確認不要（フラグは既に発見）。

---

## 6. まとめ

| 項目     | 内容                                             |
| ------ | ---------------------------------------------- |
| コマンド制限 | `ls`, `cat`, `man` などの基本コマンドが無効                |
| 代替策    | `alias` による再定義で回避                              |
| フラグ位置  | `/home/ctf-player/ala/kazam.txt`               |
| 学習ポイント | 制限環境下でのシェル機能（`alias`, `while read`, `echo`）の活用 |

---

## 考察

本問題は、シェル環境の基本的な仕組みを理解していなければ突破できないタイプのCTF課題である。
「標準的なコマンドが封じられた環境でも、Bash の組込み機能を駆使すればファイル操作が可能である」ことを示す好例となっている。

---
