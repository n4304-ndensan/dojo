# Dictionary / HashMap 基本ガイド（Java / C# / Python / Rust）

キーと値を対応づけて保存する **連想配列（マップ）** の超入門。概念 → 各言語の最小パターン → よくある落とし穴まで。

---

## 1. 基本概念

* **マップ(Map)**：`key -> value` の対応を持つデータ構造。
* **ハッシュマップ(HashMap/Dictionary)**：ハッシュ関数を使って高速に探索するマップ。平均 **O(1)**（最悪 O(n)）。
* **順序**：実装により差がある（挿入順を保持する/しない、ソートされる/されない）。
* **用途**：頻度カウント、インデックス化、重複検出、キャッシュ、グルーピングなど。

---

## 2. 用語対応表

| 概念      | Java                     | C# (.NET)                                             | Python                          | Rust                                        |
| ------- | ------------------------ | ----------------------------------------------------- | ------------------------------- | ------------------------------------------- |
| ハッシュマップ | `HashMap<K,V>`           | `Dictionary<TKey,TValue>`                             | `dict`                          | `HashMap<K,V>`                              |
| ソートマップ  | `TreeMap<K,V>`           | `SortedDictionary<TKey,TValue>`                       | `dict` + `sorted()` / `heapq` 等 | `BTreeMap<K,V>`                             |
| 挿入順保持   | `LinkedHashMap<K,V>`     | `OrderedDictionary`（`System.Collections.Specialized`） | `dict`（3.7+ 挿入順保持）              | なし（順序非保証）                                   |
| スレッド安全  | `ConcurrentHashMap<K,V>` | `ConcurrentDictionary<TKey,TValue>`                   | ロック等で制御                         | `Mutex/RwLock` + `HashMap` / `dashmap` クレート |

> 注：C# `Dictionary` の列挙順は実装依存。**順序が必要なら** `OrderedDictionary`/`SortedDictionary` を使う。

---

## 3. 最小の使い方（クイックレシピ）

### Java

```java
import java.util.*;
Map<String,Integer> m = new HashMap<>();
// 追加/更新
m.put("a", 1);
m.put("a", m.getOrDefault("a", 0) + 1);
// 取得
Integer v = m.getOrDefault("a", 0);
// 存在確認
boolean has = m.containsKey("a");
// 反復
for (Map.Entry<String,Integer> e : m.entrySet()) {
    System.out.println(e.getKey()+"="+e.getValue());
}
// 削除
m.remove("a");
```

**カウント最短**（Java 8+）

```java
for (String s : words) m.merge(s, 1, Integer::sum);
```

初期容量・負荷係数（リサイズ削減）

```java
Map<String,Integer> m = new HashMap<>(/*initialCapacity*/ 128, /*loadFactor*/ 0.75f);
```

---

### C\#

```csharp
using System; using System.Collections.Generic;
var m = new Dictionary<string,int>();
// 追加/更新
m["a"] = 1;
m["a"] = m.TryGetValue("a", out var n) ? n + 1 : 1;
// 取得
int v = m.TryGetValue("a", out var x) ? x : 0;
// 存在確認
bool has = m.ContainsKey("a");
// 反復
foreach (var kv in m) Console.WriteLine($"{kv.Key}={kv.Value}");
// 削除
m.Remove("a");
```

初期容量

```csharp
var m = new Dictionary<string,int>(capacity: 128);
```

> 参照型キーの **null は不可**。順序が必要なら `OrderedDictionary` / ソートなら `SortedDictionary`。

---

### Python

```python
m = {}
# 追加/更新
m['a'] = 1
m['a'] = m.get('a', 0) + 1
# 取得
v = m.get('a', 0)
# 存在確認
has_a = 'a' in m
# 反復
for k, v in m.items():
    print(k, v)
# 削除
del m['a']
```

頻度カウント最短

```python
from collections import Counter
m = Counter(words)
```

> Python 3.7+ の `dict` は **挿入順保持**。不可変でない（リスト等）はキーにできない。

---

### Rust

```rust
use std::collections::HashMap;
let mut m: HashMap<&str, usize> = HashMap::new();
// 追加/更新
m.insert("a", 1);
*m.entry("a").or_insert(0) += 1;
// 取得
let v = *m.get("a").unwrap_or(&0);
// 存在確認
let has = m.contains_key("a");
// 反復
for (k, v) in &m { println!("{}={}", k, v); }
// 削除
m.remove("a");
```

容量予約（リサイズ削減）

```rust
let mut m = HashMap::with_capacity(128);
```

> キー型は `Eq` と `Hash` を実装（`#[derive(Eq, PartialEq, Hash)]`）。並列書き込みは `Mutex/RwLock` 等で保護。

---

## 4. よくあるタスク（共通イディオム）

* **頻度カウント**：存在しなければ 0 を入れて +1
* **初期化付き取得**：存在しなければデフォルト値（`getOrDefault` / `TryGetValue` / `dict.get` / `entry().or_insert()`）
* **事前に容量を見積もる**：リサイズ回数を減らしパフォーマンス改善（`with_capacity` / 初期容量指定）
* **順序が必要かを確認**：挿入順保持やソート構造（`LinkedHashMap`/`TreeMap`/`OrderedDictionary`/`BTreeMap` 等）

---

## 5. カスタムキーを使うとき

* **Java**：`equals` と `hashCode` を適切に override。
* **C#**：型で `Equals`/`GetHashCode` を override、または `IEqualityComparer<T>` を渡す。
* **Python**：クラスに `__eq__` と `__hash__` を実装（不変性を保つ）。
* **Rust**：`#[derive(PartialEq, Eq, Hash)]`。ハッシュはキーの不変な属性に基づくこと。

---

## 6. スレッド安全と並行性

* 単純な `HashMap/Dictionary` は **複数スレッド同時書き込みで安全ではない**。
* Java：`ConcurrentHashMap`。
* C#：`ConcurrentDictionary`。
* Python：GIL があるが書き込みはロック等で保護すべき。
* Rust：共有するなら `Arc<Mutex<HashMap<..>>>` / `RwLock` などで保護（または外部クレート）。

---

## 7. 罠・注意点

* **最悪計算量**：敵対的入力で O(n) になる可能性。対策としてランダム化ハッシュ等（実装依存）。
* **null/None**：

  * Java `HashMap`：**null キー1つOK、null 値OK**。`ConcurrentHashMap` は null 不可。
  * C# `Dictionary`：**null キー不可**（参照型）。値は型により可。
  * Python `dict`：`None` をキーにできる。
  * Rust：`Option<T>` をキーにできるが `Hash/Eq` 条件に従う。
* **順序に依存しない**：必要なら専用の順序付き構造を使う。

---

## 8. 代表的パターン集（最小）

* **頻度カウント**：

  * Java：`for (var s: words) m.merge(s, 1, Integer::sum);`
  * C#：`foreach (var s in words) m[s] = m.TryGetValue(s, out var n) ? n+1 : 1;`
  * Python：`m = Counter(words)` / `m[s] = m.get(s,0)+1`
  * Rust：`for s in words { *m.entry(s).or_insert(0) += 1; }`

---

### 参考命名

* 一般用途: `map_basics.md` / `hashmap_cheatsheet.md`
* 頻度特化: `frequency_counter.md`

> 本資料は最小実例中心。より詳しいパフォーマンスチューニングや比較表を追記していく用途を想定。
