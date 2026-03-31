# Scenario: Azure Kubernetes Service (AKS)

## シナリオ一覧

- リージョン障害に耐える AKS 基盤を作る
- Azure Files RWX でステートフルワークロードを支える
- ACR 更新をトリガーに AKS へ継続的デプロイする

## リージョン障害に耐える-aks-基盤を作る

シナリオ  
ビジネスクリティカルなコンテナ基盤を AKS で構成し、リージョン障害まで考慮する。

構成  
Region A AKS  
↔  
Region B AKS  
↓  
Global Routing

ポイント  
- 単一クラスター冗長化とマルチリージョンは分けて考える
- 制御プレーンを Azure に任せつつ DR を設計する

関連リソース  
AKS / Front Door / Traffic Manager

出典  
- [[Sources/AKS を使用したリージョン障害耐性のある Kubernetes アーキテクチャ.md]]
- [[Sources/ステートフルWebアプリケーションの高可用アーキテクチャ（AKS + Traffic Manager）.md]]

## azure-files-rwx-でステートフルワークロードを支える

シナリオ  
複数 Pod から共有アクセスできる永続ストレージを AKS に提供する。

構成  
Pods  
↓  
PersistentVolumeClaim  
↓  
Azure Files

ポイント  
- ReadWriteMany が必要なときの標準選択肢
- ノード再スケジュール時もデータを維持しやすい

関連リソース  
AKS / Azure Files / PersistentVolume

出典  
- [[Sources/AKS ステートフルアプリケーションの永続ストレージ（Azure Files RWX）.md]]
- [[Sources/AKS ステートフルアプリケーションの共有ストレージ設計.md]]

## acr-更新をトリガーに-aks-へ継続的デプロイする

シナリオ  
ACR へ新しいイメージが入ったら AKS へ自動デプロイする。

構成  
Source / Build  
↓  
Azure Container Registry  
↓  
Azure DevOps / Pipeline  
↓  
AKS

ポイント  
- イメージ供給とデプロイを分離できる
- リング展開や承認ゲートも追加しやすい

関連リソース  
AKS / Azure Container Registry / Azure DevOps

出典  
- [[Sources/Azure DevOps Pipeline を利用した AKS への継続的デプロイ（ACR イメージ更新トリガー）.md]]
- [[Sources/AKS マルチリージョンアプリケーションの安全なリリース戦略.md]]
