[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# Azure SQL Database のクエリパフォーマンス自動改善

（Automatic Tuning – Force Plan）

---

# 1 背景

企業が運用する Azure SQL Database では、営業時間中に次のような問題が発生することがある。

- クエリの応答が遅い
    
- アプリケーションでタイムアウトが発生する
    
- 一部のユーザーのみパフォーマンス低下を報告する
    
- 同じクエリでも実行時間が大きく変動する
    

今回のシナリオでは、Azure SQL Database は **Elastic Pool（弾性プール）** 内に配置されているが、それでもパフォーマンスの不安定性が発生している。

さらに重要な条件として、

- **アプリケーションを変更しない**
    
- **自動的にパフォーマンスを改善する**
    

必要がある。

この要件を満たす Azure SQL Database の機能は

**Automatic Tuning（自動チューニング） – Force Plan**

である。

---

# 2 クエリパフォーマンス低下の主な原因

Azure SQL Database では、クエリのパフォーマンス低下は主に次の原因で発生する。

### 1 Query Plan Regression

SQL Server ではクエリ実行時に **Query Execution Plan** が作成される。

```text
SQL Query
   │
   ▼
Query Optimizer
   │
   ▼
Execution Plan
   │
   ▼
Database Engine
```

しかし統計情報の更新などにより、より遅い実行計画が選択されることがある。

これを **Query Plan Regression（計画退化）** と呼ぶ。

---

### 2 統計情報の変化

データ量が増加すると統計情報が変化し、オプティマイザが誤った実行計画を選択することがある。

---

### 3 パラメータスニッフィング

パラメータによって最適なクエリプランが変化する問題である。

---

# 3 Automatic Tuning

Azure SQL Database には **Automatic Tuning（自動チューニング）** という機能がある。

これは AI ベースの最適化機能であり、クエリのパフォーマンス問題を自動的に検出し修正する。

主な機能

|機能|説明|
|---|---|
|Force Plan|最適なクエリプランを固定|
|Create Index|推奨インデックス作成|
|Drop Index|不要インデックス削除|

今回のシナリオでは **Force Plan** が重要になる。

---

# 4 Force Plan（強制プラン）

Force Plan は、SQL Server が過去に使用した **最も高速な実行プランを固定する機能**である。

仕組み

```text
Query Execution History
       │
       ▼
Best Plan Detected
       │
       ▼
Force Plan
```

Azure SQL はクエリ履歴を分析し、パフォーマンスの良いプランを自動的に適用する。

---

# 5 Query Plan Regression の自動修正

Automatic Tuning は、次のような動作を行う。

1. クエリパフォーマンスを監視
    
2. 実行プラン変更を検知
    
3. パフォーマンス低下を検出
    
4. 過去の高速プランを強制適用
    

処理フロー

```text
Query Execution
      │
      ▼
Performance Monitoring
      │
      ▼
Regression Detected
      │
      ▼
Force Previous Plan
```

このプロセスは完全に自動化されている。

---

# 6 Elastic Pool との関係

Elastic Pool は複数のデータベースでリソースを共有する仕組みである。

```text
Elastic Pool
 ├ DB1
 ├ DB2
 └ DB3
```

しかし Elastic Pool は

- CPU共有
    
- メモリ共有
    

を行うだけであり、クエリ最適化を行う機能ではない。

そのため Elastic Pool を使用していても、クエリパフォーマンス問題は解決されない場合がある。

---

# 7 Query Performance Insight との違い

Query Performance Insight は **監視ツール**である。

できること

- クエリの実行時間分析
    
- ボトルネック検出
    
- 可視化
    

しかし

- 自動最適化は行わない
    
- 手動チューニングが必要
    

今回の要件は **自動改善**なので適さない。

---

# 8 他の選択肢の評価

## Query Performance Insight

役割

- パフォーマンス監視
    
- 可視化
    

問題

- 自動修正しない
    

---

## Scale-out Replica

読み取り専用レプリカを作成する機能。

用途

- 読み取りスケーリング
    

問題

- クエリ最適化とは無関係
    

---

## In-Memory OLTP

メモリテーブルによる高速処理機能。

問題

- アプリケーション変更が必要
    
- スキーマ変更が必要
    

今回の要件

**アプリケーション変更なし**

に反する。

---

# 9 推奨アーキテクチャ

Azure SQL Database の自動最適化構成

```text
Application
     │
     ▼
Azure SQL Database
     │
     ▼
Automatic Tuning
     │
     ├ Force Plan
     ├ Create Index
     └ Drop Index
```

これにより

- クエリプラン退化を防止
    
- 自動パフォーマンス改善
    
- アプリ変更不要
    

が実現される。

---

# 10 有効化方法

Azure Portal で次の設定を有効にする。

```
Azure SQL Database
   → Automatic tuning
       → Force Plan = ON
```

または T-SQL

```sql
ALTER DATABASE CURRENT
SET AUTOMATIC_TUNING (FORCE_LAST_GOOD_PLAN = ON);
```

---

# 11 まとめ

今回のシナリオ

- Azure SQL Database
    
- クエリタイムアウト
    
- 不安定なパフォーマンス
    
- アプリケーション変更なし
    

この問題を自動的に解決する機能は

**Automatic Tuning – Force Plan**

である。

この機能は

- クエリプラン退化の検出
    
- 最適プランの自動適用
    
- 継続的なパフォーマンス監視
    

を行うため、Azure SQL Database のパフォーマンス問題を最小の運用コストで改善できる。