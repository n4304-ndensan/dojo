#CTF #ReverseEngineering #Python #Serpentine

---

# Serpentine Writeup

**Author:** LT “syreal” Jones  
**Category:** Reverse Engineering  
**Challenge:** Find the hidden flag in the provided Python script.

---

## 問題概要

与えられたPythonスクリプトには、`print_flag()` 関数が存在するが、通常の実行ではこの関数が正しくフラグを出力しないようになっている。  
課題は、このスクリプトを解析して暗号化されたフラグを復号し、元の文字列（flag）を求めることである。

---

## ソースコード解析

### 暗号化処理

```python
def str_xor(secret, key):
    new_key = key
    i = 0
    while len(new_key) < len(secret):
        new_key = new_key + key[i]
        i = (i + 1) % len(key)
    return "".join([chr(ord(secret_c) ^ ord(new_key_c)) for (secret_c,new_key_c) in zip(secret,new_key)])
```

関数 `str_xor` は、文字列 `secret` と `key` の文字コードをXOR（排他的論理和）して復号・暗号化を行う。
キーは `key = 'enkidu'` として与えられており、`secret` の長さに合わせて繰り返し拡張される。

---

### 暗号化されたデータ

```python
flag_enc = chr(0x15) + chr(0x07) + chr(0x08) + chr(0x06) + ...
```

この部分がフラグの暗号化データである。`chr(0x??)` の形で構成されており、XOR復号で平文を得る。

---

### フラグ復号関数

```python
def print_flag():
  flag = str_xor(flag_enc, 'enkidu')
  print(flag)
```

この関数により、`flag_enc` と `'enkidu'` のXORを取ると平文のフラグが得られる。

---

## 復号手順

Pythonインタプリタ上で次のコードを実行するとフラグを復元できる。

```python
def str_xor(secret, key):
    new_key = key
    i = 0
    while len(new_key) < len(secret):
        new_key = new_key + key[i]
        i = (i + 1) % len(key)
    return "".join([chr(ord(secret_c) ^ ord(new_key_c)) for (secret_c,new_key_c) in zip(secret,new_key)])

flag_enc = (
    chr(0x15) + chr(0x07) + chr(0x08) + chr(0x06) + chr(0x27) + chr(0x21) + chr(0x23) + chr(0x15) +
    chr(0x5c) + chr(0x01) + chr(0x57) + chr(0x2a) + chr(0x17) + chr(0x5e) + chr(0x5f) + chr(0x0d) +
    chr(0x3b) + chr(0x19) + chr(0x56) + chr(0x5b) + chr(0x5e) + chr(0x36) + chr(0x53) + chr(0x07) +
    chr(0x51) + chr(0x18) + chr(0x58) + chr(0x05) + chr(0x57) + chr(0x11) + chr(0x3a) + chr(0x0f) +
    chr(0x0e) + chr(0x59) + chr(0x06) + chr(0x4d) + chr(0x55) + chr(0x0c) + chr(0x0f) + chr(0x14)
)

print(str_xor(flag_enc, 'enkidu'))
```

---

## 復号結果

```
picoCTF{...}
```

---

## 考察

* 暗号方式は単純なXOR暗号であり、キー `'enkidu'` をソースコードから直接取得可能。
* 難読化は最低限（`chr(0x??)` 形式）で、バイト列の解析で容易に解読できる。
* CTF初級レベルのリバースエンジニアリング課題として設計されている。

---

## 要約表

| 項目   | 内容                           |
| ---- | ---------------------------- |
| 分類   | Reverse Engineering / Crypto |
| 暗号方式 | XOR                          |
| 復号キー | `'enkidu'`                   |
| 実装言語 | Python                       |
| 計算量  | $O(n)$（文字列長に線形）              |
| 難易度  | 初級                           |
| 出力   | `picoCTF{...}`               |

---

**まとめ:**
本課題「Serpentine」は、Pythonスクリプト中のXOR暗号化された文字列を復号する基本的なリバースエンジニアリング問題である。`str_xor()` の動作を理解し、与えられたキー `'enkidu'` を用いて復号すれば、フラグを再現できる。

