[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# Azure SQL Database の購入モデルと展開設計

## 多数のデータベースを低コストかつ自動スケールで運用するアーキテクチャ

---

# 1 背景（シナリオ）

企業がクラウド環境で SaaS アプリケーションやデータサービスを運用する場合、複数のデータベースを効率的に管理する必要がある。

今回のシナリオでは、顧客向けに **50 個の SQL Server データベース**を Azure に展開する必要がある。

さらに、次の条件を満たす必要がある。

### 要件

1. ソフトウェア保証（Software Assurance）を利用した **ライセンスモビリティ**
    
2. データベースの **自動スケーリング**
    
3. **SQL Server ライセンスコストの最小化**
    
4. **50 個のデータベースを効率的に管理**
    

この要件から、単純に 50 個のデータベースを個別にデプロイする方法ではコストと管理負荷が大きくなる可能性がある。

そのため、Azure SQL Database の **購入モデルと展開方式**を適切に選択する必要がある。

---

# 2 Azure SQL Database の購入モデル

Azure SQL Database には主に 2 つの購入モデルがある。

|購入モデル|特徴|
|---|---|
|DTU モデル|CPU・IO・メモリをまとめた単位|
|vCore モデル|CPU コアベースの課金|

---

# 3 DTU モデル

DTU（Database Transaction Unit）は、SQL Database の古い購入モデルである。

DTU は次のリソースを 1 つの単位としてまとめて提供する。

|リソース|
|---|
|CPU|
|メモリ|
|I/O|

例

|DTU レベル|用途|
|---|---|
|Basic|軽量|
|Standard|中規模|
|Premium|高性能|

しかし DTU モデルには次の欠点がある。

- リソースの内訳が不透明
    
- ライセンス持ち込みができない
    
- スケーリングの柔軟性が低い
    

---

# 4 vCore モデル

vCore モデルは Azure SQL Database の **最新の購入モデル**である。

特徴

|特徴|説明|
|---|---|
|CPU単位課金|vCore数で課金|
|ストレージ分離|ストレージ個別課金|
|透明性|リソース把握が容易|

さらに重要なのは

```text
Azure Hybrid Benefit
```

を利用できる点である。

---

# 5 ライセンスモビリティ

ソフトウェア保証（Software Assurance）を持つ顧客は

```text
Azure Hybrid Benefit
```

を利用できる。

これにより

```text
オンプレ SQL Server ライセンス
↓
Azure で利用可能
```

になる。

結果として

```text
SQL Server ライセンスコスト削減
```

が可能になる。

この機能は **vCore モデルで利用できる**。

---

# 6 Azure SQL Database の展開方式

Azure SQL Database には複数の展開方式がある。

|展開方式|説明|
|---|---|
|Single Database|単一 DB|
|Elastic Pool|複数 DB 共有|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
|Managed Instance|SQL Server 互換 PaaS|

---

# 7 Single Database の問題

単一データベースを 50 個作成する場合

```text
50 × 専用リソース
```

が必要になる。

つまり

- CPU
    
- メモリ
    
- I/O
    

をそれぞれ確保する必要がある。

この方法は

```text
リソースの無駄
```

が発生しやすい。

---

# 8 Elastic Pool

Elastic Pool は **複数データベースでリソースを共有する仕組み**である。

構造

```text
Elastic Pool
 │
 ├ Database 1
 ├ Database 2
 ├ Database 3
 └ Database n
```

すべてのデータベースが **同じ CPU / メモリ / I/O プールを共有する**。

---

# 9 Elastic Pool のメリット

Elastic Pool の主なメリット

|メリット|説明|
|---|---|
|コスト削減|リソース共有|
|自動スケール|ワークロード変動対応|
|管理簡素化|DB 個別管理不要|

例えば

```text
DB1 高負荷
DB2 低負荷
```

の場合

```text
未使用リソースを共有
```

できる。

---

# 10 SaaS アーキテクチャと Elastic Pool

Elastic Pool は特に **SaaS アプリケーション**に適している。

SaaS では

```text
Customer1 DB
Customer2 DB
Customer3 DB
```

のように **顧客ごとに DB を分ける**ことが多い。

しかし負荷は均一ではない。

Elastic Pool を使うと

```text
複数 DB の負荷を平均化
```

できる。

---

# 11 自動スケーリング

Elastic Pool は

```text
複数データベース間で自動スケール
```

する。

つまり

```text
DB1 が高負荷
↓
プールの余剰 CPU 利用
```

が可能になる。

---

# 12 最適構成

今回の要件

|要件|必要機能|
|---|---|
|ライセンス削減|Azure Hybrid Benefit|
|自動スケール|Elastic Pool|
|複数 DB 管理|Elastic Pool|

この要件を満たす組み合わせは

```text
vCore + Elastic Pool
```

である。

---

# 13 他の選択肢が不適切な理由

### A

DTU + Single Database

問題

- ライセンスモビリティ不可
    
- リソース共有不可
    
- コスト増加
    

---

### C

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
Reserved VM + Managed Instance

これは

```text
IaaS ベース
```

であり、Elastic Pool のような

```text
クロスデータベーススケーリング
```

ができない。

---

### D

vCore + Always On

Always On は

```text
高可用性
```

機能であり

```text
スケーリング
```

ではない。

---

# 14 最終アーキテクチャ

```text
Azure SQL Elastic Pool
       │
       ├ Database1
       ├ Database2
       ├ Database3
       ├ ...
       └ Database50
```

購入モデル

```text
vCore
+
Azure Hybrid Benefit
```

---

# 15 最終回答

正解

```text
B

購入モデル: vCore  
展開方式: Elastic Pool
```

---

# 16 まとめ

今回の問題の重要ポイントは

**多数のデータベースを効率的に運用する設計**である。

|項目|最適選択|
|---|---|
|購入モデル|vCore|
|ライセンス最適化|Azure Hybrid Benefit|
|展開|Elastic Pool|

つまり最適構成は

```text
vCore
+
Elastic Pool
```

である。

この構成は **SaaS アプリケーションのマルチテナントデータベース設計の標準パターン**である。