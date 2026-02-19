# C# class / struct / record / record struct の違いまとめ

対象:
- class
- struct
- record
- record struct

C# 10+ 前提

---

## 1. メモリ特性

### class

- [[DotNETメモリ配置#8. 値型と参照型の違い|参照型]]
- [[DotNETメモリ配置#3. Heap（ヒープ）|ヒープ]]に配置
- 変数は参照（アドレス）を保持

### struct

- [[DotNETメモリ配置#8. 値型と参照型の違い|値型]]
- スタックまたはインライン配置
- 変数は値そのものを保持

### record

- 参照型（内部的にはclass）
- ヒープに配置

### record struct

- 値型
- structと同じ配置特性

---

## 2. コピーの挙動

### class

- 代入時は参照コピー
- 同じインスタンスを共有

### struct

- 代入時は値コピー
- 完全に別インスタンス

### record

- 参照型だが
- with式でコピー生成可能
- 不変設計を前提とする

### record struct

- 値コピー
- with式も使用可能

---

## 3. 等価性（Equals ）

### class

- デフォルトは参照比較
- 値比較にはオーバーライドが必要

### struct

- フィールド単位の値比較（ValueType.Equals）

### record

- 値ベース比較（全プロパティ）
- == も値比較

### record struct
- 値ベース比較
- == も値比較

---

## 4. 主目的

### class

- 振る舞いを持つオブジェクト
- ビジネスロジック中心

### struct

- 小さな値オブジェクト
- 座標・数値など

### record

- 不変データモデル
- DTO / ValueObject

### record struct

- 軽量な値オブジェクト
- 高頻度生成データ

---

## 5. 不変性（Immutability）

### class

- 可変が普通

### struct

- 可変も可能だが推奨しない

### record

- init専用プロパティが基本
- 不変前提

### record struct

- readonly と併用推奨

例:

    readonly record struct Point(int X, int Y);

---

## 6. パフォーマンス観点

### class

- GC対象
- 大量生成でGC負荷増

### struct

- GC負荷低
- 大きいstructはコピーコスト高

### record

- classと同様GC対象

### record struct

- GC削減可能
- 小さいデータ向き

---

## 7. 実務での使い分け

ビジネスロジック中心 → class  
DTO / APIレスポンス → record  
小さい値オブジェクト → readonly record struct  
高頻度生成軽量データ → record struct  

---

## 8. あなたの例

    private record SearchTarget
    {
        string SearchDirectory
        string Pattern
    }

これは値オブジェクトなので推奨は:

    private readonly record struct SearchTarget(
        string SearchDirectory,
        string Pattern
    );

理由:

- 小型データ
- 値比較が自然
- 不変が適切
- GC削減

---

## 9. 注意点

struct / record struct は:

- nullにできない（Nullable<`T`>を除く）
- boxingが発生するケースあり
- 大きいサイズは避ける（目安: 16byte超は慎重）

---

## 10. 比較表

| 種類            | 型   | 等価性  | GC対象 | 主用途           |
| ------------- | --- | ---- | ---- | ------------- |
| class         | 参照型 | 参照比較 | あり   | ロジック          |
| struct        | 値型  | 値比較  | なし   | 小型データ         |
| record        | 参照型 | 値比較  | あり   | 不変DTO         |
| record struct | 値型  | 値比較  | なし   | 軽量ValueObject |


---

## 結論

SearchTarget のような:

- データ保持のみ
- 小型
- 値比較したい

ケースでは

    readonly record struct

が最適。

---

## 追加参照

[[CS_struct_struct-record違い]]
