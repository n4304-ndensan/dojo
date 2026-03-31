# SQL Server Always On データベース移行設計ドキュメント

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
（SQL Server 2017 → SQL Server 2019 on Azure Linux VM）

---

# 1 背景と目的

企業ではオンプレミス環境で **SQL Server 2017** を使用しており、可用性確保のために **Always On Availability Group (AG)** を利用した高可用性構成でデータベースを運用している。

今回、このデータベース環境をクラウドへ移行するため、**Azure 上の Linux 仮想マシンに構築された SQL Server 2019 インスタンスへ移行**する計画が立てられている。

移行において最も重要な要件は、業務システムの停止時間を可能な限り短くすることである。多くの企業システムではデータベースが基幹システムの中心に位置するため、長時間の停止は業務への重大な影響を引き起こす可能性がある。

そのため、この移行では以下の要件を満たす必要がある。

- データベース移行時の **ダウンタイムを最小限に抑えること**
    
- 移行後も **高可用性構成（Always On）を維持すること**
    
- SQL Server のバージョンを **2017 から 2019 にアップグレードすること**
    
- Azure Linux VM 上の SQL Server 環境へ移行すること
    

これらの要件を満たすためには、単純なバックアップとリストアではなく、**継続的にデータを同期しながら移行を進める方式**を採用する必要がある。

---

# 2 SQL Server Always On Availability Group

Always On Availability Group（AG）は、SQL Server における高可用性と災害復旧を実現するための機能である。

この機能では、複数の SQL Server インスタンス間でデータベースをレプリケーションすることで、プライマリサーバーが障害を起こした場合でも、セカンダリサーバーへフェイルオーバーすることが可能となる。

基本構成は次のようになる。

```text
Primary Replica
     │
     │ Transaction Log Replication
     ▼
Secondary Replica
```

プライマリレプリカで発生したトランザクションログがセカンダリに送信されることで、データの整合性を維持しながら高可用性を実現する。

Always On は主に次の目的で利用される。

- 高可用性 (High Availability)
    
- 災害対策 (Disaster Recovery)
    
- 読み取り負荷分散
    

今回の移行では、この Always On 環境を Azure へ移行する必要がある。

---

# 3 移行方式の検討

SQL Server のデータベース移行にはいくつかの方法が存在する。それぞれの方法にはメリットとデメリットがあり、要件に応じて適切な方法を選択する必要がある。

主な移行方式は以下の通りである。

|方式|特徴|ダウンタイム|
|---|---|---|
|Backup / Restore|最も単純|長い|
|Log Shipping|継続的同期可能|短い|
|Replication|特定用途向け|短い|
|Always On Replica|高可用性構成|短い|
|Distributed AG|AG間の同期|短い|

今回の要件では

- ダウンタイム最小
    
- 高可用性構成維持
    

という条件があるため、**継続的なデータ同期が可能な方式**が必要となる。

この条件を満たす方法として適しているのが **Log Shipping** である。

---

# 4 Log Shipping の仕組み

Log Shipping は、SQL Server のトランザクションログを定期的にバックアップし、別の SQL Server インスタンスへ適用することでデータ同期を行う仕組みである。

構成は次のようになる。

```text
Primary Server
   │
   │ Transaction Log Backup
   ▼
Log Copy Server
   │
   ▼
Secondary Server
   │
   │ Log Restore
   ▼
Standby Database
```

この方法では、トランザクションログが継続的にコピーされるため、セカンダリサーバーのデータベースは常にプライマリに近い状態を維持する。

そのため、最終切り替え時には **最後のログを適用するだけで移行を完了できる**。  
これにより、アプリケーション停止時間を最小限に抑えることができる。

---

# 5 移行の準備

移行を開始する前に、Azure 上の SQL Server 環境を準備する必要がある。

今回の構成では、Azure Linux VM 上に SQL Server 2019 インスタンスを構築する。さらに、移行後の高可用性構成を維持するために **新しい Availability Group を作成する**。

準備段階の構成は次のようになる。

```text
Azure VM
   │
   └ SQL Server 2019
        │
        └ New Availability Group
```

新しい AG を作成することで、移行後のデータベース高可用性環境を事前に整備することができる。

---

# 6 移行の実行

移行実行時には Log Shipping を使用してデータ同期を行う。

移行手順の概要は次の通りである。

1. オンプレミス SQL Server 2017 でログバックアップを取得する
    
2. ログバックアップを Azure VM へコピーする
    
3. Azure SQL Server 2019 でログをリストアする
    
4. 定期的にログを適用して同期状態を維持する
    
5. 移行タイミングで最後のログを適用する
    
6. アプリケーション接続先を Azure SQL Server に切り替える
    

この方法では、移行中もデータベースが継続的に同期されるため、最終切り替え時のダウンタイムは非常に短くなる。

---

# 7 他の選択肢が適切でない理由

## セカンダリレプリカ追加

既存の Always On AG に Azure VM を直接追加する方法は、ネットワーク構成やドメイン構成の違いによって複雑になる場合がある。

---

## Distributed AG

Distributed Availability Group は、複数の AG を接続する高度な構成であり、大規模な災害対策環境で利用されることが多い。今回の単純な移行シナリオでは過剰な構成となる。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Migrate]]
## Azure Migrate

Azure Migrate は主に仮想マシン移行や環境評価を目的としたツールであり、SQL Server のトランザクションレベル同期には適していない。

---

# 8 最終構成

今回の要件を満たす最適な構成は次の通りである。

**準備**

```text
VM1 で新しい Availability Group を作成
```

**移行**

```text
Log Shipping を使用してデータ同期
```

---

# 9 まとめ

SQL Server Always On 環境を Azure へ移行する場合、最も重要なポイントは **ダウンタイムを最小限に抑える移行方式を選択すること**である。

今回のケースでは

- SQL Server 2017 → SQL Server 2019
    
- Always On 構成維持
    
- Azure Linux VM への移行
    
- ダウンタイム最小
    

という要件があるため、**Log Shipping を使用した段階的移行方式**が最も適している。

最終的な回答は次の構成である。

```text
準備: VM1 で新しい AG を作成
移行: Log Shipping を使用
```