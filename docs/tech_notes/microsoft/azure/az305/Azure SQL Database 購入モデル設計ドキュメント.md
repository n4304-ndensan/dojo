# Azure SQL Database 購入モデル設計ドキュメント

（vCore / DTU / Serverless / Elastic Pool）

## 1. 概要

Azure SQL Databaseは **PaaS型リレーショナルデータベースサービス**であり、アプリケーションの要件に応じて複数の購入モデル（Pricing Model）が提供されています。

Azure SQL Databaseを設計する際には次の要素を検討する必要があります。

主な設計要素

* コンピュート性能
* ストレージ容量
* コストモデル
* スケーラビリティ
* ライセンス利用
* バックアップと冗長性

Azure SQL Databaseには主に次の購入モデルがあります。

| 購入モデル     | 概要                   |
| --------- | -------------------- |
| DTU モデル   | リソースがパッケージ化されたモデル    |
| vCore モデル | コンピュートとストレージを分離したモデル |

さらにデプロイ方式として以下があります。

| デプロイ方式          | 概要          |
| --------------- | ----------- |
| Single Database | 単一データベース    |
| Elastic Pool    | 複数DBでリソース共有 |
| Serverless      | 自動スケール型     |

---

# 2. Azure SQL Database アーキテクチャ

Azure SQL Databaseは次のような構成で提供されます。

```id="sq7dk1"
Application
    │
    ▼
Azure SQL Database
    │
    ├ Compute
    ├ Storage
    ├ Backup
    └ High Availability
```

Azureが以下を管理します。

* OS
* SQL Server
* パッチ
* バックアップ
* 高可用性

---

# 3. DTU モデル

DTU（Database Transaction Unit）は、Azure SQL Databaseの初期の購入モデルです。

DTUは次のリソースをまとめた単位です。

```id="6e4p8q"
DTU
 ├ CPU
 ├ Memory
 └ IO
```

つまり、個別に設定できません。

例

| サービス層    | DTU        |
| -------- | ---------- |
| Basic    | 5 DTU      |
| Standard | 10–300 DTU |
| Premium  | 最大4000 DTU |

---

## DTU モデルの特徴

メリット

* シンプル
* 小規模アプリに適している

デメリット

* CPUとストレージを分離できない
* コスト最適化が難しい
* Azure Hybrid Benefit利用不可

---

# 4. vCore モデル

vCoreモデルは **現在推奨されている購入モデル**です。

特徴

* CPU（vCore）を選択可能
* ストレージを独立設定
* 高い柔軟性

構造

```id="59n6j7"
vCore Model
   │
   ├ Compute (vCore)
   ├ Memory
   ├ Storage
   └ Backup Storage
```

---

## vCore モデルの主な特徴

### 1. コンピュートとストレージの独立設定

例

```id="zsd1ji"
Compute = 8 vCore
Storage = 1 TB
```

この柔軟性により

* コスト最適化
* ワークロード調整

が可能です。

---

### 2. バックアップストレージレプリケーション

Azure SQL Databaseではバックアップの冗長性を選択できます。

| レプリケーション | 説明         |
| -------- | ---------- |
| LRS      | ローカル冗長     |
| ZRS      | ゾーン冗長      |
| GRS      | 地理冗長       |
| RA-GRS   | 読み取り可能地理冗長 |

---

### 3. Azure Hybrid Benefit

Azure Hybrid Benefitは

> 既存のSQL ServerライセンスをAzureで使用できる仕組み

です。

メリット

* SQLライセンスコスト削減
* 最大約40%コスト削減

---

### 4. Reserved Capacity

Reserved Capacityとは

**長期予約による割引**

です。

例

```id="vb03tm"
1年予約
3年予約
```

これにより

* 最大約33〜40%割引

が可能になります。

---

# 5. Serverless モデル

Serverlessは **自動スケール型データベース**です。

特徴

* コンピュート自動スケール
* 使用量ベース課金
* 自動停止

構造

```id="fhfa0g"
Workload
   │
   ▼
Auto Scale Compute
   │
   ▼
Pause when idle
```

用途

* 不定期ワークロード
* 開発環境
* テスト環境

---

# 6. Elastic Pool

Elastic Poolは **複数データベースのリソース共有モデル**です。

構造

```id="pf2vt1"
Elastic Pool
   │
   ├ Database A
   ├ Database B
   └ Database C
```

特徴

* 複数DBがCPUを共有
* コスト最適化

用途

* SaaSアプリ
* マルチテナントDB

---

# 7. サービス層

vCoreモデルには複数のサービス層があります。

| サービス層             | 用途      |
| ----------------- | ------- |
| General Purpose   | 一般アプリ   |
| Business Critical | 高性能OLTP |
| Hyperscale        | 超大容量DB  |

---

# 8. Hyperscale

Hyperscaleは

**超大規模データベース用**

です。

特徴

* 最大100TB
* 高速スケール
* 分散ストレージ

---

# 9. Azure SQL Database バックアップ

Azure SQL Databaseは自動バックアップを提供します。

バックアップタイプ

* Full Backup
* Differential Backup
* Transaction Log Backup

保持期間

* 7〜35日（標準）
* 長期保存（LTR）

---

# 10. SQL Database 設計例

典型構成

```id="3z5lkl"
Web Application
      │
      ▼
Azure SQL Database (vCore)
      │
      ├ Compute
      ├ Storage
      ├ Backup
      └ High Availability
```

---

# 11. 購入モデル比較

| モデル          | 特徴         |
| ------------ | ---------- |
| DTU          | シンプルだが柔軟性低 |
| vCore        | 柔軟性高（推奨）   |
| Serverless   | 自動スケール     |
| Elastic Pool | 複数DB共有     |

---

# 12. モデル選択指針

| 用途        | モデル          |
| --------- | ------------ |
| 小規模DB     | DTU          |
| 柔軟なリソース管理 | vCore        |
| 断続的ワークロード | Serverless   |
| マルチテナント   | Elastic Pool |

---

# 13. まとめ

Azure SQL Databaseの購入モデルで最も柔軟なのは

**vCore モデル**

です。

vCoreモデルのメリット

* コンピュートとストレージ分離
* バックアップ冗長性選択
* Azure Hybrid Benefit
* Reserved Capacity割引

そのため、企業の高度な要件を満たす設計では

**vCoreモデルが推奨されます。**
