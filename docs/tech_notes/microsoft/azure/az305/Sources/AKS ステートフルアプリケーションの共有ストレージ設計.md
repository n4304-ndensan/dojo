[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
# AKS ステートフルアプリケーションの共有ストレージ設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
（Azure Files + ReadWriteMany）

---

# 1 背景とシナリオ

Kubernetes 上でアプリケーションを実行する場合、多くのワークロードは **ステートレス（stateless）** として設計される。しかし実際の業務システムでは、以下のようにデータを保持する **ステートフル（stateful）アプリケーション** が必要になる。

例

- 分散アプリケーションのログ保存
    
- 共有キャッシュ
    
- 共有設定ファイル
    
- 分散処理の中間データ
    
- 機械学習のモデルファイル
    
- コンテンツ管理システム
    

このようなアプリケーションでは、複数の Pod から同じストレージにアクセスできる必要がある。また Kubernetes 環境では Pod の再作成やスケールアウトが頻繁に発生するため、ストレージには次の要件が求められる。

必要要件

- 永続ストレージ（Pod再作成後もデータ保持）
    
- 複数Podから同時アクセス
    
- 動的プロビジョニング
    
- 高スループット
    

これらの要件を満たす Azure のストレージソリューションとして推奨されるのが **Azure Files（ReadWriteMany）** である。

---

# 2 Kubernetes における永続ストレージ

Kubernetes では永続ストレージは次の仕組みで管理される。

主要コンポーネント

- Persistent Volume（PV）
    
- Persistent Volume Claim（PVC）
    
- Storage Class
    

構造は次のようになる。

```text
Pod
 │
 ▼
PersistentVolumeClaim
 │
 ▼
StorageClass
 │
 ▼
Cloud Storage
```

AKS では CSI（Container Storage Interface）ドライバーが Azure ストレージと Kubernetes を統合する。

---

# 3 Kubernetes のアクセスモード

Kubernetes のストレージにはアクセスモードという概念がある。これは **どのようにボリュームにアクセスできるか** を定義する。

主なアクセスモード

|モード|意味|
|---|---|
|ReadWriteOnce (RWO)|1ノードのみ読み書き|
|ReadOnlyMany (ROX)|複数ノード読み取り|
|ReadWriteMany (RWX)|複数ノード読み書き|

今回の要件

- 複数 Pod が同時アクセス
    
- 複数ノードからアクセス可能
    

したがって必要なモードは

**ReadWriteMany (RWX)**

である。

---

# 4 Azure Files

Azure Files は Azure の **マネージドファイル共有サービス**であり、SMB または NFS プロトコルを利用してファイルシステムとしてアクセスできる。

特徴

- フルマネージド
    
- SMB / NFS サポート
    
- Kubernetes CSI ドライバー対応
    
- 動的プロビジョニング
    
- 複数ノード同時アクセス
    

Azure Files は Kubernetes の **RWX ストレージ**として利用できる。

---

# 5 Azure Files のアーキテクチャ

AKS と Azure Files の構成は次のようになる。

```text
Pods
 │ │ │
 ▼ ▼ ▼
Kubernetes Volume
 │
 ▼
Azure Files CSI Driver
 │
 ▼
Azure Storage Account
 │
 ▼
Azure File Share
```

複数の Pod が同じファイル共有をマウントできる。

---

# 6 動的プロビジョニング

動的プロビジョニングとは、PVC を作成すると Kubernetes が自動でストレージを作成する仕組みである。

例

```yaml
apiVersion: v1
kind: PersistentVolumeClaim
metadata:
  name: shared-storage
spec:
  accessModes:
    - ReadWriteMany
  storageClassName: azurefile
  resources:
    requests:
      storage: 100Gi
```

この設定により

- Azure File Share が自動作成
    
- Pod が自動マウント
    

される。

---

# 7 高スループット

Azure Files は次の SKU を選択できる。

|SKU|特徴|
|---|---|
|Standard|HDDベース|
|Premium|SSDベース|

高スループットが必要な場合は

**Premium Azure Files**

を使用する。

特徴

- 低レイテンシ
    
- 高 IOPS
    
- 高帯域幅
    

---

# 8 他の選択肢との比較

## Azure Disk (ReadWriteOnce)

Azure Disk は **高性能ブロックストレージ**である。

特徴

- 非常に低レイテンシ
    
- 高IOPS
    
- 高スループット
    

しかし問題がある。

RWO の場合

```text
1 node only
```

つまり

- 複数ノードから同時アクセス不可
    
- 複数 Pod 共有不可
    

そのため今回の要件には適さない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure NetApp Files]]
## Azure NetApp Files

Azure NetApp Files (ANF) は非常に高性能なファイルストレージである。

特徴

- NFS / SMB
    
- 超低レイテンシ
    
- 高スループット
    
- エンタープライズ向け
    

しかし

- コストが高い
    
- 構成が複雑
    

一般的な AKS ワークロードでは **Azure Files の方がシンプル**である。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
## Azure Blob Storage

Azure Blob Storage は **オブジェクトストレージ**である。

特徴

- REST API
    
- 大容量ストレージ
    
- 低コスト
    

しかし次の問題がある。

- POSIXファイルシステムではない
    
- ファイルロックなし
    
- ディレクトリ操作が制限
    

そのため

**共有ファイルシステム用途には適さない。**

---

# 9 推奨アーキテクチャ

AKS ステートフルアプリケーションの構成

```text
AKS Cluster
   │
   ├ Pod 1
   ├ Pod 2
   ├ Pod 3
   │
   ▼
Shared Volume (RWX)
   │
   ▼
Azure Files
   │
   ▼
Azure Storage Account
```

この構成により

- 複数 Pod 同時アクセス
    
- 永続データ保存
    
- 自動ストレージ作成
    
- 高スループット
    

を実現できる。

---

# 10 まとめ

今回の要件

- ステートフルアプリケーション
    
- 複数 Pod 共有ストレージ
    
- 動的プロビジョニング
    
- 高スループット
    

これらを満たす Azure ストレージは

**Azure Files（ReadWriteMany）**

である。

理由

1. 複数 Pod 同時アクセス (RWX)
    
2. Kubernetes CSI ドライバー対応
    
3. 動的プロビジョニング
    
4. 高スループット対応（Premium SKU）
    

そのため AKS で共有永続ストレージを実装する場合、最もシンプルで一般的なソリューションは **Azure Files RWX** となる。