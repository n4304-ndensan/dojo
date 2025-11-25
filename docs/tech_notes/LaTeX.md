# LaTeX 入門 & チートシート

## 1. LaTeXとは？

LaTeX（ラテフ/レイテック）は、数式を美しく表現するための文書作成システムです。  
普段は論文やレポートに使われますが、Markdown や Jupyter Notebook、Obsidian、GitHub などでも `$$ ... $$` で LaTeX 数式を埋め込むことができます。

特に **MathJax / KaTeX** と呼ばれるレンダリングエンジンが使われることが多く、環境によっては ` ```math ... ``` ` のような書き方も可能です。  
ただし、最も汎用的で移植性が高いのは `$$ ... $$` です。

---

## 2. LaTeX 数式チートシート

### 基本構文
- **インライン数式**: `$ ... $`  
  例: `これは $a+b=c$ の形` → これは \(a+b=c\) の形
- **ブロック数式**: `$$ ... $$`  
  例:
  $$
  a^2 + b^2 = c^2
  $$

$$
a^2 + b^2 = c^2
$$

---

### 1. 上下付き文字

| 記法        | 表示        |
| --------- | --------- |
| `x_i`     | $x_i$     |
| `x^2`     | $x^2$     |
| `x_{i+1}` | $x_{i+1}$ |
| `x^{10}`  | $x^{10}$  |
| `x_i^2`   | $x_i^2$   |

---

### 2. ギリシャ文字

| コード      | 表示       | コード      | 表示       |
| -------- | -------- | -------- | -------- |
| `\alpha` | $\alpha$ | `\beta`  | $\beta$  |
| `\gamma` | $\gamma$ | `\Gamma` | $\Gamma$ |
| `\theta` | $\theta$ | `\Theta` | $\Theta$ |
| `\pi`    | $\pi$    | `\Pi`    | $\Pi$    |
| `\sigma` | $\sigma$ | `\Sigma` | $\Sigma$ |
| `\phi`   | $\phi$   | `\Phi`   | $\Phi$   |

---

### 3. 分数・ルート

| コード           | 表示            |
| ------------- | ------------- |
| `\frac{a}{b}` | $\frac{a}{b}$ |
| `\sqrt{x}`    | $\sqrt{x}$    |
| `\sqrt[3]{x}` | $\sqrt[3]{x}$ |

---

### 4. 積分・総和・積

| コード                 | 表示                  |
| ------------------- | ------------------- |
| `\sum_{i=1}^n i`    | $\sum_{i=1}^n i$    |
| `\prod_{i=1}^n i`   | $\prod_{i=1}^n i$   |
| `\int_a^b f(x)\,dx` | $\int_a^b f(x)\,dx$ |

---

### 5. 括弧（自動サイズ調整）

| コード                       | 表示                           |
| ---------------------------- | ------------------------------ |
| `(a+b)`                      | $(a+b)$                        |
| `\left( \frac{a}{b} \right)` | $\left(\frac{a}{b}\right)$     |
| `\left\| x \right\|` | $\left\| x \right\|$   |

---

### 6. 行列

```latex
\begin{bmatrix}
a & b \\
c & d
\end{bmatrix}
```

$$
\begin{bmatrix}
a & b \\
c & d
\end{bmatrix}
$$

---

### 7. 揃え（複数行）

```latex
\begin{aligned}
a &= b + c \\
  &= d + e
\end{aligned}
```

$$
\begin{aligned}
a &= b + c \\
  &= d + e
\end{aligned}
$$

---

### 8. その他便利記号

| 記法            | 表示            | 説明      |
| ------------- | ------------- | ------- |
| `\infty`      | $\infty$      | 無限大     |
| `\cdot`       | $\cdot$       | 中黒（掛け算） |
| `\times`      | $\times$      | ×（掛け算）  |
| `\pm`         | $\pm$         | ±       |
| `\to`         | $\to$         | 矢印      |
| `\Rightarrow` | $\Rightarrow$ | 太い矢印    |
| `\approx`     | $\approx$     | おおよそ    |
| `\equiv`      | $\equiv$      | 恒等      |

---
