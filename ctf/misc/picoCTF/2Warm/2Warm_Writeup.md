# 2Warm — Writeup

**Author:** Sanjay C / Danny Tunitis
**Category:** General Skills / Warmup
**Task:** 10 進数 **42** を **2 進数** に変換せよ。

---

## TL;DR（答え）

**42₁₀ = 101010₂**

---

## アプローチ

10 進 →2 進変換の基本は「2 で割って余りを並べる」方式。

```text
n を 2 で割る → 余り(0/1)を記録 → 商で続行 → 商が 0 になったら終了 → 余りを逆順に読む
```

### 手計算の流れ（42）

| 割り算 |  商 | 余り |
| ------ | --: | ---: |
| 42 ÷ 2 |  21 |    0 |
| 21 ÷ 2 |  10 |    1 |
| 10 ÷ 2 |   5 |    0 |
| 5 ÷ 2  |   2 |    1 |
| 2 ÷ 2  |   1 |    0 |
| 1 ÷ 2  |   0 |    1 |

余りを**下から上（逆順）**に読む → **101010**

---

## Python での確認

```python
n = 42
print(bin(n))        # '0b101010'
print(format(n, 'b'))# '101010'

# 汎用: 任意基数→任意基数（2〜36）
def base_convert(num_str: str, from_base: int, to_base: int) -> str:
    digits = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
    num = int(num_str, from_base)
    if num == 0:
        return "0"
    out = []
    while num > 0:
        out.append(digits[num % to_base])
        num //= to_base
    return ''.join(reversed(out))

print(base_convert("42", 10, 2))  # '101010'
```

---

## ビット視点の理解

42 は 32 + 8 + 2（= 2⁵ + 2³ + 2¹）。該当ビットを 1 にして並べる。

| 2⁵  | 2⁴  | 2³  | 2²  | 2¹  | 2⁰  |
| :-: | :-: | :-: | :-: | :-: | :-: |
|  1  |  0  |  1  |  0  |  1  |  0  |

→ **101010₂**

---

## よくあるミス

- 余りの**順番**を逆にし忘れる。
- `bin()` の結果 `0b` プレフィックスをそのまま出す。
- 0 の特別扱いを忘れる（結果が空文字になる）。

---

## 学び（ポイント）

- 2 進数は「**割り算余り**」でも「**ビット分解**」でも導出可能。
- Python では `bin(n)` / `format(n, 'b')` が最短。
- 汎用変換器を持っておくと他問題（base64/hex 混在など）の土台になる。

---
