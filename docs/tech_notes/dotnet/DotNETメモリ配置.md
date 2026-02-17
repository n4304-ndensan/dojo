#memory #DotNET
# .NET メモリ配置

## 1. 全体像

.NET の[[Memory|メモリ]]は大きく分けて以下の領域で構成される。

- **Stack（スタック）**
- **Heap（ヒープ）**
  - Small Object Heap (SOH)
  - Large Object Heap (LOH)
  - Pinned Object Heap (POH) ※ .NET 5+
- **Static領域**
- **Code領域（JIT済みネイティブコード）**

---

## 2. Stack（スタック）

### 特徴

- スレッドごとに存在
- LIFO（後入れ先出し）
- 自動解放（スコープ終了で消える）
- 高速
- サイズ制限あり（通常1MB程度）

### ここに置かれるもの

- 値型（int, double, bool, structなど）

- メソッドのローカル変数

- 引数

- 参照型の「参照（アドレス）」自体

### 例

```csharp
int x = 10;
string s = "hello";
````

```
Stack:
x = 10
s → (ヒープ上の文字列への参照)
```

---

## 3. Heap（ヒープ）

### 特徴

- 全スレッド共有

- GC（ガベージコレクタ）管理

- 動的確保

- 解放は自動

### ここに置かれるもの

- class

- string

- array

- boxingされた値型

- newされたオブジェクト

---

## 4. Small Object Heap (SOH)

通常のオブジェクトが配置される。

### 世代管理

- Gen0（短命）

- Gen1（中間）

- Gen2（長命）

### GCの基本戦略

- 多くのオブジェクトはすぐ死ぬ前提

- 若い世代から回収

---

## 5. Large Object Heap (LOH)

85,000バイト以上のオブジェクト。

例：

```csharp
byte[] buffer = new byte[100000];
```

### 特徴

- Gen2扱い

- コンパクションされない（断片化しやすい）

---

## 6. Pinned Object Heap (POH)

.NET 5+ で追加。

- GC移動禁止オブジェクト専用

- unmanaged連携用

---

## 7. 参照型の実際の構造

```csharp
class Person
{
    public int Age;
}
```

```
Stack:
p → 0x1234

Heap:
0x1234:
  [ObjectHeader]
  [MethodTablePtr]
  Age = 20
```

---

## 8. 値型と参照型の違い

|項目|値型|参照型|
|---|---|---|
|配置|Stack（通常）|Heap|
|コピー|値コピー|参照コピー|
|GC対象|ならない|なる|

---

## 9. Boxing / Unboxing

### Boxing

```csharp
int x = 10;
object o = x;
```

- ヒープにコピー作成

- object化

### Unboxing

```csharp
int y = (int)o;
```

---

## 10. string の特殊性

- 参照型

- 不変（immutable）

- intern poolがある

---

## 11. 配列の配置

```csharp
int[] arr = new int[3];
```

```
Stack:
arr → 0x2000

Heap:
0x2000:
  length = 3
  [0] 0
  [1] 0
  [2] 0
```

---

## 12. メソッド呼び出し時

```
Stack Frame:
------------------
Return Address
引数
ローカル変数
------------------
```

メソッド終了でフレーム消滅。

---

## 13. static 変数

- AppDomain単位

- プロセス終了まで生存

- GC対象ではあるが常駐扱い

---

## 14. GC（ガベージコレクタ）の仕組み

### マーク＆スイープ方式

1. ルート探索

2. 到達可能オブジェクトをマーク

3. 未到達を回収

4. コンパクション

### ルート例

- Stack上の参照

- static参照

- CPUレジスタ

---

## 15. メモリ断片化

- LOHで起きやすい

- Pinnedオブジェクトで発生

---

## 16. 実務で重要なポイント

- 大量boxingを避ける

- LOHを乱発しない

- 不要なstring連結を避ける

- Listを使う（ArrayList非推奨）

- Dictionary<TKey,TValue>を使う

---

## 17. 図まとめ

```
[Stack]              [Heap]
x = 10               Person object
p → 0x1234     →     Age = 20
```

---

## 18. パフォーマンス設計の観点

### 高速化

- struct活用

- boxing回避

- Span使用

- pooling利用

### 安定化

- 大配列使い回し

- IDisposable徹底

- static肥大化回避

---

## 19. まとめ

- 値型は基本Stack

- 参照型はHeap

- GCは世代管理

- Boxingはstack→heap

- Dictionaryはboxing回避に有効

---
