# mus1c - Writeup

## 問題概要

Rockstar言語で記述された歌詞風コードをオンラインインタプリタ上で実行し，出力をもとにフラグ文字列を復元する問題である。

## 解析手順

1. 与えられた歌詞形式コードを [https://codewithrockstar.com/online](https://codewithrockstar.com/online) に入力して実行する。
2. 実行結果として14行の整数出力が得られる。
3. これらの整数を ASCII コードとみなして文字へ変換する。
4. 得られた文字列 `rrrocknrn0113r` を picoCTF フラグ形式に整形する。

## 実行結果

出力された整数列:

```
114
114
114
111
99
107
110
114
110
48
49
49
51
114
```

Python により ASCII 変換を行う:

```python
n = 14
l = [int(input()) for _ in range(n)]
s = ''.join(chr(i) for i in l)
print(s)
```

結果: `rrrocknrn0113r`

## フラグ

```
picoCTF{rrrocknrn0113r}
```

## 要約表

| 項目      | 内容                                          |
| ------- | ------------------------------------------- |
| 分類      | Rockstar / Esoteric Language / ASCII Decode |
| 主手法     | Rockstarコード実行 → ASCII変換                     |
| 出力形式    | picoCTF{...}                                |
| 得られた文字列 | rrrocknrn0113r                              |
| フラグ     | picoCTF{rrrocknrn0113r}                     |
