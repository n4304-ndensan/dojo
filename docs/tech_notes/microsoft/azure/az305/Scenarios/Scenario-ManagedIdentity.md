# Scenario: Managed Identity

## シナリオ一覧

- App Service から Key Vault へ接続する
- Functions から Storage と SQL へ接続する
- AKS Workload Identity で Azure SQL へ接続する

## app-service-から-key-vault-へ接続する

シナリオ  
App Service が接続文字列を持たずに Key Vault から秘密を取得する。

構成  
App Service  
↓  
Managed Identity  
↓  
Key Vault

ポイント  
- シークレットをアプリ設定から分離できる
- RBAC で取得権限だけを付与できる
- ローテーション時のアプリ差し替えが減る

関連リソース  
Managed Identity / Azure Key Vault / Azure App Service

出典  
- [[Sources/Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Sources/Azure Key Vault と Managed Identity を使用したシークレットアクセス制御.md]]

## functions-から-storage-と-sql-へ接続する

シナリオ  
Functions が Storage や SQL Database へパスワードレス接続する。

構成  
Azure Functions  
↓  
Managed Identity  
↓  
Azure Storage / Azure SQL Database

ポイント  
- 接続文字列管理を減らせる
- `DefaultAzureCredential` と相性がよい
- 権限過多になりやすいので RBAC の範囲を絞る

関連リソース  
Managed Identity / Azure Functions / Azure Storage / Azure SQL Database

出典  
- [[Sources/Azure Functions の安全な認証設計（Managed Identity + RBAC）.md]]
- [[Sources/Azure Blob Storage の暗号化とキー管理（Key Vault + Managed Identity）.md]]

## aks-workload-identity-で-azure-sql-へ接続する

シナリオ  
AKS 上の Pod から Azure SQL Database へシークレットレス接続する。

構成  
Pod  
↓  
Kubernetes Service Account  
↓  
Federated Identity Credential / Workload Identity  
↓  
Azure SQL Database

ポイント  
- Pod 単位に権限を分離できる
- ノード共有 ID より最小権限にしやすい
- Key Vault や Storage への展開もしやすい

関連リソース  
Managed Identity / AKS / Azure SQL Database

出典  
- [[Sources/AKS から Azure SQL Database に安全に接続する認証方式.md]]
- [[Sources/AKS における Azure リソース認証アーキテクチャ.md]]
