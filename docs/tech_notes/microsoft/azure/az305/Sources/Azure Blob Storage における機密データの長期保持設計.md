[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
# Azure Blob Storage における機密データの長期保持設計

（Standard GPv2 + Hot Tier + Immutability Policy）

---

# 1 背景

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#WORM]]
企業では監査ログ、契約データ、金融記録などの **機密データ**を長期間保存する必要がある。これらのデータは規制やコンプライアンス要件により、一定期間 **変更不可（WORM: Write Once Read Many）**として保持しなければならない場合が多い。

今回のシナリオでは以下の要件がある。

- 保存データ量：**10GB未満**
    
- データは **毎日アクセス可能**である必要がある
    
- データは **5年間保持**する必要がある
    
- 保存期間中は **変更不可（Immutable）**でなければならない
    
- 5年後には **削除可能**である必要がある
    
- **アクセスコストを最小化**する必要がある
    

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
この要件は Azure Storage の **Blob Storage 不変ポリシー（Immutable Blob Storage）**を利用することで実現できる。

---

# 2 Azure Blob Storage のアカウントタイプ

Azure Blob Storage のストレージアカウントには複数の種類が存在する。

|ストレージタイプ|特徴|
|---|---|
|GPv1|旧世代ストレージ|
|GPv2|最新の汎用ストレージ|
|BlockBlobStorage (Premium)|高性能・高コスト|

現在 Microsoft が推奨しているのは **Standard GPv2** である。

理由

- 最新機能対応
    
- 価格最適化
    
- ライフサイクル管理
    
- 不変ポリシー対応
    

---

# 3 Blob Storage のアクセス層

Azure Blob Storage には 3 種類のアクセス層が存在する。

|層|用途|
|---|---|
|Hot|頻繁アクセス|
|Cool|低頻度アクセス|
|Archive|長期アーカイブ|

今回の要件は

```text
毎日アクセス
```

であるため **Hot Tier** が最適となる。

---

# 4 各ストレージ層の特徴

### Hot Tier

- 頻繁アクセス向け
    
- 低アクセスコスト
    
- 高スループット
    

---

### Cool Tier

- 月数回アクセス
    
- ストレージ安価
    
- アクセスコスト高
    

---

### Archive Tier

- ほぼアクセスなし
    
- 復元に数時間
    
- 最低コスト
    

---

今回のケース

```text
Daily Access
```

のため **Hot Tier** が最適である。

---

# 5 不変ストレージ（Immutable Storage）

Azure Blob Storage には **不変性ポリシー（Immutability Policy）**がある。

これは **WORM (Write Once Read Many)** ストレージとして機能する。

保存期間中

- 削除不可
    
- 更新不可
    
- 上書き不可
    

が保証される。

---

# 6 不変ポリシーの種類

Azure Blob Storage では 2 種類の不変設定がある。

|タイプ|説明|
|---|---|
|Time-based retention|指定期間変更不可|
|Legal hold|法的保持|

今回の要件

```text
5年間保持
```

なので **Time-based retention policy** を使用する。

---

# 7 Time-based retention policy

設定例

```text
Retention Period = 5 years
```

動作

|期間|状態|
|---|---|
|保存期間中|変更不可|
|期間終了後|削除可能|

これにより

- コンプライアンス
    
- データ保護
    

が実現できる。

---

# 8 コンテナアクセスポリシー

コンテナアクセスポリシーは

- コンテナアクセス権限
    
- 認証管理
    

を制御する。

例

|アクセス|説明|
|---|---|
|Private|認証必要|
|Blob|Blob 読み取り可|
|Container|公開|

機密データの場合

```text
Private container
```

が一般的である。

---

# 9 推奨アーキテクチャ

今回の最適構成

```text
Azure Storage Account (Standard GPv2)
        │
        ▼
Blob Container
        │
        ├─ Access Tier : Hot
        │
        └─ Immutability Policy
                │
                ▼
          Retention : 5 years
```

---

# 10 ストレージ選択理由

### Standard GPv2

理由

- 最新機能
    
- 不変ポリシー対応
    
- コスト効率
    

---

### Hot Tier

理由

- 毎日アクセス
    
- 最低アクセスコスト
    

---

### Immutable Blob

理由

- 5年変更不可
    
- WORM対応
    

---

# 11 他の選択肢が不適切な理由

## Cool Tier

問題

- アクセスコストが高い
    
- 毎日アクセスに不向き
    

---

## Premium Block Blob

問題

- 高コスト
    
- パフォーマンス過剰
    

---

## Archive Tier

問題

- 取り出しに時間
    
- 日常アクセス不可
    

---

## GPv1

問題

- 古いストレージ
    
- 新機能非対応
    

---

# 12 最終回答

正解

**B**

```text
Standard GPv2 + Hot Tier + Container Access Policy
```

（実運用では Immutability Policy を設定）

---

# 13 まとめ

今回の要件を整理すると

|要件|解決策|
|---|---|
|毎日アクセス|Hot Tier|
|低コスト|Standard Storage|
|5年保持|Immutability Policy|
|変更不可|WORM Storage|

そのため最適構成は

```text
Standard GPv2
+ Hot Tier
+ Time-based Immutability Policy
```

となる。

この構成は Azure の **コンプライアンスデータ保存設計の標準パターン**であり、金融・医療・監査ログなどの保存に広く利用されている。