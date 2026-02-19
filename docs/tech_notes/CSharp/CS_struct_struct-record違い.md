# CS_struct_struct-record違い

対象:
- struct
- record struct

C# 10+ 前提

---
## 1. 共通点

- どちらも値型（Value Type）
- スタックまたはインライン配置
- 代入時は値コピー
- nullにできない（Nullable<`T`>除く）
- GC対象にならない（単体では）

つまりメモリ特性は同じ。

---

## 2. 最大の違い

違いは「自動生成される機能」。

------------------------------------------------------------

### struct

例:

    struct Point
    {
        public int X;
        public int Y;
    }

特徴:

- Equals は ValueType ベース（最適ではない）
- == 演算子は未定義（自分で実装必要）
- Deconstruct なし
- with式 なし
- ToString は単純

基本的に「ただの値の箱」。

------------------------------------------------------------

### record struct

例:

    record struct Point(int X, int Y);

自動生成されるもの:

- 値ベース Equals
- == / != 演算子
- IEquatable`<T>` 実装
- Deconstruct
- with式
- 読みやすい ToString()

つまり:

record struct = struct + 値オブジェクト機能

---

## 3. 等価性の違い

struct:

- フィールド単位の比較だが最適化されていない
- == は自前実装が必要

record struct:

- すべてのプロパティで値比較
- == 自動生成
- IEquatable`<T>` 実装済

値オブジェクト用途では record struct が明確に有利。

---

## 4. with式の有無

struct:

    var p2 = p1; // コピー
    p2.X = 10;

record struct:

    var p2 = p1 with { X = 10 };

不変設計と相性が良い。

---

## 5. パフォーマンス

両者とも値型。

- 小さいデータでは高速
- GC発生なし

ただし:

- 大きいstructはコピーコスト大
- 16byte超は慎重に設計

---

## 6. 実務での使い分け

ただの高速な値の箱 → struct

値オブジェクト（比較・DTO） → record struct

DDD的設計 → record struct

---
## 7. 可変structの危険性

値型 = 安全 ではない。

例:

    struct Counter
    {
        public int Value;
    }

    var a = new Counter { Value = 10 };
    var b = a;
    b.Value = 20;

aは10のまま。

問題はこれ:

    List<Counter> list = new();
    list.Add(new Counter { Value = 1 });

    list[0].Value = 5; // コンパイルエラー

理由:

- list[0] はコピーが返る
- コピーを書き換えている
- 元の値は変わらない

さらに:

- プロパティ経由でもコピーが発生
- readonlyフィールド経由で defensive copy 発生

結果:

- 意図しないバグ
- パフォーマンス低下

---

## 8. readonly の重要性

    readonly record struct Point(int X, int Y);

利点:

- 完全不変
- defensive copy抑制
- 意図が明確
- 値オブジェクトとして安全

structを使う場合も:

    readonly struct Point

が推奨。

---
## 9. 結論

struct と record struct の違いは:

record struct は
「値オブジェクト向けに強化された struct」。

値比較・with式・等価性を考慮するなら
record struct を選ぶべき。

可変structはバグの温床になりやすいため、
実務では readonly record struct が最も安全な選択。
