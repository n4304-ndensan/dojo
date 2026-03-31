[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Managed Instance]]
# Azure SQL Managed Instance 接続設計ドキュメント

（オンプレミスネットワークからの安全なアクセス）

---

# 1 背景

企業ではデータベースをクラウドへ移行するケースが増えており、Azure SQL Managed Instance（SQL MI）はその代表的な PaaS データベースサービスの一つである。SQL Managed Instance は SQL Server と高い互換性を持ちながら、インフラ管理を Azure 側に任せることができるため、既存のオンプレミス SQL Server 環境からの移行先として広く利用されている。

今回のシナリオでは、Azure SQL Managed Instance を Azure 上に配置するが、データベースは社内システムから利用されるため、オンプレミスネットワークからアクセスする必要がある。一方でセキュリティ要件として、データベースをパブリックインターネットに公開することは禁止されている。そのため、Azure 上のデータベースへアクセスする際にも、インターネットに公開されたパブリックエンドポイントを利用することなく、安全な方法で接続する必要がある。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
このような要件では、Azure のネットワーク機能を利用して、オンプレミスネットワークと Azure 仮想ネットワーク（VNet）を安全に接続し、データベースへプライベート IP アドレスでアクセスする構成を設計する必要がある。

---

# 2 要件

今回の設計では、以下の要件を満たす必要がある。

- Azure SQL Managed Instance にオンプレミスネットワークからアクセスできること
    
- Azure SQL Managed Instance がパブリックインターネットに公開されないこと
    
- データベースはプライベート IP でアクセスできること
    
- 安全なネットワーク通信を維持できること
    

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Link]]
これらの条件を満たす Azure の機能として適切なのは **Private Endpoint（Azure Private Link）** である。

---

# 3 Azure Private Endpoint の概要

Private Endpoint は Azure Private Link の機能の一つであり、Azure の PaaS サービスを仮想ネットワーク内のプライベート IP アドレスとして公開する仕組みである。通常、Azure の多くの PaaS サービスはパブリックエンドポイントを通じてインターネットからアクセス可能だが、Private Endpoint を使用すると、そのサービスをインターネットから完全に隔離し、仮想ネットワーク内からのみアクセス可能にすることができる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
この仕組みにより、Azure SQL Managed Instance や Azure Storage、Azure Key Vault などのサービスをパブリックインターネットに公開することなく利用することができる。企業のセキュリティポリシーにおいて、データベースをインターネットに公開しないという要件がある場合、Private Endpoint は非常に重要な役割を果たす。

Private Endpoint を作成すると、Azure 仮想ネットワーク内のサブネットにネットワークインターフェースが作成され、そのインターフェースにプライベート IP アドレスが割り当てられる。この IP アドレスが Azure SQL Managed Instance への接続ポイントとなり、仮想ネットワーク内のリソースやオンプレミスネットワークから安全にアクセスできるようになる。

---

# 4 接続アーキテクチャ

Private Endpoint を利用した場合の基本的な接続構成は次のようになる。

```
On-premises Network
        │
        │ (VPN または ExpressRoute)
        │
        ▼
Azure Virtual Network
        │
        ├─ Application Servers
        │
        └─ Private Endpoint
                │
                ▼
        Azure SQL Managed Instance
```

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure ExpressRoute]]
オンプレミスネットワークは VPN Gateway や ExpressRoute を利用して Azure 仮想ネットワークへ接続される。仮想ネットワーク内では Private Endpoint が Azure SQL Managed Instance に紐づけられており、オンプレミスネットワークのサーバーやアプリケーションは、この Private Endpoint のプライベート IP アドレスを通じてデータベースへアクセスする。

この構成では Azure SQL Managed Instance はインターネットに公開されないため、外部から直接アクセスすることはできない。

---

# 5 通信の流れ

実際の通信は次のような流れで行われる。

1. オンプレミスネットワーク上のアプリケーションが SQL Server へ接続要求を送信する
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
2. DNS により SQL Managed Instance の FQDN が Private Endpoint のプライベート IP に解決される
    
3. 接続要求は VPN または ExpressRoute を通じて Azure 仮想ネットワークへ送信される
    
4. 仮想ネットワーク内の Private Endpoint が接続を受け取り、Azure SQL Managed Instance に転送する
    
5. データベースからの応答が同じ経路でオンプレミスへ返される
    

この通信はすべてプライベートネットワーク内で行われるため、パブリックインターネットに露出することはない。

---

# 6 DNS の役割

Private Endpoint を使用する場合、DNS の設定が重要になる。通常、Azure SQL Managed Instance のドメイン名はパブリック IP に解決されるが、Private Endpoint を使用すると DNS を構成することで、そのドメイン名が仮想ネットワーク内のプライベート IP に解決されるようになる。

これにより、アプリケーション側では接続先の URL を変更することなく、同じ FQDN を使用して安全なプライベート接続を実現できる。

---

# 7 他の選択肢が適切でない理由

パブリックエンドポイントとファイアウォールルールを利用する方法では、接続元 IP を制限することはできるが、データベース自体はインターネット上に公開されることになる。そのため、セキュリティ要件である「パブリックインターネットに公開しない」という条件を満たすことができない。

Point-to-Site VPN は個々のクライアント PC が Azure に接続するための方式であり、オンプレミスネットワーク全体を Azure に接続する用途には適していない。

ExpressRoute はオンプレミスと Azure を専用回線で接続するサービスであり、高速で安定した通信を提供するが、これは接続方式であり、SQL Managed Instance を非公開にする仕組みではない。ExpressRoute を使用する場合でも、Private Endpoint のようなプライベート接続の仕組みが必要になる。

Azure Relay はファイアウォールの背後にあるアプリケーションへ安全にアクセスするためのサービスであり、データベース接続用途としては設計されていない。

---

# 8 推奨構成

最も適切な構成は次の通りである。

- Azure SQL Managed Instance を VNet 内に配置
    
- Private Endpoint を作成して SQL MI に接続
    
- オンプレミスネットワークから Azure VNet へ VPN または ExpressRoute で接続
    
- DNS を構成して SQL MI のドメイン名を Private Endpoint の IP に解決
    

この構成により、オンプレミスから Azure SQL Managed Instance へ安全なプライベート接続が可能になる。

---

# 9 まとめ

今回のシナリオでは、Azure SQL Managed Instance にオンプレミスネットワークからアクセスする必要があるが、パブリックインターネットに公開することは許されない。

この要件を満たす最適なソリューションは **Azure Private Endpoint（Azure Private Link）** である。

Private Endpoint を使用することで、Azure SQL Managed Instance は仮想ネットワーク内のプライベート IP アドレスとして公開され、オンプレミスネットワークから VPN または ExpressRoute を通じて安全にアクセスできるようになる。また、インターネットに公開されないため、セキュリティリスクを大幅に低減することができる。