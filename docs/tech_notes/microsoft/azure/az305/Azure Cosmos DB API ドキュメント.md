# Azure Cosmos DB API ドキュメント

（SQL API / MongoDB API / Cassandra API / Gremlin API / Table API）

## 1. 概要

Azure Cosmos DBは **グローバル分散型NoSQLデータベースサービス**です。  
低レイテンシ、高可用性、水平スケーリングを特徴とし、クラウドアプリケーションや大規模データ処理に使用されます。

Cosmos DBの特徴は **複数のデータモデルをAPIとして提供する**点です。  
同じCosmos DB基盤を使いながら、異なるデータモデルに対応できます。

主な特徴

- グローバル分散
    
- ミリ秒レベルの低レイテンシ
    
- 自動スケーリング
    
- マルチモデルデータベース
    
- SLA保証
    

---

# 2. Cosmos DB の基本構造

Cosmos DBのデータ構造は次のようになります。

```id="m7y9df"
Cosmos DB Account
        │
        └ Database
             │
             └ Container
                   │
                   └ Items (Documents)
```

主な概念

|要素|説明|
|---|---|
|Account|Cosmos DBインスタンス|
|Database|データベース|
|Container|コレクション|
|Item|JSONドキュメント|

---

# 3. Cosmos DB API

Cosmos DBでは **複数のAPI** を提供しています。

|API|データモデル|
|---|---|
|SQL API|ドキュメント|
|MongoDB API|ドキュメント|
|Cassandra API|ワイドカラム|
|Gremlin API|グラフ|
|Table API|Key-Value|

これにより、既存アプリケーションの移行や特定のデータモデルに適した設計が可能になります。

---

# 4. SQL API

SQL APIは **Cosmos DBのネイティブAPI**です。

特徴

- JSONドキュメント保存
    
- SQL風クエリ
    
- 高性能
    
- 最も一般的
    

データ例

```json
{
  "id": "1",
  "name": "Taro",
  "age": 30
}
```

クエリ例

```sql
SELECT * FROM c WHERE c.age > 25
```

用途

- Webアプリ
    
- モバイルアプリ
    
- IoT
    
- マイクロサービス
    

---

# 5. MongoDB API

MongoDB APIは **MongoDB互換API**です。

特徴

- MongoDBドライバ使用可能
    
- BSON/JSONドキュメント
    
- Mongoクエリ
    

例

```javascript
db.users.find({ age: { $gt: 25 } })
```

用途

- MongoDBアプリ移行
    
- ドキュメントデータ
    

メリット

- MongoDBツール利用可能
    
- 既存アプリ変更不要
    

---

# 6. Cassandra API

Cassandra APIは **Apache Cassandra互換API**です。

データモデル

ワイドカラムストア

構造

```id="zvafsu"
Table
 ├ Partition Key
 ├ Clustering Key
 └ Columns
```

特徴

- 高スループット
    
- 分散データ
    
- Cassandraクエリ
    

用途

- IoT
    
- ログデータ
    
- 大規模分析
    

---

# 7. Gremlin API

Gremlin APIは **グラフデータベース**です。

データモデル

```id="h0zvdu"
Vertex (Node)
Edge (Relationship)
```

例

```id="q8pnhh"
User
  │
  ├ Friend
  │
  └ Follow
```

クエリ例

```gremlin
g.V().has("name","Taro").out("friend")
```

用途

- SNS
    
- 推薦システム
    
- ネットワーク分析
    

---

# 8. Table API

Table APIは **Key-Valueデータストア**です。

Azure Table Storage互換です。

構造

```id="8qslwr"
PartitionKey
RowKey
Properties
```

特徴

- シンプル構造
    
- 高速Key検索
    

用途

- セッションデータ
    
- メタデータ
    
- シンプルNoSQL
    

---

# 9. API比較

|API|データモデル|用途|
|---|---|---|
|SQL API|JSONドキュメント|Webアプリ|
|MongoDB API|JSONドキュメント|Mongo移行|
|Cassandra API|ワイドカラム|IoT|
|Gremlin API|グラフ|SNS|
|Table API|Key-Value|シンプルNoSQL|

---

# 10. JSONドキュメント対応API

JSONドキュメントに対応するAPI

|API|理由|
|---|---|
|SQL API|JSONネイティブ|
|MongoDB API|JSONドキュメント|

---

# 11. Cosmos DB アーキテクチャ例

典型構成

```id="51p2y6"
Web App
   │
   ▼
Azure Cosmos DB
   │
   ├ SQL API
   ├ MongoDB API
   ├ Cassandra API
   └ Gremlin API
```

---

# 12. マイクロサービスアーキテクチャ例

```id="tbx9lj"
Microservice A
   │
   ▼
Cosmos DB SQL API

Microservice B
   │
   ▼
Cosmos DB Mongo API

Microservice C
   │
   ▼
Cosmos DB Gremlin API
```

---

# 13. Cosmos DB の主なユースケース

主な用途

- IoTデータ
    
- マイクロサービス
    
- セッション管理
    
- カタログデータ
    
- リアルタイムアプリ
    

---

# 14. API選択の設計指針

API選択は **データモデルで決まります**。

|データタイプ|API|
|---|---|
|JSONドキュメント|SQL / MongoDB|
|グラフ|Gremlin|
|ワイドカラム|Cassandra|
|Key-Value|Table|

---

# 15. まとめ

Azure Cosmos DBは **マルチモデルNoSQLデータベース**です。

提供API

```id="v3lsyb"
Cosmos DB
   │
   ├ SQL API (Document)
   ├ MongoDB API (Document)
   ├ Cassandra API (Column)
   ├ Gremlin API (Graph)
   └ Table API (Key-Value)
```

JSONドキュメントを扱う場合は

**SQL API または MongoDB API**

を使用します。