# Scenario: Azure Private Endpoint

## シナリオ一覧

- SQL Database を Private Endpoint で公開する
- Storage Account を Private Endpoint で閉域化する
- Web App から VNet リソースへ安全に接続する

## sql-database-を-private-endpoint-で公開する

シナリオ  
SQL Database をプライベート IP で公開し、アプリ側からのみ到達させる。

構成  
App VNet  
↓  
Private Endpoint  
↓  
Azure SQL Database

ポイント  
- パブリック経由通信を避けられる
- Private DNS の整備が必須
- オンプレミス連携時は DNS 転送も要設計

関連リソース  
Azure Private Endpoint / Azure SQL Database / Private DNS

出典  
- [[Sources/Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計.md]]
- [[Sources/Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]

## storage-account-を-private-endpoint-で閉域化する

シナリオ  
Storage Account を内部ネットワークだけで利用できるようにする。

構成  
App VNet  
↓  
Private Endpoint  
↓  
Storage Account

ポイント  
- Storage Firewall と組み合わせて公開面を縮める
- Blob / Files / Queue のどれを閉じるかを整理する

関連リソース  
Azure Private Endpoint / Azure Storage Account

出典  
- [[Sources/Azure Storage アカウントのネットワーク制御（Private Endpoint と Storage Firewall）.md]]
- [[Sources/Azure Blob Storage の包括的データ保護（SSE + Private Link）.md]]

## web-app-から-vnet-リソースへ安全に接続する

シナリオ  
Web App から VNet 内リソースや Private Endpoint 配下の PaaS に安全に接続する。

構成  
Azure Web App  
↓  
VNet Integration  
↓  
Private Endpoint / Private Resource

ポイント  
- Web App 側の VNet Integration と対象側の Private Endpoint は役割が違う
- 名前解決まで通して初めて動作する

関連リソース  
Azure App Service / Azure Private Endpoint / Azure Virtual Network

出典  
- [[Sources/Azure Web App から VNet リソースへ安全にアクセスする方法（Private Endpoint）.md]]
- [[Sources/Azure PaaS サービスへのプライベート接続設計（Private Endpoint）.md]]
