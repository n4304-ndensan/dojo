[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
## Azure App Service 構成管理アーキテクチャ設計

### ― Azure Repos と ZIP Deployment を利用したコードベース Web アプリケーション管理 ―

---

# 1 背景と問題のコンテキスト

企業がクラウド環境で多数の Web アプリケーションを運用する場合、**アプリケーションの構成管理と変更履歴管理**が非常に重要になる。今回のシナリオでは、Azure サブスクリプション内に **50 個の Azure App Service インスタンス**を展開し、それぞれでコードベースの Web アプリケーションをホストする予定である。

Azure App Service は PaaS 型の Web アプリケーション実行基盤であり、アプリケーション開発者は OS やミドルウェアの管理を行う必要がなく、アプリケーションコードのデプロイに集中できる。しかし、アプリケーションが増えてくると、次のような課題が発生する。

- アプリケーションの設定変更を管理する必要がある
    
- 誰がいつ変更したか履歴を追跡する必要がある
    
- 複数の開発者が同時に変更する可能性がある
    
- 問題が発生した場合、以前のバージョンへ戻す必要がある
    

このような運用課題に対応するため、**ソースコードとデプロイパッケージをバージョン管理システムで管理する方法**が一般的に採用される。

今回の要件では特に次の点が重要である。

- App Service の設定やアプリケーションコードを定義すること
    
- 設定変更の履歴を長期的に保存すること
    
- バージョン管理された状態で変更履歴を追跡できること
    

この要件を満たす最もシンプルな方法は、**Azure Repos を使用してアプリケーションを ZIP パッケージとして管理し、App Service にデプロイする方法**である。

---

# 2 Azure App Service の役割

Azure App Service は Microsoft が提供する **Web アプリケーション向けの PaaS プラットフォーム**である。アプリケーションの実行環境をクラウドで提供し、開発者はインフラ管理を行う必要がない。

App Service は以下のような言語やフレームワークをサポートしている。

- .NET
    
- Java
    
- Node.js
    
- Python
    
- PHP
    

アプリケーションの実行構造は次のようになる。

```text
Developer
   │
   ▼
Application Source Code
   │
   ▼
Azure App Service
   │
   ▼
Running Web Application
```

App Service は自動スケールやロードバランシングなどを提供するため、企業の Web アプリケーション基盤として広く使用されている。

---

# 3 Azure Repos の役割

Azure Repos は **Git ベースのソースコード管理システム**であり、Azure DevOps の主要コンポーネントの一つである。Git を利用することで、アプリケーションコードや構成ファイルの変更履歴を長期間保存できる。

Azure Repos を利用すると次のような管理が可能になる。

- コード変更履歴の保存
    
- 開発者ごとの変更追跡
    
- ブランチによる並行開発
    
- Pull Request によるレビュー
    
- 過去バージョンへのロールバック
    

Git リポジトリは次のような構造で管理される。

```text
Repository
 ├ main
 ├ feature/login
 ├ feature/api
 └ hotfix/security
```

これにより、アプリケーションの構成変更や機能追加の履歴を完全に追跡できる。

---

# 4 ZIP Deployment の仕組み

App Service には複数のデプロイ方法が存在するが、コードベースのアプリケーションでは **ZIP Deploy** がよく使用される。

ZIP Deploy は、アプリケーションのコードを ZIP ファイルとしてまとめ、それを App Service にアップロードすることでデプロイする方式である。

ZIP デプロイの流れは次の通りである。

```text
Application Source
        │
        ▼
ZIP Package
        │
        ▼
Azure App Service
        │
        ▼
Application deployed to /wwwroot
```

App Service は ZIP ファイルを展開し、内部の `wwwroot` ディレクトリへ配置してアプリケーションを実行する。

この方式の利点は次の通り。

- シンプルなデプロイ方法
    
- CI/CD パイプラインと統合しやすい
    
- バージョン管理が容易
    

---

# 5 推奨アーキテクチャ

今回のシナリオで推奨される構成は次の通りである。

```text
Developers
     │
     ▼
Azure Repos (Git)
     │
     ▼
ZIP Package
     │
     ▼
Azure App Service
     │
     ▼
Running Web Applications
```

この構成では、すべてのアプリケーションコードが Git リポジトリで管理されるため、変更履歴が自動的に保存される。

---

# 6 変更履歴の管理

Git を使用すると、アプリケーションの変更履歴を詳細に管理できる。例えば次のような履歴が残る。

```text
Commit History

v1.0 Initial release
v1.1 Add authentication
v1.2 Fix API bug
v1.3 Update configuration
```

もし問題が発生した場合、過去の安定したバージョンに戻すことも可能である。

```text
rollback → v1.1
```

このように、Git はアプリケーション運用において非常に重要な役割を果たす。

---

# 7 多数の App Service を管理する場合

50 個の App Service インスタンスを管理する場合、リポジトリ構造は次のように整理できる。

```text
repo
 ├ app1
 ├ app2
 ├ app3
 ├ app4
 └ app50
```

または、ZIP パッケージとして管理することもできる。

```text
repo
 ├ app1.zip
 ├ app2.zip
 ├ app3.zip
 └ app50.zip
```

CI/CD パイプラインを導入すると、コードの変更からデプロイまでを自動化できる。

```text
Developer
   │
   ▼
Azure Repos
   │
   ▼
Build Pipeline
   │
   ▼
ZIP Package
   │
   ▼
Deploy to App Service
```

---

# 8 他の選択肢が適さない理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Container Registry]]
### Azure Container Registry

Azure Container Registry は Docker コンテナイメージを管理するサービスであり、コンテナベースのアプリケーションに適している。しかし今回のシナリオは **コードベースの App Service アプリケーション**であるため、コンテナレジストリは不要である。

### Azure Compute Gallery

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
Azure Compute Gallery は仮想マシンのイメージ管理サービスであり、VM のスケールアウトなどに使用される。App Service は PaaS サービスであるため、VM イメージ管理は関係しない。

### Bicep ファイル

Bicep は Infrastructure as Code (IaC) を実現するためのツールであり、Azure リソースの構成を管理するために使用される。Bicep はインフラ定義には適しているが、アプリケーションコードの履歴管理には適していない。

---

# 9 最終アーキテクチャ

最終的な構成は次のようになる。

```text
Developers
     │
     ▼
Azure Repos (Git Repository)
     │
     ▼
ZIP Package
     │
     ▼
Azure App Service (50 instances)
     │
     ▼
Running Web Applications
```

この構成では、アプリケーションのコードと設定が Git で管理されるため、変更履歴を追跡でき、将来の運用やトラブル対応が容易になる。

---

# 10 まとめ

今回の要件は次の2点である。

- App Service 設定を定義する
    
- 設定変更の履歴をバージョン管理する
    

この要件を満たす最適な方法は

**Azure Repos でアプリケーションデプロイメントを ZIP パッケージとして管理する方法**

である。

この設計により

- Git による変更履歴管理
    
- バージョン管理されたデプロイ
    
- CI/CD との統合
    
- 大量アプリケーションの効率的管理
    

を実現できる。