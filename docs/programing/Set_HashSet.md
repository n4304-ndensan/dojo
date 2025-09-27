# Set / HashSet 基本ガイド（Java / C# / Python / Rust）

要素を **重複なしで保存する集合** の超入門。概念 → 各言語の最小パターン → よくある落とし穴まで。

---

## 1. 基本概念

* **集合(Set)**：重複しない要素の集まり。
* **ハッシュセット(HashSet)**：ハッシュ関数で管理。平均 **O(1)** で追加・検索・削除。
* **順序**：実装により異なる（順序なし / 挿入順保持 / ソートあり）。
* **用途**：重複排除、会員リスト管理、既読チェック、素集合・和集合・積集合など。

---

## 2. 用語対応表

| 概念     | Java                  | C# (.NET)               | Python                | Rust                   |
| ------ | --------------------- | ----------------------- | --------------------- | ---------------------- |
| ハッシュ集合 | `HashSet<E>`          | `HashSet<T>`            | `set`                 | `HashSet<T>`           |
| ソート集合  | `TreeSet<E>`          | `SortedSet<T>`          | `set` + `sorted()`    | `BTreeSet<T>`          |
| 挿入順保持  | `LinkedHashSet<E>`    | なし（代替: `List+Distinct`） | `set`（3.7+はdictと順序揃う） | なし（順序非保証）              |
| スレッド安全 | `CopyOnWriteArraySet` | なし（ロック併用が必要）            | ロック等で制御               | `Mutex/RwLock+HashSet` |

---

## 3. 最小の使い方（クイックレシピ）

### Java

```java
import java.util.*;

Set<String> s = new HashSet<>();
// 追加
s.add("apple");
s.add("apple"); // 重複は無視
// 存在確認
boolean has = s.contains("apple");
// 反復
for (String x : s) System.out.println(x);
// 削除
s.remove("apple");
```

ソートが必要なら `TreeSet`：

```java
Set<Integer> ts = new TreeSet<>();
ts.add(3); ts.add(1);
System.out.println(ts); // [1, 3]
```

---

### C\#

```csharp
using System; using System.Collections.Generic;

var s = new HashSet<string>();
// 追加
s.Add("apple");
s.Add("apple"); // 重複は無視
// 存在確認
bool has = s.Contains("apple");
// 反復
foreach (var x in s) Console.WriteLine(x);
// 削除
s.Remove("apple");
```

ソートが必要なら `SortedSet<T>`。

---

### Python

```python
s = set()
# 追加
s.add("apple")
s.add("apple")  # 重複は無視
# 存在確認
has = "apple" in s
# 反復
for x in s:
    print(x)
# 削除
s.remove("apple")   # 存在しない場合は KeyError
s.discard("apple")  # 存在しなくてもエラーにならない
```

集合演算

```python
a = {1,2,3}
b = {3,4}
print(a | b)  # 和 {1,2,3,4}
print(a & b)  # 積 {3}
print(a - b)  # 差 {1,2}
```

---

### Rust

```rust
use std::collections::HashSet;

let mut s: HashSet<&str> = HashSet::new();
// 追加
s.insert("apple");
s.insert("apple"); // 重複は無視
// 存在確認
let has = s.contains("apple");
// 反復
for x in &s {
    println!("{}", x);
}
// 削除
s.remove("apple");
```

ソートが必要なら `BTreeSet`。

---

## 4. よくあるタスク（共通イディオム）

* **重複排除**：リストから集合に変換する

  * Java: `new HashSet<>(list)`
  * C#: `new HashSet<T>(list)`
  * Python: `set(list)`
  * Rust: `list.into_iter().collect::<HashSet<_>>()`

* **和集合 / 積集合 / 差集合**

  * Java: `addAll` / `retainAll` / `removeAll`
  * C#: `UnionWith` / `IntersectWith` / `ExceptWith`
  * Python: `|` / `&` / `-`
  * Rust: `.union()` / `.intersection()` / `.difference()`

* **存在確認**：`contains` / `in`

---

## 5. カスタム要素を入れるとき

* **Java**：`equals` と `hashCode` を実装必須。
* **C#**：`Equals`/`GetHashCode` を override か `IEqualityComparer<T>` を渡す。
* **Python**：クラスに `__eq__` と `__hash__` を実装。
* **Rust**：`Eq` と `Hash` を derive（`#[derive(Eq, PartialEq, Hash)]`）。

---

## 6. スレッド安全と並行性

* Java: `CopyOnWriteArraySet` や `Collections.synchronizedSet`
* C#: ロックで保護する（標準で並行Setはない）
* Python: マルチスレッドではロック併用が推奨
* Rust: `Arc<Mutex<HashSet<_>>>` や `dashmap` クレート

---

## 7. 罠・注意点

* **順序保証なし**：順序が必要なら専用構造 (`TreeSet`/`LinkedHashSet`/`SortedSet`/`BTreeSet`) を使う。
* **最悪計算量 O(n)**：ハッシュ衝突次第で性能が落ちる可能性。
* **存在しない要素の削除**：

  * Java/C#/Rust → `false` を返すだけ
  * Python → `remove` は例外、`discard` は安全

---

## 8. 代表的パターン集（最小）

* **重複排除**：

  * Java: `new HashSet<>(list)`
  * C#: `new HashSet<T>(list)`
  * Python: `set(list)`
  * Rust: `vec.into_iter().collect::<HashSet<_>>()`

* **集合演算**：

  * Java: `a.addAll(b)` (和) / `a.retainAll(b)` (積) / `a.removeAll(b)` (差)
  * C#: `a.UnionWith(b)` / `a.IntersectWith(b)` / `a.ExceptWith(b)`
  * Python: `a | b`, `a & b`, `a - b`
  * Rust: `a.union(&b)`, `a.intersection(&b)`, `a.difference(&b)`

---

### 参考命名

* 一般用途: `set_basics.md` / `hashset_cheatsheet.md`
* 集合演算特化: `set_operations.md`

> 本資料は最小実例中心。順序付き・ソート付き集合の違いを意識して使うのがコツ。

---
