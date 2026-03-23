# Scenario: Virtual Network

## シナリオ一覧

- Hub-and-Spoke で共有ネットワークを集約する
- Global VNet Peering でリージョン間接続を作る
- Application Gateway で URL ベースルーティングを行う
- Front Door でマルチリージョン Web をフェイルオーバーする
- ExpressRoute と VPN を併用して可用性を上げる
- Virtual WAN で多拠点接続を集約する
- NSG と Firewall で VM の入口を絞る
- Network Watcher で疎通障害を切り分ける

## hub-and-spoke-で共有ネットワークを集約する

シナリオ  
共有ファイアウォール、DNS、接続ハブを 1 つに寄せ、業務 VNet をスポークに分ける。

構成  
Hub VNet  
↓  
Peering  
↓  
Spoke VNets

ポイント  
- 共通サービスの集約に向く
- 部門ごとの分離と中央統制を両立しやすい

関連リソース  
Azure Virtual Network / Peering / Azure Firewall

出典  
- [[Sources/Azure ハブ＆スポークネットワークにおける適切なピアリング構成.md]]

## global-vnet-peering-でリージョン間接続を作る

シナリオ  
異なるリージョンの VNet を Azure バックボーンで低遅延接続する。

構成  
VNet A  
↔  
Global VNet Peering  
↔  
VNet B

ポイント  
- Azure 内通信で高性能
- VPN 装置管理が不要

関連リソース  
Azure Virtual Network / Global VNet Peering

出典  
- [[Sources/Azure VNet 間通信（グローバル VNet ピアリング）.md]]
- [[Sources/異なるリージョン間の安全で低遅延なVNet通信.md]]

## application-gateway-で-url-ベースルーティングを行う

シナリオ  
1 つの入口から URL パスで複数バックエンドへ振り分ける。

構成  
Client  
↓  
Application Gateway  
↓  
Web / API / AKS Service

ポイント  
- HTTPS 終端と WAF を同時に扱える
- AKS Ingress とも相性がよい

関連リソース  
Azure Application Gateway / WAF / AKS

出典  
- [[Sources/Azure Application Gateway の URL ベースルーティング.md]]
- [[Sources/AKS Ingress Controller 設計ドキュメント.md]]

## front-door-でマルチリージョンwebをフェイルオーバーする

シナリオ  
複数リージョンへ配置した Web アプリをグローバルに最適配信する。

構成  
User  
↓  
Azure Front Door  
↓  
Region A / Region B

ポイント  
- グローバル入口を 1 つにできる
- WAF とヘルスプローブを同時に使える
- DNS レベルより細かい HTTP 制御がしやすい

関連リソース  
Azure Front Door / App Service / Traffic Manager

出典  
- [[Sources/Azure Front Door によるグローバルアプリケーション配信.md]]
- [[Sources/マルチリージョン Web アプリケーションのグローバル負荷分散（Azure Front Door）.md]]

## expressroute-と-vpn-を併用して可用性を上げる

シナリオ  
基幹系は ExpressRoute、本線障害時は VPN でバックアップ接続する。

構成  
On-prem  
↓  
ExpressRoute + VPN Gateway  
↓  
Azure VNet

ポイント  
- 高品質回線と冗長経路を両立できる
- BGP 設計が重要

関連リソース  
ExpressRoute / VPN Gateway / Azure Virtual Network

出典  
- [[Sources/ExpressRoute と VPN を組み合わせたハイブリッドネットワーク接続設計.md]]
- [[Sources/Azure ExpressRoute と VPN Gateway を使用したハイブリッド接続アーキテクチャ.md]]

## virtual-wan-で多拠点接続を集約する

シナリオ  
多拠点と複数 VNet の接続を Secure Virtual Hub に集約する。

構成  
Branches / VNets  
↓  
Virtual WAN Hub  
↓  
Shared Connectivity

ポイント  
- 拠点数が多いほど運用が軽くなる
- ルーティングの集中管理に向く

関連リソース  
Azure Virtual WAN / ExpressRoute / VPN

出典  
- [[Sources/Azure Virtual WAN 設計.md]]
- [[Sources/Azure Virtual WAN と Hub-and-Spoke トポロジによるグローバル接続アーキテクチャ.md]]

## nsg-と-firewall-で-vm-の入口を絞る

シナリオ  
VM の公開ポートを絞り、必要な通信だけを許可する。

構成  
Internet  
↓  
Azure Firewall  
↓  
Subnet + NSG  
↓  
VM

ポイント  
- NSG は局所制御、Firewall は中央制御
- 運用ルールと脅威対策を分けて考えやすい

関連リソース  
NSG / Azure Firewall / Azure Virtual Machines

出典  
- [[Sources/Azure 仮想マシン向けネットワークセキュリティ設計.md]]
- [[Sources/Azure Firewall によるネットワークセキュリティ設計.md]]

## network-watcher-で疎通障害を切り分ける

シナリオ  
通信不可の原因をルール・経路・フローの観点で確認する。

構成  
VM  
↓  
Network Watcher  
↓  
IP Flow Verify / NSG Flow Logs

ポイント  
- NSG かルーティングかの切り分けが速い
- 監視とトラブルシュートを分けて考えられる

関連リソース  
Network Watcher / NSG / Log Analytics

出典  
- [[Sources/Azure Network Watcher IP Flow Verify によるネットワークトラブルシューティング.md]]
- [[Sources/NSG フローログによるネットワークトラフィック分析.md]]
