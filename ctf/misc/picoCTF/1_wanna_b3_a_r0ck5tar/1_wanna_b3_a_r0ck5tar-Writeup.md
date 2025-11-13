#CTF #picoCTF #GeneralSkills #Rockstar #Writeup

# 1_wanna_b3_a_r0ck5tar - Writeup

## 概要

本問題は、picoCTFの General Skills カテゴリに属する問題であり、タイトルからは音楽やロックを連想させる。しかし、提示された「lyrics.txt」や「1_wanna_b3_a_r0ck5tar」の内容をよく見ると、それは**Rockstar言語**で書かれたソースコードであることが分かる。

Rockstarは2018年に発表された、英語の歌詞のような構文で書ける難解プログラミング言語（esolang）である。本問ではそのコードを実際に実行し、出力結果を解析することでフラグを導く。

---

## 問題ファイル

提供ファイル：
```

1_wanna_b3_a_r0ck5tar

```

中身はRockstar言語形式のテキストであり、拡張子 `.rock` にすればそのまま実行可能である。

---

## 環境構築

Rockstarの公式実装を利用する。Node.jsが必要。

```bash
git clone https://github.com/RockstarLang/rockstar
cd rockstar/satriani
```

`node` コマンドで `.rock` ファイルを実行できる。

---

## 解析

中身を読むと、以下のような構造が確認できる：

* `Listen to the ~`
* `If the ~`
* `Else ~`

これらはRockstar言語における**入力・条件分岐構文**であり、特定の入力をしないと何も出力しない。

---

## 解法

### 1. 入力待ちを削除する

`Listen to the ~` は標準入力待ちになるため削除。

### 2. 条件分岐を削除する

`If the ~`, `Else ~` も削除。
これにより、単純なprint文として実行できる。

### 3. ファイルを `lyrics2.rock` として保存し、実行

```bash
node rockstar lyrics2.rock
```

実行結果：

```
66
79
78
74
79
86
73
```

---

## 出力解析

ASCIIに変換する：

| 数値 | 文字 |
| -- | -- |
| 66 | B  |
| 79 | O  |
| 78 | N  |
| 74 | J  |
| 79 | O  |
| 86 | V  |
| 73 | I  |

結果：

```
BONJOVI
```

---

## ✅ Flag

```
picoCTF{BONJOVI}
```

---

## 考察

この問題は、単なるテキストではなく**Rockstar言語のソースコード**であることを見抜けるかが肝となる。
ヒント「rockstarをマスターできるかい？」は「ロックをマスター」ではなく、**“Rockstarプログラミング言語を扱えるか”** という意味。

Rockstar特有の構文（`is`, `say`, `listen`, `shout`, `break it down`）に気づけば、実際にコードを実行してASCIIコードを復号することでフラグを導ける。

---

## 参考資料

* [Rockstar Language GitHub Repository](https://github.com/RockstarLang/rockstar)
* [Rockstar Online Interpreter](https://codewithrockstar.com/online)
* [Esolang Wiki: Rockstar](https://esolangs.org/wiki/Rockstar)

---

## 総評

この問題は、難解言語に気づく観察力と、出力のASCII変換というシンプルな符号処理を組み合わせた、CTFらしい良問である。
Rockstarの文法（詩的代入、`Listen`, `Say`, `Shout`, `If`, `Break it down`など）を理解すれば、他の類似問題にも応用可能である。

---
