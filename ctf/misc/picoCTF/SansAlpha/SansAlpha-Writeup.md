# SansAlpha - WriteUp

**Author:** syreal  
**Category:** General Skills  
**Points:** 400pts  
**Challenge Description:**  
> The Multiverse is within your grasp!  
> Unfortunately, the server that contains the secrets of the multiverse is in a universe where keyboards only have numbers and (most) symbols.

---

## Challenge Summary

この問題では、接続先のサーバー上で **英字が入力できない制限環境** に置かれる。  
`ls`, `cd`, `cat` など通常のコマンドが直接入力できないため、  
Bash の内部機能（パラメータ展開・リダイレクト・部分文字列抽出）を駆使して英字を生成し、  
最終的に `flag.txt` を読むのが目的。

---

## Key Idea

Bash では「`.`（ドット）」コマンドを単独で実行するとエラーになる：

```bash
bash: .: filename argument required
.: usage: . filename [arguments]
```

この **usageメッセージ** に英字が含まれている。
これをファイルに保存し、部分的に抽出すれば英字コマンドを再構築できる。

---

## Step-by-Step Solution

### エラーメッセージをファイルに保存

```bash
.>&$
```

→ usageメッセージをファイル `$` に出力。

### ファイル `$` の内容を変数に取り込む

```bash
__="$(<$)"
```

→ `$` ファイルの中身を `__` に格納。

### 文字列加工（不要部分の削除）

```bash
__="${_##*. ??}"
```

→ `__` の中から「最後のドット＋2文字」を削除して、
`filename [arguments]` という英字入り文字列を得る。

###  部分文字列展開で英字を抽出

```bash
___=${__:0:1}${__:16:1}
```

→ `${__:0:1}` = `l`、`${__:16:1}` = `s`
→ `___="ls"`

### `ls` 実行

```bash
$___
```

出力：

```
'$'   blargh   on-calastran.txt
```

### `ls -l` の作成と実行

```bash
$___ -${__:0:1}
```

`-l` オプション付きで実行できる。

### 結果を配列に格納

```bash
____=($(${___}))
```

`____[0]="$"`
`____[1]="blargh"`
`____[2]="on-calastran.txt"`

### `nl` コマンド（catの代用）を生成

```bash
_____=${__:2:1}${__:0:1}
```

→ `${__:2:1}` = `n`、`${__:0:1}` = `l`
→ `_____="nl"`

### ファイル内容の確認

```bash
$_____ "${____[2]}"
```

→ `on-calastran.txt` の内容が出力される。

### `cd` コマンドを生成

```bash
_______="${_##*.}"
______=${_______:3:1}${_______:9:1}
```

→ 変数加工で `cd` を作る。

```bash
$______ "${____[1]}"
```

→ `cd blargh`（`blargh` ディレクトリに移動）

---

## Inside blargh Directory

```bash
$___ -${__:0:1}
total 8
-rw-r--r-- 1 root root   53 Mar 12  2024 flag.txt
-rw-r--r-- 1 root root 1090 Feb  7  2024 on-alpha-9.txt
```

### flagファイルの表示

```bash
____=($(${___}))
$_____ "${____[1]}"
```

出力：

```
return 0 picoCTF{7h15_mu171v3r53_15_m4dn355_4945630a}
```

---

## FLAG

```
picoCTF{7h15_mu171v3r53_15_m4dn355_4945630a}
```

---

## Key Bash Techniques Used

| 構文                    | 機能                  | 使用例                |
| --------------------- | ------------------- | ------------------ |
| `. >&$`               | usageメッセージをファイルに保存  | エラーメッセージから英字取得     |
| `$(<file)`            | ファイル内容を読み込み（cat短縮形） | `__="$(<$)"`       |
| `${var:start:length}` | 部分文字列抽出             | `${__:0:1}`        |
| `${var##pattern}`     | パターンに最長一致して削除       | `${_##*. ??}`      |
| 配列展開                  | コマンド結果を要素に格納        | `____=($(${___}))` |

---

## Lessons Learned

* Bash は**英字入力なしでも文字列操作だけで英字を生成できる**。
* `${}` の構文（パラメータ展開）を理解すれば、
  どんな制限下でもコマンド構築が可能。
* `. >&$` のトリックは、CTFで「文字列ソースを作る」上で非常に強力。

---

### Author’s note

SansAlphaは、単なる「英字禁止問題」ではなく、
**Bashという言語そのものの柔軟性と脳筋トリックの極地**を体感できる傑作。
「`.` のエラーメッセージで英字を生み出す」発想がすごすぎる。

---

**参考:**

* [Bash Parameter Expansion](https://www.gnu.org/software/bash/manual/html_node/Shell-Parameter-Expansion.html)
* [picoCTF 2024 - General Skills - SansAlpha (syreal)](https://picoctf.org)

