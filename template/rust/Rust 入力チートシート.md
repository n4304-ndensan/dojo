（`std::io::read_to_string` / `split_whitespace` 版）

---

## 🧩 共通テンプレート

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    // 以下に入力処理を書く
}
```

以降はこの `it`（イテレータ）から `.next().unwrap().parse::<T>().unwrap()` で読み取ります。

---

## 整数 1 個

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    println!("{}", n);
}
```

---

## 整数 2 個

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();
    println!("{}", a + b);
}
```

---

## 整数 N 個（配列）

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    println!("{:?}", a);
}
```

---

## 文字列 1 つ

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let s = it.next().unwrap();
    println!("{}", s);
}
```

---

## 文字列を 1 文字ずつ `Vec<char>` に

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let s: Vec<char> = it.next().unwrap().chars().collect();
    println!("{:?}", s);
}
```

---

## 2次元配列（H×W）

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    let a: Vec<Vec<i32>> = (0..h)
        .map(|_| (0..w)
            .map(|_| it.next().unwrap().parse().unwrap())
            .collect())
        .collect();

    println!("{:?}", a);
}
```

---

## タプル入力（例：座標列）

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let xy: Vec<(i64, i64)> = (0..n)
        .map(|_| {
            let x = it.next().unwrap().parse().unwrap();
            let y = it.next().unwrap().parse().unwrap();
            (x, y)
        })
        .collect();

    println!("{:?}", xy);
}
```

---

## 文字列 + 数値の混在

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut data = Vec::new();
    for _ in 0..n {
        let s = it.next().unwrap().to_string();
        let x: i32 = it.next().unwrap().parse().unwrap();
        data.push((s, x));
    }

    println!("{:?}", data);
}
```

---

## 文字グリッド入力（`Vec<Vec<char>>`）

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();

    let grid: Vec<Vec<char>> = (0..h)
        .map(|_| it.next().unwrap().chars().collect())
        .collect();

    println!("{:?}", grid);
}
```

---

## 複数テストケース（T ケース分）

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let t: usize = it.next().unwrap().parse().unwrap();
    for _ in 0..t {
        let a: i32 = it.next().unwrap().parse().unwrap();
        let b: i32 = it.next().unwrap().parse().unwrap();
        println!("{}", a + b);
    }
}
```

---

## 補足メモ

|要素|型|パース方法|
|---|---|---|
|整数|`i32`, `i64`, `usize`|`.parse().unwrap()`|
|文字列|`&str` or `String`|`.to_string()` or そのまま|
|文字列→文字|`s.chars().collect::<Vec<char>>()`||
|複数行|ループ＋`collect()`||
|空白区切り|`split_whitespace()`||

---

## 最小テンプレートまとめ

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    // 例:
    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<i32> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    println!("{:?}", a);
}
```

---

## 整数 + 浮動小数 + 文字列

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();   // 整数
    let x: f64 = it.next().unwrap().parse().unwrap();   // 浮動小数
    let s = it.next().unwrap().to_string();             // 文字列

    println!("n={}, x={}, s={}", n, x, s);
}
```

入力例：

```
10 3.14 apple
```

---

## 浮動小数の配列

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let a: Vec<f64> = (0..n).map(|_| it.next().unwrap().parse().unwrap()).collect();

    println!("{:?}", a);
}
```

入力例：

```
5
1.0 2.5 3.25 4.0 5.75
```

---

## 文字列の配列

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s: Vec<String> = (0..n).map(|_| it.next().unwrap().to_string()).collect();

    println!("{:?}", s);
}
```

入力例：

```
3
apple banana cherry
```

---

## (文字列, 浮動小数) のペア配列（例：商品と価格）

```rust
fn main() {
    let stdin = std::io::read_to_string(std::io::stdin()).unwrap();
    let mut it = stdin.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let items: Vec<(String, f64)> = (0..n)
        .map(|_| {
            let name = it.next().unwrap().to_string();
            let price: f64 = it.next().unwrap().parse().unwrap();
            (name, price)
        })
        .collect();

    for (name, price) in items {
        println!("{}: {:.2}", name, price);
    }
}
```

入力例：

```
3
apple 120.5
banana 80.0
grape 250.75
```

