# writeup.md

## 問題概要

* **問題名**: Tab, Tab, Attack
* **出題元**: picoCTF
* **ジャンル**: misc（基礎的な Linux 操作）
* **Author**: syreal

### Description

```
Using tabcomplete in the Terminal will add years to your life, esp. when dealing with long rambling directory structures and filenames: Addadshashanammu.zip
```

要するに「タブ補完（Tab キーでファイル・ディレクトリ名を補完する機能）」を使えば、長いディレクトリ階層でも楽に操作できるよ、という内容。
与えられるファイルは **Addadshashanammu.zip**。

---

## 解法の流れ

### 1. zip ファイルの展開

```bash
unzip Addadshashanammu.zip
```

展開すると、非常に長い名前のディレクトリが多階層で生成される。
例：

```
Addadshashanammu/
  Almurbalarammi/
    Ashalmimilkala/
      Assurnabitashpi/
        Maelkashishi/
          Onnissiralis/
            Ularradallaku/
              fang-of-haynekhtnamet
```

※ 一見するとタイプするのが大変な長い名前ばかり。
ここで **Tab 補完**を使うと効率的に移動できる。

---

### 2. 対象ファイルの確認

最深部に `fang-of-haynekhtnamet` という実行ファイルがある。

```bash
ls Addadshashanammu/Almurbalarammi/Ashalmimilkala/Assurnabitashpi/Maelkashishi/Onnissiralis/Ularradallaku/
```

結果：

```
fang-of-haynekhtnamet
```

---

### 3. ファイル種別の確認

Linux では `file` コマンドで種別を判定できる。

```bash
file fang-of-haynekhtnamet
```

出力：

```
ELF 64-bit LSB pie executable, x86-64, dynamically linked, not stripped
```

→ 64bit ELF 実行ファイルであることが分かる。

---

### 4. 静的解析（安全に中身を確認）

CTF のバイナリは直接実行する前に **strings** で埋め込まれた文字列を調べるのが定石。

```bash
strings fang-of-haynekhtnamet | grep pico
```

出力：

```
*ZAP!* picoCTF{l3v3l_up!_t4k3_4_r35t!_76266e38}
```

→ ここでフラグを発見 🎉

---

### 5. 逆アセンブルで裏付け

さらに `radare2` や `objdump` で `main` 関数を解析すると、
プログラムは単に `puts()` を呼んでフラグ文字列を出力しているだけであることが分かる。

例（radare2 の出力一部）：

```
0x0000063e      lea rdi, str.ZAP__picoCTF{...}
0x00000645      call sym.imp.puts
```

つまり **実行しても同じフラグが出力される**だけのバイナリ。

---

## 最終フラグ

```
picoCTF{l3v3l_up!_t4k3_4_r35t!_76266e38}
```

---

## 学びポイント

1. **タブ補完の便利さ**
   長い階層・名前のディレクトリも `cd A<Tab>` → `cd Al<Tab>` のように補完で楽に移動できる。
   問題タイトル「Tab, Tab, Attack」はその練習。

2. **CTF バイナリの安全な調査手順**

   * いきなり実行せず `file` / `strings` で確認
   * 文字列にフラグが埋め込まれているケースも多い
   * 必要なら `radare2` / `objdump` で `main` 関数を見る

3. **ELF ファイルの理解**

   * 「not stripped」ならシンボル情報が残っていて解析しやすい
   * `puts` 呼び出しでそのまま文字列を表示するだけと分かった

---

## まとめ

* ZIP を展開 → 深い階層を **Tab 補完**で移動
* バイナリを調べる → `strings` ですぐフラグ発見
* 解析で裏付け → 単純に `puts()` しているだけの実行ファイル

**難易度**: 入門レベル
**得られるスキル**: ターミナル操作（Tab 補完）、基本的な ELF ファイル解析、CTF の基本的な調査フロー

---
