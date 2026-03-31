[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
# Azure App Service 高可用性設計

（リージョン障害対応 + 自動トラフィックフェイルオーバー）

---

# 1 背景

ある組織では **Azure App Service** を使用してミッションクリティカルな Web アプリケーションを運用している。ミッションクリティカルなシステムでは、サービス停止がビジネスに重大な影響を与えるため、障害発生時でもサービスを継続できる高可用性アーキテクチャが求められる。

今回のシステムには次の要件がある。

- Azure リージョン全体の障害（Regional Outage）に耐えること
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#RTO]]
- 15分以内の **RTO（Recovery Time Objective）** を満たすこと
    
- 障害発生時に **トラフィックを自動的に健全なリージョンへリダイレクト**すること
    

RTO とは、システム障害が発生した場合にサービスを復旧させるまでの許容時間を意味する。今回の要件では、リージョン障害が発生しても **15分以内にサービスが復旧する構成**が必要である。

これらの要件を満たす最適な Azure 構成は

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
**Azure Front Door + 複数リージョンの Azure App Service**

である。

---

# 2 Azure Front Door

Azure Front Door は Microsoft が提供する **グローバルレベルのレイヤー7ロードバランサ**である。世界中の Azure エッジネットワークを利用して、ユーザーから最適なバックエンドへトラフィックをルーティングすることができる。

Front Door は次の機能を提供する。

|機能|説明|
|---|---|
|グローバルロードバランシング|最も近いリージョンへルーティング|
|ヘルスチェック|バックエンドの状態監視|
|自動フェイルオーバー|障害時の自動切り替え|
|高速レスポンス|エッジネットワーク利用|

Front Door は HTTP / HTTPS レベルで動作するため、Web アプリケーションの可用性を向上させる目的で広く利用されている。

---

# 3 マルチリージョン App Service 構成

リージョン障害に対応するためには、単一リージョンではなく **複数リージョンにアプリケーションをデプロイ**する必要がある。

典型的な構成は次のようになる。

```text
Users
   │
   ▼
Azure Front Door
   │
   ├─ App Service (Region A)
   │
   └─ App Service (Region B)
```

通常時は最も近いリージョン、または優先リージョンにトラフィックが送信される。もしリージョン A が障害を起こした場合、Front Door が自動的にリージョン B にトラフィックを転送する。

---

# 4 フェイルオーバーの仕組み

Azure Front Door はバックエンドのヘルスチェックを定期的に実行する。

```text
Front Door Health Probe
        │
        ▼
Region A  (Healthy)
Region B  (Healthy)
```

もし Region A が停止すると次のように動作する。

```text
Front Door Health Probe
        │
        ▼
Region A  (Unhealthy)
Region B  (Healthy)

Traffic → Region B
```

この自動フェイルオーバーにより、ユーザーはサービス停止をほとんど意識することなくアプリケーションを利用できる。

---

# 5 RTO 15分を満たす理由

Front Door は数十秒〜数分程度でバックエンド障害を検知し、トラフィックを別リージョンへ切り替えることができる。そのため、リージョン障害が発生しても **RTO 15分以内の復旧要件を十分に満たす**。

さらに、アプリケーションがあらかじめ複数リージョンにデプロイされているため、新しいインフラを起動する必要がなく、迅速なフェイルオーバーが可能になる。

---

# 6 システムアーキテクチャ

この構成の全体像は次のようになる。

```text
Global Users
      │
      ▼
Azure Front Door
      │
      ▼
 ┌─────────────┐
 │ Region A    │
 │ App Service │
 └─────────────┘
      │
      │ Failover
      ▼
 ┌─────────────┐
 │ Region B    │
 │ App Service │
 └─────────────┘
```

この構成では、Front Door がグローバルエントリポイントとして機能し、バックエンドの App Service インスタンスを監視する。

---

# 7 他の選択肢が適切でない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Availability Zone]]
App Service の空き領域（Availability Zone）を利用する方法は、単一リージョン内の障害には対応できるが、リージョン全体の障害には対応できない。そのため、地域的な停電への耐性を提供することはできない。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
Traffic Manager は DNS ベースのロードバランサであり、フェイルオーバーは可能だが、DNS キャッシュの影響を受けるため切り替え時間が長くなる場合がある。RTO が厳しいミッションクリティカルシステムでは Front Door の方が適している。

App Service の自動スケーリングは負荷対策の機能であり、リージョン障害の対策ではない。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Backup]]
Azure Backup や geo-restore はデータ復旧のための仕組みであり、リアルタイムフェイルオーバーを提供するものではない。

---

# 8 メリット

この構成にはいくつかの重要なメリットがある。

まず、マルチリージョン構成によりリージョン障害に対する高い可用性を実現できる。また、Azure Front Door を利用することでユーザーは最も近いリージョンへ接続できるため、パフォーマンスも向上する。

さらに、Front Door は WAF（Web Application Firewall）や SSL 終端などの機能も提供するため、セキュリティとパフォーマンスの両方を向上させることができる。

---

# 9 まとめ

今回の要件は次の通りである。

- ミッションクリティカルな Web アプリケーション
    
- 15分以内の RTO
    
- リージョン障害への対応
    
- 自動トラフィックリダイレクト
    

これらの要件を満たす最適な構成は

**Azure Front Door + マルチリージョン App Service**

である。

このアーキテクチャにより、リージョン障害が発生した場合でも自動的にトラフィックが健全なリージョンにルーティングされ、高可用性と高速な復旧を実現できる。