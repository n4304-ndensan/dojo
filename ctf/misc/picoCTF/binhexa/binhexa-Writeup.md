# binhexa Writeup

## 問題情報

- **Challenge name:** binhexa

- **Author:** Nana Ama Atombo-Sackey

- **Description:**

  > How well can you perform basic binary operations? Start searching for the flag here
  > `nc titan.picoctf.net 51613`

- **ジャンル:** Misc / Binary Operations

- **難易度:** 初級

- **提供:** picoCTF

---

## 問題の概要

サーバーに `nc` で接続すると、以下のように「2 進数表記された 2 つの数」に対して演算を行う問題が順番に出題されます。

演算内容は以下の通り：

- AND (`&`)
- OR (`|`)
- シフト (`<<`, `>>`)
- 足し算 (`+`)
- 掛け算 (`*`)

各問題の答えを正しい 2 進数で入力すると次へ進め、最後に「16 進数で答えを入力」するとフラグが得られます。

---

## 解法の流れ

1. サーバーから受け取った「Binary Number 1」「Binary Number 2」を 10 進数に変換する。
   例: `00011000` = 24, `11010011` = 211

2. 出題された演算子に応じて計算する。

   - `&` → ビット AND
   - `|` → ビット OR
   - `<<` / `>>` → シフト（問題文に「by X bits」と書いてあるのでシフト量を抽出）
   - `+` → 加算
   - `*` → 乗算

3. 結果を 2 進数に変換して回答する。

   - 掛け算のように結果が 8bit を超える場合は **最小ビット長で表記**する。
   - 例: 5064 → `1001111001000`

4. 最後の設問では「結果を 16 進数で答えろ」と言われるので、計算結果を 16 進数に変換して入力する。

---

## 実装例（Python, pwntools）

```python
from pwn import *

HOST = "titan.picoctf.net"
PORT = 51613

def b2i(s): return int(s, 2)
def i2b(n): return format(n, "b")   # 最小ビット長で返す

io = remote(HOST, PORT)
bin1 = bin2 = last = None

while True:
    line = io.recvline().decode(errors="ignore").strip()
    if not line:
        continue
    print(line)

    if line.startswith("Binary Number 1:"):
        bin1 = b2i(line.split(":")[1].strip())
    if line.startswith("Binary Number 2:"):
        bin2 = b2i(line.split(":")[1].strip())

    if line.startswith("Operation"):
        op = line.split("'")[1]
        if op == "&": last = bin1 & bin2
        elif op == "|": last = bin1 | bin2
        elif op == "+": last = bin1 + bin2
        elif op == "-": last = bin1 - bin2
        elif op == "*": last = bin1 * bin2
        elif op == ">>":
            shift = int(line.split("by")[1].split()[0])
            last = bin2 >> shift if "Number 2" in line else bin1 >> shift
        elif op == "<<":
            shift = int(line.split("by")[1].split()[0])
            last = bin2 << shift if "Number 2" in line else bin1 << shift

    if "Enter the binary result" in line:
        io.sendline(i2b(last))
    if "Enter the results of the last operation in hexadecimal" in line:
        io.sendline(format(last, "X"))
    if "picoCTF{" in line:
        print("FLAG:", line)
        break
```

---

## 学び

- CTF の Misc 問題として「**バイナリ表現の基礎演算**」をきちんと理解しているかが問われている。
- その場で手計算も可能だが、自動スクリプトを用いると確実。
- Python の `int(s, 2)` と `format(n, "b")` / `format(n, "X")` が便利。
- CTF の実戦では **pwntools での自動応答**が標準テクニック。

---
