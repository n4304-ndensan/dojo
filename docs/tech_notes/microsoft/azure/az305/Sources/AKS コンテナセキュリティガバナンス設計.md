[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
# AKS コンテナセキュリティガバナンス設計

（承認レジストリ制限 + 読み取り専用ファイルシステム）

---

# 1 背景

ある企業では、ミッションクリティカルな本番ワークロードを **Azure Kubernetes Service（AKS）** 上で実行している。コンテナベースのアプリケーションでは、セキュリティやコンプライアンスの観点から、コンテナイメージの信頼性と実行環境の安全性を確保することが非常に重要である。

今回のシステムでは、Kubernetes クラスター上で実行されるすべてのワークロードに対して、次のセキュリティ要件が定義されている。

- コンテナイメージは **特定の承認されたレジストリからのみ取得**すること
    
- すべてのポッドは **読み取り専用のルートファイルシステム（readOnlyRootFilesystem）** を使用して実行すること
    
- これらのルールを **クラスター全体で強制**すること
    
- 運用担当者が手動でチェックするのではなく **自動的にコンプライアンスを維持すること**
    

これらの要件は、単一の Kubernetes 設定ではなく、**クラスター全体に対するガバナンスルール**として適用する必要がある。そのため、Azure のガバナンスサービスを利用してポリシーを適用する方法が適している。

この要件を満たす最適な Azure 機能は

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Policy]]
**Azure Policy for AKS**

である。

---

# 2 Azure Policy for AKS

Azure Policy は、Azure 環境におけるリソース構成を評価し、組織のセキュリティ基準やコンプライアンスルールを自動的に適用するサービスである。AKS に対して Azure Policy を適用することで、Kubernetes リソースの作成時にポリシー違反を検出し、デプロイを拒否したり監査したりすることができる。

Azure Policy for AKS は **Gatekeeper（Open Policy Agent）** をベースとしており、Kubernetes リソースの作成や更新時にポリシー評価が行われる。これにより、ポッドやコンテナの設定が組織のルールに違反している場合、そのリソースのデプロイを自動的にブロックすることができる。

Azure Policy for AKS の主な機能は次の通りである。

|機能|説明|
|---|---|
|ポリシー適用|Kubernetes構成ルールの強制|
|監査|非準拠リソースの検出|
|自動ブロック|違反リソースのデプロイ拒否|
|集中管理|Azure ポータルからポリシー管理|

---

# 3 承認レジストリ制限

コンテナセキュリティでは、信頼できないイメージを使用しないことが重要である。もし開発者が外部レジストリから未検証のイメージを使用すると、マルウェアや脆弱性を含むコンテナが本番環境にデプロイされる可能性がある。

Azure Policy を利用すると、コンテナイメージの取得元を制限するポリシーを設定できる。

例

```text
Allowed Container Registries

mycompany.azurecr.io
```

このポリシーを適用すると、次のようなデプロイは拒否される。

```text
docker.io/nginx
gcr.io/example/image
```

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
これにより、すべてのコンテナイメージを **Azure Container Registry（ACR）などの承認されたレジストリ**に限定することができる。

---

# 4 読み取り専用ルートファイルシステム

コンテナセキュリティのベストプラクティスの一つとして、コンテナのルートファイルシステムを読み取り専用にすることが推奨されている。

読み取り専用ルートファイルシステムを使用することで、次のリスクを低減できる。

- マルウェアの書き込み
    
- コンテナ内部の改ざん
    
- 不正なファイル生成
    

Kubernetes では次の設定でこの機能を有効化できる。

```yaml
securityContext:
  readOnlyRootFilesystem: true
```

Azure Policy for AKS を使用すると、すべてのポッドに対してこの設定を強制できる。

---

# 5 アーキテクチャ

Azure Policy を利用した AKS セキュリティ構成は次のようになる。

```text
Azure Policy
      │
      ▼
AKS Cluster
      │
      ├─ Allowed Registry Policy
      │
      ├─ readOnlyRootFilesystem Policy
      │
      ▼
Kubernetes Workloads
```

ポリシーは Kubernetes API サーバーに統合されており、ポッド作成時に自動的に評価される。

---

# 6 デプロイフロー

実際のポッドデプロイの流れは次のようになる。

```text
Developer
   │
   ▼
kubectl apply
   │
   ▼
AKS API Server
   │
   ▼
Azure Policy Evaluation
   │
   ├─ Compliant → Deploy
   │
   └─ Non-compliant → Reject
```

この仕組みにより、ポリシー違反のワークロードがクラスターにデプロイされることを防止できる。

---

# 7 他の選択肢が適切でない理由

Pod Security Policy（PSP）は Kubernetes でポッドのセキュリティ設定を制御する機能であったが、現在は Kubernetes の新しいバージョンでは廃止されている。そのため、AKS では PSP を使用したセキュリティ管理は推奨されていない。

Azure Container Registry のイメージスキャンはコンテナイメージの脆弱性を検出する機能であり、イメージの安全性を評価することはできるが、未承認レジストリからのイメージのデプロイをブロックする機能は提供していない。

Kubernetes のネットワークポリシーは、ポッド間通信やネットワークアクセスを制御するための機能であり、コンテナイメージのソースやファイルシステム設定を制御することはできない。

Azure Defender for Kubernetes はランタイムセキュリティ監視や脅威検知を提供するサービスであり、コンプライアンスルールの強制を行う仕組みではない。

---

# 8 メリット

Azure Policy for AKS を利用することで、Kubernetes クラスター全体に対して統一されたセキュリティポリシーを適用することができる。これにより、すべてのワークロードが組織のセキュリティ基準に準拠することを保証できる。

また、ポリシーは Azure ポータルから集中管理できるため、複数のクラスターに対して同じセキュリティルールを適用することも容易である。監査機能も備えているため、ポリシー違反のリソースを継続的に監視することも可能である。

---

# 9 まとめ

今回の要件は次の通りである。

- 承認されたレジストリのコンテナイメージのみ使用
    
- すべてのポッドを読み取り専用ファイルシステムで実行
    
- AKS クラスター全体に対してルールを強制
    

これらの要件を満たす最適な Azure 機能は

**Azure Policy for AKS**

である。

Azure Policy を利用することで、コンテナセキュリティポリシーをクラスター全体に自動的に適用し、AKS 環境のガバナンスとコンプライアンスを強化することができる。