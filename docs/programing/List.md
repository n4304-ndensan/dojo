# List 基本ガイド（Java / C# / Python / Rust）

要素を **順序付きで保持するリスト** の超入門。概念 → 各言語の最小パターン → よくある落とし穴まで。

---

## 1. 基本概念

* **リスト(List)**：順序付きで要素を保持するコレクション。
* **重複あり**：同じ要素を複数回入れられる。
* **インデックス**：添字でアクセス可能（0始まりが多い）。
* **用途**：順序付きデータ、ログ保持、配列より柔軟な可変長コレクション。

---

## 2. 用語対応表

| 概念     | Java            | C# (.NET)          | Python                      | Rust                      |
| ------ | --------------- | ------------------ | --------------------------- | ------------------------- |
| リスト    | `ArrayList<E>`  | `List<T>`          | `list`                      | `Vec<T>`                  |
| リンクリスト | `LinkedList<E>` | `LinkedList<T>`    | なし（代替: `collections.deque`） | `LinkedList<T>`（標準コレクション） |
| 読み取り専用 | `List.of()`（不変） | `IReadOnlyList<T>` | `tuple`（代替）                 | `&[T]`（スライス参照）            |

---

## 3. 最小の使い方（クイックレシピ）

### Java

```java
import java.util.*;

List<String> list = new ArrayList<>();
list.add("apple");
list.add("banana");
System.out.println(list.get(0)); // apple
for (String x : list) System.out.println(x);
list.remove("apple"); // 値を削除
```

---

### C\#

```csharp
using System; using System.Collections.Generic;

var list = new List<string>();
list.Add("apple");
list.Add("banana");
Console.WriteLine(list[0]); // apple
foreach (var x in list) Console.WriteLine(x);
list.Remove("apple"); // 値を削除
```

---

### Python

```python
lst = []
lst.append("apple")
lst.append("banana")
print(lst[0])  # apple
for x in lst:
    print(x)
lst.remove("apple")  # 値を削除（存在しないと ValueError）
```

---

### Rust

```rust
fn main() {
    let mut v: Vec<&str> = Vec::new();
    v.push("apple");
    v.push("banana");
    println!("{}", v[0]); // apple
    for x in &v {
        println!("{}", x);
    }
    v.retain(|&x| x != "apple"); // 値を削除
}
```

---

## 4. よくあるタスク

* **重複OK**：同じ要素を複数回追加可能。
* **順序保持**：挿入順を維持。
* **サイズ確認**：

  * Java: `list.size()`
  * C#: `list.Count`
  * Python: `len(lst)`
  * Rust: `v.len()`
* **要素の削除**：

  * Java: `list.remove("x")` / `list.remove(index)`
  * C#: `list.Remove("x")` / `list.RemoveAt(index)`
  * Python: `lst.remove("x")` / `del lst[i]` / `pop(i)`
  * Rust: `v.remove(index)` / `v.retain(...)`

---

## 5. よくある落とし穴

* **範囲外アクセス**：`IndexOutOfBoundsException` / `ArgumentOutOfRangeException` / `IndexError` / `panic!`
* **削除時の挙動**：インデックスで削除か値で削除か注意。
* **Rust の Vec**：イテレーション中に `remove` は不可 → `retain` を使う。
* **不変リスト**：Java `List.of` は追加/削除不可。

---

## 6. スレッド安全

* Java: `Collections.synchronizedList(list)` でラップ可能。
* C#: 標準 `List<T>` は非スレッドセーフ、必要なら `ConcurrentBag<T>` やロック。
* Python: `list` 自体はスレッドセーフではない → ロック併用。
* Rust: `Arc<Mutex<Vec<T>>>` などで共有管理。

---

## 7. 代表的パターン集

* **初期化**

  * Java: `Arrays.asList("a","b","c")`
  * C#: `new List<string> {"a","b","c"}`
  * Python: `["a","b","c"]`
  * Rust: `vec!["a","b","c"]`
* **検索**

  * Java: `list.contains("x")`
  * C#: `list.Contains("x")`
  * Python: `"x" in lst`
  * Rust: `v.contains(&"x")`

---
