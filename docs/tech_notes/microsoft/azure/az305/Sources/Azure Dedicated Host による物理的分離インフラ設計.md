---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/virtual-machines
  - cloud/azure/dedicated-host
  - cloud/architecture/isolation
  - cloud/architecture/single-tenant
  - security/infrastructure-isolation
  - security/compliance
  - exam/azure
---
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Dedicated Host]]
# Azure Dedicated Host による物理的分離インフラ設計

## 1. 背景（シナリオ）

ある企業が、**機密性の高い財務データを処理する新しいアプリケーション**を Azure 上に導入しようとしています。  
このアプリケーションは厳格なセキュリティ要件を満たす必要があります。

特にセキュリティチームから次の要件が提示されています。

- アプリケーションのインフラは **他の顧客ワークロードと物理的に隔離されている必要がある**
- 企業が **基盤となるホストサーバーへ独占的にアクセスできること**
- 共有ホスト（マルチテナント）は許可されない

これは通常のクラウドマルチテナント環境ではなく、**シングルテナントの物理ホスト**を要求する設計になります。

---

## 2. 要件整理

問題文から重要な要件を整理します。

まずセキュリティ要件があります。

このアプリケーションは財務データを扱うため、次の条件が必須です。

- **物理的隔離（Physical Isolation）**
- **ホストの専有（Exclusive Host Access）**

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
つまり、同じ物理サーバー上で他の顧客の VM が実行されることは許可されません。

次に運用要件です。

- Azure の管理機能は利用したい
- VM ベースのアプリケーションを実行

この条件から **Azure IaaS の中でも専有ハードウェアを提供するサービス**が必要になります。

---

## 3. 技術の基本概念

### Azure Dedicated Host

**Azure Dedicated Host** は、Azure が提供する **物理サーバーを単一顧客専用として割り当てるサービス**です。

通常の Azure VM は以下のような構造です。

```

Physical Host  
├ VM (Customer A)  
├ VM (Customer B)  
└ VM (Customer C)

```

これは **マルチテナント環境**です。

一方 Dedicated Host では次のようになります。

```

Dedicated Host  
├ VM (Your Organization)  
├ VM (Your Organization)  
└ VM (Your Organization)

```

つまり **同一顧客のみがホストを使用**します。

---

### Dedicated Host の特徴

Dedicated Host には次の特徴があります。

- **物理サーバーを単一サブスクリプションへ割り当て**
- 他の顧客と **ハードウェア共有なし**
- VM を特定ホストに配置可能
- コンプライアンス要件対応
- ライセンス持ち込み（BYOL）対応

そのため以下のようなケースで使用されます。

- 金融システム
- 規制産業（金融・政府）
- 厳格なコンプライアンス要件
- 物理分離が必要な環境

---

## 4. アーキテクチャまたは設計のポイント

今回の設計では、次の構成が推奨されます。

**Dedicated Host + Azure Virtual Machines**

構成は次のようになります。

```

Azure Dedicated Host  
├ Virtual Machine (App Server)  
├ Virtual Machine (Processing Server)  
└ Virtual Machine (Security Components)

```

この構成のポイントは以下です。

まず、Dedicated Host により **物理ハードウェアが企業専用**になります。

次に、VM はそのホスト上に配置されるため、**他の顧客のワークロードが混在することはありません**。

さらに、Azure VM の機能も利用できます。

- スケール
- 管理ツール
- セキュリティ機能
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
- Azure Backup / Monitor

---

## 5. 設計判断（なぜこの構成になるか）

この設計が最適な理由は、**物理分離要件を満たす唯一の Azure IaaS ソリューションだから**です。

通常の Azure VM はマルチテナント環境で実行されます。

つまり

- 他の顧客の VM
- 同じ物理サーバー

で実行される可能性があります。

しかし Dedicated Host では

- 物理サーバーが専用
- VM 配置を制御可能
- ホストレベルの可視性

が提供されます。

そのため、**セキュリティ要件とコンプライアンス要件を満たすことができます**。

---

## 6. 他の選択肢が誤りな理由

### A. 可用性セットで Azure VM を展開

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Set]]
Availability Set は **高可用性のための仕組み**です。

目的は以下です。

- 障害ドメイン分散
- 更新ドメイン分散

しかしこれは **物理分離を保証するものではありません**。

VM は依然として **共有ホスト上で実行される可能性があります**。

---

### C. App Service Environment (ASE)

ASE は **ネットワーク分離された App Service 環境**です。

しかし ASE は次の特徴があります。

- プラットフォーム管理
- PaaS サービス
- マルチテナント基盤

つまり **物理ホスト専有ではありません**。

---

### D. 別サブスクリプションで VM を展開

Azure サブスクリプションは **論理的な管理境界**です。

しかし物理ホストは **Azure 全体で共有**されます。

そのため次のことが起こり得ます。

```

Host  
├ VM (Your subscription)  
└ VM (Other customer)

```

つまり **物理分離は保証されません**。

---

## 7. 最終回答

**B. Azure Dedicated Host を使用して Azure Virtual Machines をデプロイする**

---

## 8. まとめ

この問題の重要なポイントは **物理的分離（Physical Isolation）**です。

Azure の多くのサービスはマルチテナントですが、Dedicated Host は例外です。

Dedicated Host を使用すると次が実現できます。

- 物理サーバーの専有
- 他顧客とのハードウェア共有なし
- コンプライアンス対応
- VM 配置の制御

そのため、**金融データなどの高セキュリティワークロード**では Dedicated Host が推奨されます。
