#CTF #picoCTF #Based

# Based - Writeup

## 1. 問題概要

**Based** は picoCTF におけるデータエンコーディング理解問題であり、バイナリ・8進数・16進数といった異なる基数表現を ASCII 文字列へ正しく変換する能力が求められる。サーバへ接続し、提示されるエンコード形式を適切にデコードすることでフラグを取得する形式である。

---

## 2. 制約分析

* 入力文字列は複数の基数（binary / octal / hexadecimal）で提示される。
* Python でのデコードが想定されるが、他言語や Linux コマンドでも対応可能。
* 各基数変換において、**1バイト = 8ビット = 0x00–0xFF** を単位として処理する必要がある。
* `chr(int(x, base))` は **1バイトごと** に処理する必要があり、複数バイトをまとめて `chr()` することは不可能である。

---

## 3. 解法設計

### 3.1 Binary → ASCII のデコード

スペース区切りの 8bit 列を分割し、個別に 2進数→整数→文字へ変換する。

```python
bins = "01100011 01101111 01101101 01110000 01110101 01110100 01100101 01110010".split()
text = ''.join(chr(int(b, 2)) for b in bins)
```

### 3.2 Octal → ASCII のデコード

エスケープ付き (例: `o146`) の場合は先頭の余分な文字を除去して処理する。

```python
octs = "o160 o145 o141 o162".split()
text = ''.join(chr(int(o.replace('o', ''), 8)) for o in octs)
```

### 3.3 Hex → ASCII のデコード

16進数はそのまま 1バイトごとに処理。

```python
hexes = "6c 69 67 68 74".split()
text = ''.join(chr(int(h, 16)) for h in hexes)
```

---

## 4. 典型的エラー例と原因

### 4.1 `int()` での変換エラー

```
ValueError: invalid literal for int() with base 2
```

**原因：** 2進数以外の文字（スペース・不可視文字）を含む。

### 4.2 `chr()` の範囲外エラー

```
ValueError: chr() arg not in range(0x110000)
```

**原因：** 複数バイトをまとめて `int(..., 2)` してしまい、巨大数を chr() に渡した。

単一バイトずつ処理する必要がある。

---

## 5. 実際の解析例

### 5.1 Binary 例

```python
bin = '01110011 01101111 01100011 01101011 01100101 01110100'
text = ''.join(chr(int(b, 2)) for b in bin.split())
```

出力：

```
socket
```

### 5.2 Octal 例

```python
oct = 'o143 o157 o156 o164 o141 o151 o156 o145 o162'
text = ''.join(chr(int(o.replace('o', ''), 8)) for o in oct.split())
```

出力：

```
container
```

### 5.3 Hex 例

```python
hex = '63 68 61 69 72'
text = ''.join(chr(int(h, 16)) for h in hex.split())
```

出力：

```
chair
```

---

## 6. フラグ

正しく変換を行いサーバとの対話を進めると、最終的に以下のフラグが得られる。

```
picoCTF{learning_about_converting_values_bf1F59A2}
```

---

## 7. 解法の要点まとめ（表）

| 分類     | 必要操作      | Pythonの典型処理            | 注意点                |
| ------ | --------- | ---------------------- | ------------------ |
| Binary | 2進→ASCII  | `int(b, 2)` → `chr()`  | 1バイトずつ処理           |
| Octal  | 8進→ASCII  | `int(o, 8)` → `chr()`  | `o146` のような余分文字を除去 |
| Hex    | 16進→ASCII | `int(h, 16)` → `chr()` | 1バイト＝2桁の16進        |

---

## 8. 総括

この問題は基数変換の基本操作を理解しているかを問うものであり、CTF で頻出する「デコード処理」の基礎を体系的に身につける良い練習となる。各基数の特性と Python におけるデコード方法を把握すれば、より高度な暗号化問題にも応用可能である。
