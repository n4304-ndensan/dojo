# picoCTF: static（“Static ain't always noise”） Writeup

作者: syreal / 分類: Reversing・General Skills / 難易度: ★☆☆

---

## TL;DR（最短解）

`strings static | grep -i picoCTF` を実行すると、

```
picoCTF{dummy}
```

がヒット。フラグは `.rodata` に埋め込まれており、実行時には出力されない“飾り文字列”です。付属の `ltdis.sh`（BASHスクリプト）は `objdump` と `strings` を自動実行する補助ツールに過ぎません。

---

## 問題文の要点

* バイナリ名: `static`
* ヒント: 「この BASH スクリプト（`ltdis.sh`）が役立つかも」
* メッセージ: *“Static ain't always noise”*（「静的領域はいつもノイズとは限らない → 役立つ文字列が隠れているかも」）

---

## ねらい（学べること）

1. ELF バイナリの基本（セクション: `.text` / `.rodata` など）
2. `strings`, `objdump`, `readelf` などの定番バイナリ解析コマンドの使い方
3. 実行フロー（`_start → __libc_start_main → main`）と、リテラル文字列の配置場所の理解

---

## 付属スクリプト `ltdis.sh` の役割

```bash
objdump -Dj .text static > static.ltdis.x86_64.txt
strings -a -t x static > static.ltdis.strings.txt
```

* `.text`（実行コード）を逆アセンブルして保存
* バイナリ全体から**印字可能文字列**を抽出して、ファイルオフセット付きで保存

> ポイント: このスクリプト自体は“解析の作法”を自動化しているだけ。核心は `strings` の結果を読むこと。

---

## 再現手順（コマンドと観察）

### 1) 実行してみる

```bash
$ chmod +x static
$ ./static
Oh hai! Wait what? A flag? Yes, it's around here somewhere!
```

* 出力は**フラグっぽい案内メッセージのみ**。ここでフラグは出ません。

### 2) 文字列抽出（最短の解）

```bash
$ strings -a -t x static | grep -i picoCTF
    1020 picoCTF{dummy}
```

* オフセット `0x1020` 付近にフラグ。多くの CTF で“何もしていない文字列”が `.rodata` に眠っています。

### 3) セクションの確認（任意）

```bash
$ readelf -S static | egrep "\.(text|rodata|data|bss)"
```

* フラグは**読み取り専用データ**である `.rodata` に格納されるのが一般的。

### 4) 逆アセンブルの観察（任意）

```bash
$ objdump -d static | less
```

* `main` の部分は以下の通り：

```
000000000000063a <main>:
 63a: 55                    push   %rbp
 63b: 48 89 e5              mov    %rsp,%rbp
 63e: 48 8d 3d a3 00 00 00  lea    0xa3(%rip),%rdi        # 6e8
 645: e8 c6 fe ff ff        call   510 <puts@plt>
 64a: b8 00 00 00 00        mov    $0x0,%eax
 64f: 5d                    pop    %rbp
 650: c3                    ret
```

* `lea ... -> %rdi` で `puts` に渡しているのは「Oh hai! ...」の**案内メッセージ**（`.rodata@0x6e8`）。
* **フラグ文字列は参照されていない**ため、実行時には出力されません（＝`strings` で探す狙い）。

---

## なぜ `strings` で見えるの？

* コンパイル時に**文字列リテラル**は通常 `.rodata`（読み取り専用データ）に格納されます。
* 実行時に使われなくても、リンク後の最終バイナリに**静的に含まれている**ため、`strings` のようなツールで抽出できます。
* シンボルが `strip` されていても、**文字列のバイト列**は残る限り見つけられます。

> ちなみにこのバイナリは名前 `static` ですが、`ldd` を見ると `libc.so.6` などに依存しており**動的リンク**です。タイトルは“静的領域（static storage）”のダジャレ的ニュアンス。

---

## ELF とセクション超ざっくり

* `.text`: 実行命令（機械語）が入る
* `.rodata`: 読み取り専用の定数・文字列
* `.data`: 初期化済みの書き込み可能データ
* `.bss`: 初期化されていないデータ（実体はゼロ埋め）

今回のフラグは `.rodata` にあり、`main` では参照されません。

---

## 補足ツールとコマンド（便利メモ）

* `file static` … ELF 種別やリンク方式（動的/静的）を確認
* `readelf -S static` … セクション一覧
* `objdump -s -j .rodata static | less` … `.rodata` の**生ダンプ**を覗く
* `strings -a -t x static | less` … 文字列+ファイルオフセット
* `nm -D static` / `objdump -T static` … 動的シンボル（`puts@plt` など）

---

## よくあるハマりどころ

* **実行しても何も出ない** → そもそもフラグを印字するコードがない（今回は案内メッセージのみ）
* **`strings` でノイズが多い** → `grep -i picoCTF` などで**フィルタ**
* **本当に動的リンク？** → `ldd static` で依存確認（`libc.so.6` などが表示される）

---

## まとめ（学び）

* 「とりあえず `strings` で全スキャンして絞り込む」は Reversing 入門の定番
* 実行フローを逆アセンブリで確認し、「出力される文字列」と「埋め込みだけの文字列」を見分けられると強い
* セクションの役割（`.text` / `.rodata` …）を押さえておくと、次のバイナリでも応用が効く

---

## 付録: 代表的な `strings` 出力抜粋

```
    6e8 Oh hai! Wait what? A flag? Yes, it's around here somewhere!
   1020 picoCTF{dummy}
```

---

## 次のステップ（練習）

1. `objdump -s -j .rodata static` で `.rodata` の生バイトを眺め、ASCII がどう埋まるか確認
2. `strip static` して**シンボル無し**の状態でも `strings` が効くことを試す
3. 逆に、フラグを**単純な XOR でエンコード**したバイナリを自作して、`strings` では見破れない例を体験

---
