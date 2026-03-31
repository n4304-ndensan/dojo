[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Logic Apps]]
# Azure Logic Apps からオンプレミス SQL Server へ接続するアーキテクチャ

（インターネット接続のない環境でのデータ統合）

---

# 1 背景

企業では業務プロセスの自動化のために **Azure Logic Apps** を利用している。Logic Apps は SaaS 連携やデータ処理のワークフローをクラウド上で自動化するサービスであり、さまざまなシステムと接続することができる。

今回のシナリオでは、Logic Apps から **オンプレミス環境に存在する SQL Server データベースへデータを書き込む必要**がある。しかし、この SQL Server はセキュリティポリシーの制約により **インターネットから直接アクセスできないネットワーク環境**に配置されている。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
また、このサブスクリプションは **ハイブリッド Azure AD テナント**に関連付けられており、オンプレミスとクラウドのアイデンティティ基盤が統合されている。

この状況で次の要件を満たす必要がある。

- Azure Logic Apps からオンプレミス SQL Server へアクセスする
    
- SQL Server はインターネットから直接公開されていない
    
- セキュアな通信を維持する
    
- Azure とオンプレミス間の接続を確立する
    

---

# 2 問題の本質

この問題のポイントは次の条件である。

```text
Logic Apps（クラウド）
↓
オンプレミス SQL Server
↓
インターネット接続なし
```

通常、クラウドサービスからオンプレミスのデータベースへアクセスする場合は、次のような方法が考えられる。

- VPN 接続
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
- ExpressRoute
    
- Azure Relay
    
- オンプレミスデータゲートウェイ
    

今回の問題では **Logic Apps からデータベースへの接続**であるため、Microsoft が推奨する方法は **On-premises Data Gateway（オンプレミスデータゲートウェイ）**となる。

---

# 3 On-premises Data Gateway

オンプレミスデータゲートウェイは、Azure サービスとオンプレミスのデータソースを安全に接続するためのコンポーネントである。

このゲートウェイをオンプレミス環境にインストールすることで、クラウドサービスからオンプレミスのデータベースへ安全にアクセスできるようになる。

重要な特徴は次の通りである。

- SQL Server をインターネットへ公開する必要がない
    
- Azure への **アウトバウンド通信のみ**を使用
    
- 暗号化された通信
    
- Azure サービスとの統合
    

このゲートウェイは主に次の Azure サービスと連携する。

- Logic Apps
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Power BI]]
- Power BI
    
- Power Automate
    
- Azure Analysis Services
    

---

# 4 アーキテクチャ

オンプレミスデータゲートウェイを使用した接続構成は次のようになる。

```text
Azure Logic Apps
        │
        │
        ▼
Azure 接続ゲートウェイ
        │
        │
        ▼
オンプレミスデータゲートウェイ
        │
        │
        ▼
SQL Server
```

この構成では、Azure 側からオンプレミスへ直接接続するのではなく、**ゲートウェイを経由して通信が行われる**。

---

# 5 通信の仕組み

通信は次のような流れで行われる。

1. Logic Apps が SQL Server へ書き込み要求を送信
    
2. Azure 側の接続ゲートウェイがリクエストを受信
    
3. オンプレミスデータゲートウェイへ転送
    
4. ゲートウェイが SQL Server へ接続
    
5. SQL クエリを実行
    

重要なポイントは

```text
SQL Server はインターネット公開されない
```

という点である。

ゲートウェイが Azure へ **アウトバウンド接続**を確立することで通信が成立する。

---

# 6 セキュリティ

オンプレミスデータゲートウェイでは以下のセキュリティ機能が提供される。

### TLS 暗号化

すべての通信は TLS によって暗号化される。

---

### 認証

Azure AD による認証が行われる。

---

### ポート制限

SQL Server を外部に公開する必要がないため、ファイアウォールポリシーを維持できる。

---

# 7 他の選択肢が不適切な理由

## Azure AD Application Proxy

Application Proxy は **オンプレミス Web アプリケーション公開**のためのサービスである。

用途

- 社内 Web アプリ
    
- リモートアクセス
    

しかし SQL Server への直接データアクセスには使用されない。

---

## Application Gateway

Application Gateway は Azure 内の Web トラフィックを負荷分散するサービスであり、オンプレミス SQL Server 接続には利用されない。

---

## Hybrid Connection Manager

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
Hybrid Connections は Azure App Service からオンプレミスリソースへ接続する仕組みである。

ただし

- Azure Relay が必要
    
- インターネット接続が必要
    

今回の条件には適合しない。

---

# 8 最適構成

今回の要件を満たす構成は次の通りである。

**オンプレミス**

```text
オンプレミスデータゲートウェイ
```

**Azure**

```text
接続ゲートウェイ
```

---

# 9 最終回答

正解

**C**

```text
オンプレミス: オンプレミスデータゲートウェイ
Azure: 接続ゲートウェイ
```

---

# 10 まとめ

Azure Logic Apps からオンプレミスデータベースへアクセスする場合、最も一般的で安全な方法は **オンプレミスデータゲートウェイを利用すること**である。

この方法により

- SQL Server をインターネットに公開しない
    
- Azure とオンプレミス間の安全な通信
    
- Azure AD と統合した認証
    

を実現することができる。