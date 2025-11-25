#CTF #picoCTF #ReverseEngineering #Transformation

# Transformation Writeup

## 1. 問題概要

本問題は picoCTF Reverse Engineering カテゴリ「Transformation」に位置し、与えられた不可解な Unicode 文字列を元の ASCII フラグへ復元する課題である。提示される変換式は以下である：

```
enc = ''.join([chr((ord(flag[i]) << 8) + ord(flag[i + 1]))
               for i in range(0, len(flag), 2)])
```

これは **2 バイト（2 文字）の ASCII を 1 文字の 16bit Unicode にパックするエンコード**である。したがって復元には、Unicode 文字を再び 2 バイトへ分解する必要がある。

問題文には不可解な漢字列が与えられる：

```
灩捯䍔䙻ㄶ形楴獟楮獴㌴摟潦弸彤㔲挶戹㍽
```

## 2. 制約分析

変換式を詳細に見ると：

* `ord(flag[i]) << 8` → **上位 8bit** を構成
* `ord(flag[i+1])` → **下位 8bit** を構成
* 合計で `0xHHLL`（16bit Unicode）1文字が生成される
* したがって、復元時は:

  * `chr(ord(c) >> 8)` → 元の1文字目（上位8bit）
  * `chr(ord(c) & 0xFF)` → 元の2文字目（下位8bit）

この復元カップリングは全 Unicode 文字に適用可能であり、可逆である。

## 3. 解法設計

### 3.1 文字単位の 16bit 分解

エンコード済み文字 `c` に対し：

* `ord(c)` は 0〜65535 の整数
* 上位バイト抽出 → `ord(c) >> 8`
* 下位バイト抽出 → `ord(c) & 255`

これを各文字に適用し ASCII 文字を生成する。

### 3.2 復号コード（最小実装）

```
c = "灩捯䍔䙻ㄶ形楴獟楮獴㌴摟潦弸彤㔲挶戹㍽"

result = []
for ch in c:
    result.append(chr(ord(ch) >> 8))
    result.append(chr(ord(ch) & 0xFF))

print(''.join(result))
```

この処理により、元の `flag` が完全に復元される。

## 4. 計算量評価

* 入力文字数を `N` とすると、1 文字につき O(1) の bit 操作と append のみであり、**全体で O(N)**。
* 文字列結合は最終段階で 1 回のみ実施するため効率的である。

## 5. 実装上の注意点

* Python の `ord()` と `chr()` は Unicode 全域を扱えるため 16bit データの扱いに問題はない。
* UTF-8/UTF-16 の誤デコードは不要であり、**提示された Unicode 文字列をそのまま 16bit パック文字として扱う**ことが最も正しい。
* ファイルから読み込む際には、UTF-8 として受け取り Unicode 文字列として処理すればよい。

## 6. 要約表

| 項目      | 内容                                    |
| ------- | ------------------------------------- |
| 分類      | Reverse Engineering / Unicode packing |
| エンコード形式 | 2文字を1 Unicode (16bit) にパック            |
| デコード式   | `chr(ord(c)>>8)` / `chr(ord(c)&0xFF)` |
| 計算量     | O(N)                                  |
| 注意点     | 文字列を UTF-16 と誤って処理しない                 |
| 最終結果    | ASCII ベースの本来のフラグを再構築                  |

---

本 Writeup は変換式の逆操作の理解を基礎に、16bit パッキングの復元処理を体系的に解説した。
