[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# Azure SQL Database 設計ドキュメント

（PII データ保護・可用性・自動スケーリング・コスト最適化）

---

# 1 背景

企業では、従業員情報を管理する **Web アプリケーション**を構築している。  
このアプリケーションでは、従業員の個人情報（PII : Personally Identifiable Information）を **Azure SQL Database** に保存する予定である。

従業員情報には次のような機密データが含まれる可能性がある。

- 氏名
    
- 社員番号
    
- 住所
    
- 社会保障番号
    
- 給与情報
    

このような情報は法規制やセキュリティポリシーの対象となるため、**適切な暗号化とアクセス制御が必要**となる。

また、給与計算などの処理は特定の時間帯に集中することが多く、通常時と比較してデータベースの負荷が急激に増加する場合がある。そのため、インフラ設計では **自動スケーリング機能**を活用することで、必要な時だけリソースを増やし、コストを抑えることが望ましい。

さらに、このシステムは企業の業務システムであるため、**データセンター障害時にもサービスが利用可能であること**が求められる。

---

# 2 要件整理

この問題では、次の4つの要件を満たす必要がある。

### ① データセンター障害時の可用性

Azure データセンターが停止した場合でも、サービスを継続できる必要がある。  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Failover Group]]
この要件は **Geo レプリケーションや Failover Group** を利用することで実現できる。

---

### ② PII データの暗号化

PII データは特定の列に保存されるため、**列単位の暗号化（Column Level Encryption）**が必要となる。

SQL Database では次の2種類の暗号化方式がある。

|暗号化方式|説明|
|---|---|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Transparent Data Encryption (TDE)]]
|TDE (Transparent Data Encryption)|データを保存時に暗号化|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Always Encrypted]]
|Always Encrypted|特定列を暗号化（使用中も保護）|

今回の要件では

```text
特定のPII列を暗号化
```

とあるため

**Always Encrypted**

を使用する必要がある。

---

### ③ 自動スケーリング

給与計算のタイミングでデータベースの負荷が急増する可能性がある。

例えば

- 月末給与処理
    
- ボーナス計算
    
- 年末調整
    

これらのタイミングでは CPU 使用率やクエリ処理量が増加する。

このような負荷変動に対応するためには **Serverless コンピュートモデル**を使用するのが適している。

Serverless モデルでは

- ワークロードに応じて自動スケール
    
- 使用していない時間は自動停止
    

といった機能を利用できる。

---

### ④ コスト最小化

クラウド設計では **必要以上のリソースを確保しないこと**が重要である。

Serverless モデルでは次のような料金体系となる。

- 使用したコンピュート時間のみ課金
    
- アイドル時は停止
    

そのため、負荷の波があるアプリケーションでは **Serverless が最もコスト効率が良い**。

---

# 3 Azure SQL Database のサービス階層

Azure SQL Database には複数のサービス階層が存在する。

|階層|特徴|
|---|---|
|General Purpose|一般用途|
|Business Critical|高性能・低レイテンシ|
|Hyperscale|超大規模データ|

---

## General Purpose

General Purpose は最も一般的な階層であり、多くのアプリケーションに適している。

特徴

- コスト効率が高い
    
- Serverless モードを利用可能
    
- 多くの業務アプリに十分な性能
    

---

## Business Critical

Business Critical は非常に高い I/O 性能を提供する階層である。

特徴

- 低レイテンシ
    
- 高いトランザクション性能
    
- 高いコスト
    

この階層は主に次の用途で使用される。

- 高頻度トランザクション
    
- 大規模 OLTP
    
- 高性能要求アプリ
    

---

## Hyperscale

Hyperscale は非常に大きなデータベース向けの構成である。

特徴

- 数十 TB 以上のデータ
    
- 分散ストレージ
    
- 高スケール
    

ただしコストが高く、一般的な業務アプリでは過剰となることが多い。

---

# 4 Always Encrypted

Always Encrypted は SQL Database の高度な暗号化機能であり、特定の列を暗号化することができる。

この暗号化方式では

- データはクライアント側で暗号化される
    
- SQL Server は暗号化された状態のデータのみ扱う
    
- 管理者でも平文を閲覧できない
    

構造は次のようになる。

```text
Application
     │
     │ Encryption
     ▼
Encrypted Data
     │
     ▼
Azure SQL Database
```

そのため、PII データなどの機密情報の保護に適している。

---

# 5 Serverless モデル

Serverless コンピュートは、Azure SQL Database の自動スケーリング機能である。

特徴

- CPU 自動スケーリング
    
- アイドル時自動停止
    
- 秒単位課金
    

ワークロードが変動するアプリケーションでは、非常にコスト効率が高い。

給与計算システムのように

```text
通常時 → 低負荷
給与処理 → 高負荷
```

というパターンでは、Serverless が最適な構成となる。

---

# 6 可用性設計

データセンター障害への対応には **Geo レプリケーション**または **Failover Group**を利用する。

構成イメージ

```text
Primary Region
      │
      │ Replication
      ▼
Secondary Region
```

この構成により、リージョン障害時でもサービスを継続できる。

---

# 7 最適構成

今回の要件を整理すると次の通りである。

|要件|必要機能|
|---|---|
|PII列暗号化|Always Encrypted|
|自動スケール|Serverless|
|コスト最小|General Purpose|
|可用性|Geo レプリケーション|

したがって最適構成は

```text
General Purpose
+
Serverless
+
Always Encrypted
```

---

# 8 最終回答

正解は

**B**

```text
General Purpose
Serverless
Always Encrypted
```

---

# 9 まとめ

Azure SQL Database の設計では、次の3つの観点が重要となる。

```text
Compute Model
Service Tier
Encryption
```

今回の設計では

- Serverless → 自動スケール
    
- General Purpose → コスト効率
    
- Always Encrypted → PII保護
    

という組み合わせが最も適した構成となる。