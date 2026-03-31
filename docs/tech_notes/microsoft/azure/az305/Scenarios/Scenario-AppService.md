# Scenario: Azure App Service

## シナリオ一覧

- Entra ID で Web アプリ認証を統一する
- Deployment Slots でゼロダウンタイム展開を行う
- App Service から Key Vault の秘密を参照する
- 高可用な Web 実行基盤として App Service と VM を比較する

## entra-id-で-web-アプリ認証を統一する

シナリオ  
社内向け Web アプリの認証を Entra ID に寄せ、アプリ実装の認証ロジックを減らす。

構成  
User  
↓  
Microsoft Entra ID  
↓  
Azure App Service

ポイント  
- Easy Auth と組み合わせやすい
- Conditional Access や MFA を統一適用できる

関連リソース  
Azure App Service / Microsoft Entra ID

出典  
- [[Sources/Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]
- [[Sources/Azure App Service 認証セキュリティ設計.md]]

## deployment-slots-でゼロダウンタイム展開を行う

シナリオ  
稼働中アプリを止めずに新バージョンを差し替える。

構成  
Production Slot  
↔  
Staging Slot  
↓  
Swap

ポイント  
- スワップ前に検証できる
- リリース失敗時の切り戻しが速い

関連リソース  
Azure App Service / Deployment Slots

出典  
- [[Sources/Azure App Service のゼロダウンタイムデプロイ（Deployment Slots）.md]]

## app-service-から-key-vault-の秘密を参照する

シナリオ  
App Service の設定から秘密情報を排除し、Key Vault 参照へ置き換える。

構成  
Azure App Service  
↓  
Managed Identity  
↓  
Azure Key Vault

ポイント  
- 構成漏えいリスクを減らせる
- シークレット更新がしやすい

関連リソース  
Azure App Service / Managed Identity / Azure Key Vault

出典  
- [[Sources/Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Sources/Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]

## 高可用な-web-実行基盤として-app-service-と-vm-を比較する

シナリオ  
高可用 Web アプリを PaaS で組むか、VM/VMSS で組むかを要件で切り分ける。

構成  
Option A: App Service + Front Door  
Option B: VMSS + Load Balancer / Traffic Manager

ポイント  
- OS 制御が不要なら App Service が運用軽量
- レガシー依存や特殊モジュールが強いなら VM 側

関連リソース  
Azure App Service / Azure Virtual Machines / VM Scale Sets

出典  
- [[Sources/Azure App Service 高可用性設計.md]]
- [[Sources/Azure Virtual Machine Scale Sets による高可用性 Web アプリケーション設計.md]]
