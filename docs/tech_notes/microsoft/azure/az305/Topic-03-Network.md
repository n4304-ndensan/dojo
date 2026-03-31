# Topic-03-Network

## 学習ゴール

[[Azure用語集.md#Azure Virtual Network]] を中心に、入口、閉域接続、ハイブリッド接続、ネットワーク保護を 1 本の地図として理解する。

## このTopicの全体像

この Topic では、Virtual Network、Application Gateway、Front Door、ExpressRoute、VPN、Private Endpoint、NSG、Firewall、Network Watcher を扱う。  
判断軸は「どこで受けるか」「どこまで私設化するか」「誰をどこで止めるか」。

---

# 第1章 学習マップ

## 1.1 学習順序

1. 入口サービスとして Application Gateway、Front Door、Traffic Manager、Load Balancer を比較する。
2. Virtual Network と Peering をベースに到達性を作る。
3. ExpressRoute、VPN Gateway、Virtual WAN でハイブリッド接続を整理する。
4. Private Endpoint と DNS で PaaS を閉域化する。
5. NSG、Firewall、Network Watcher で保護と診断を固める。

## 1.2 Azureリソース一覧

- Azure Virtual Network / Subnet / Peering
- Azure Application Gateway
- Azure Front Door / Traffic Manager / Load Balancer
- ExpressRoute / VPN Gateway / Virtual WAN
- Azure Private Endpoint / Private DNS
- Azure Firewall / NSG / Network Watcher

---

# 第2章 Azureリソース解説

## Resource: Azure Virtual Network

### 概要

[[Azure用語集.md#Azure Virtual Network]] は Azure ワークロードの通信境界であり、すべてのプライベート接続設計の土台になる。

### できること

- サブネット分割
- VNet Peering
- ハブアンドスポーク
- VPN / ExpressRoute の受け口
- NSG や UDR の適用

### 技術仕様

- サブネット単位で NSG、ルート、Private Endpoint を設計する。
- グローバル VNet Peering でリージョン間接続を作れる。
- Hub-and-Spoke は共有ネットワーク サービスの集約に向く。

### SDK / API

- ARM / Azure Network API
- Azure CLI `az network vnet`

### 他サービスとの比較

- VNet Peering vs VPN: Azure 内高速接続なら Peering、暗号化トンネルやオンプレミスなら VPN。
- Virtual WAN vs 個別接続: 拠点数が増えるほど Virtual WAN が運用しやすい。

### どのようなときに使うか

- ワークロードをプライベート IP 空間で隔離したいとき
- ハイブリッド接続の受け口を作りたいとき
- PaaS への Private Endpoint を収容したいとき

### 関連シナリオ

- [[Scenarios/Scenario-VirtualNetwork.md#hub-and-spoke-で共有ネットワークを集約する]]
- [[Scenarios/Scenario-VirtualNetwork.md#global-vnet-peering-でリージョン間接続を作る]]

### 主な出典

- [[Sources/Topic-03.md]]
- [[Sources/Azure VNet 間通信（グローバル VNet ピアリング）.md]]
- [[Sources/Azure ハブ＆スポークネットワークにおける適切なピアリング構成.md]]
- [[Sources/Azure仮想ネットワークにおけるサブネット設計とVPNゲートウェイサブネット.md]]

## Resource: Azure Application Gateway and Edge Routing

### 概要

[[Azure用語集.md#Azure Application Gateway]] と [[Azure用語集.md#Azure Front Door]] は「どこで HTTP を受けるか」を決める主要サービス。

### できること

- HTTPS 終端
- URL ベース ルーティング
- WAF
- グローバル フェイルオーバー
- AKS Ingress 連携

### 技術仕様

- Application Gateway はリージョン内 L7 入口。
- Front Door はグローバル エッジ入口。
- Traffic Manager は DNS ベースのリージョン選択。
- Load Balancer は L4 配信に向く。

### SDK / API

- Azure CLI `az network application-gateway`
- Front Door / Traffic Manager 管理 API

### 他サービスとの比較

- Front Door vs Traffic Manager: HTTP 最適化と WAF が必要なら Front Door。
- Application Gateway vs Load Balancer: L7 制御が必要なら Application Gateway。
- AGIC vs NGINX Ingress: Azure ネイティブ運用を重視するなら AGIC。

### どのようなときに使うか

- Web アプリをグローバル配信したいとき
- AKS の HTTP 入口を Azure ネイティブに寄せたいとき
- URL パスで複数バックエンドへ分岐したいとき

### 関連シナリオ

- [[Scenarios/Scenario-VirtualNetwork.md#application-gateway-で-url-ベースルーティングを行う]]
- [[Scenarios/Scenario-VirtualNetwork.md#front-door-でマルチリージョンwebをフェイルオーバーする]]

### 主な出典

- [[Sources/Topic-03.md]]
- [[Sources/Azure Application Gateway の URL ベースルーティング.md]]
- [[Sources/Azure Front Door によるグローバルWebアプリケーションの高可用性設計.md]]
- [[Sources/AKS Ingress Controller 設計ドキュメント.md]]

## Resource: Hybrid Connectivity

### 概要

ExpressRoute、VPN Gateway、Virtual WAN は Azure とオンプレミス、拠点間ネットワークをつなぐ接続基盤。

### できること

- オンプレミス接続
- 拠点間接続
- BGP ルーティング
- フェイルオーバー
- セキュア Virtual Hub 集約

### 技術仕様

- ExpressRoute は専用線、VPN はインターネット越し暗号化トンネル。
- Global Reach は ExpressRoute 間の拠点接続に使う。
- Virtual WAN は多拠点・多 VNet で運用簡素化が効く。

### SDK / API

- Azure Network API
- BGP / routing configuration

### 他サービスとの比較

- ExpressRoute vs VPN: 帯域・安定性・閉域性を優先するなら ExpressRoute。
- Virtual WAN vs 個別 Hub-and-Spoke: 拠点数と運用負荷で判断する。

### どのようなときに使うか

- 基幹系と Azure を安定接続したいとき
- 複数拠点の経路を集約したいとき
- オンプレミス出口経由の経路制御をしたいとき

### 関連シナリオ

- [[Scenarios/Scenario-VirtualNetwork.md#expressroute-と-vpn-を併用して可用性を上げる]]
- [[Scenarios/Scenario-VirtualNetwork.md#virtual-wan-で多拠点接続を集約する]]

### 主な出典

- [[Sources/Topic-03.md]]
- [[Sources/Azure ExpressRoute Global Reach を用いたマルチサイトネットワーク設計.md]]
- [[Sources/ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]
- [[Sources/Azure Virtual WAN 設計.md]]

## Resource: Azure Private Endpoint

### 概要

[[Azure用語集.md#Azure Private Endpoint]] は Azure PaaS へプライベート IP で接続するための標準手段。

### できること

- SQL Database、Storage、Key Vault の閉域化
- Web App から VNet リソースへの安全接続
- Private DNS と組み合わせた名前解決

### 技術仕様

- VNet 内に NIC が作られる。
- 既定の公開 FQDN を Private DNS で私設 IP へ引き直す。
- オンプレミス連携時は DNS 転送まで考える必要がある。

### SDK / API

- Azure Network API
- Private DNS zone management

### 他サービスとの比較

- Private Endpoint vs Service Endpoint: 特定 PaaS を私設 IP で閉じるなら Private Endpoint。
- Private Endpoint vs VNet Integration: App Service から VNet へ出る機能とは役割が違う。

### どのようなときに使うか

- PaaS への通信をインターネットに出したくないとき
- SQL Database や Storage を内部ネットワークだけへ見せたいとき

### 関連シナリオ

- [[Scenarios/Scenario-PrivateEndpoint.md#sql-database-を-private-endpoint-で公開する]]
- [[Scenarios/Scenario-PrivateEndpoint.md#storage-account-を-private-endpoint-で閉域化する]]
- [[Scenarios/Scenario-PrivateEndpoint.md#web-app-から-vnet-リソースへ安全に接続する]]

### 主な出典

- [[Sources/Topic-03.md]]
- [[Sources/Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
- [[Sources/Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Sources/Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]

## Resource: Network Security and Troubleshooting

### 概要

NSG、Azure Firewall、Network Watcher はネットワークの「遮断」と「見える化」を担当する。

### できること

- ポート制御
- 中央集約ファイアウォール
- パケット/フローログ分析
- IP Flow Verify による疎通確認

### 技術仕様

- NSG はサブネットや NIC 単位。
- Azure Firewall は集中出口制御や脅威インテリジェンスに向く。
- Network Watcher は診断系ツール群として切り分ける。

### SDK / API

- Azure CLI `az network nsg`
- Network Watcher API

### 他サービスとの比較

- NSG vs Firewall: 局所制御か、中央制御か。
- Firewall vs WAF: L3-L4-L7 の広い制御か、HTTP 特化保護か。

### どのようなときに使うか

- VM やサブネットへの入出力を限定したいとき
- 通信障害の原因を切り分けたいとき

### 関連シナリオ

- [[Scenarios/Scenario-VirtualNetwork.md#nsg-と-firewall-で-vm-の入口を絞る]]
- [[Scenarios/Scenario-VirtualNetwork.md#network-watcher-で疎通障害を切り分ける]]

### 主な出典

- [[Sources/Topic-03.md]]
- [[Sources/Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[Sources/Azure Network Watcher IP Flow Verify によるネットワークトラブルシューティング.md]]
- [[Sources/NSG フローログによるネットワークトラフィック分析.md]]

---

# 第3章 設計判断ガイド

## 3.1 入口を選ぶとき

- グローバル HTTP は Front Door。
- リージョン内 HTTP は Application Gateway。
- DNS レベル切替なら Traffic Manager。
- L4 分散なら Load Balancer。

## 3.2 閉域化するとき

- PaaS を私設化するなら Private Endpoint。
- 経路だけ Azure 内に寄せたい、私設 IP は不要なら Service Endpoint も検討する。
- Private DNS まで含めて完成形と考える。

## 3.3 ハイブリッド接続を選ぶとき

- 帯域と SLA を重視するなら ExpressRoute。
- 迅速導入やバックアップ経路なら VPN。
- 拠点と VNet が多いなら Virtual WAN。

---

# 第4章 関連シナリオ

- [[Scenarios/Scenario-VirtualNetwork.md]]
- [[Scenarios/Scenario-PrivateEndpoint.md]]

