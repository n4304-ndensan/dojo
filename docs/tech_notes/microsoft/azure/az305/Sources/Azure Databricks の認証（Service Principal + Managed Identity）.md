---
分類: Security / Identity
tags:
  - cloud/azure
  - cloud/azure/databricks
  - cloud/azure/identity
  - cloud/azure/service-principal
  - cloud/azure/managed-identity
  - cloud/security/authentication
  - cloud/security/service-identity
  - exam/azure
---

# Azure Databricks の認証（Service Principal + Managed Identity）

## 1. 背景（シナリオ）

Azure Databricks ワークスペースに対して認証を構成する必要があります。

このワークスペースには次のような環境からアクセスがあります。

Azure 環境で実行されるアプリケーション  
オンプレミス環境で実行されるアプリケーション  

さらに、組織には次のような運用上の要件があります。

頻繁な資格情報更新を避けたい  
スタッフの離職による影響を受けない認証方式  
安全なアクセス制御  
複雑な資格情報管理を避ける  

つまり、この問題は **アプリケーション認証（Machine to Machine 認証）** をどう設計するかという問題です。

## 2. 要件整理

問題文から読み取れる重要ポイントを整理します。

ユーザー認証ではなくアプリケーション認証  
複数アプリケーションからアクセス  
Azure とオンプレミス両方からアクセス  
資格情報の管理を最小化  
セキュアなアクセス  

これらの条件を満たす Azure のベストプラクティスは次の組み合わせです。

Service Principal  
Managed Identity  

## 3. 技術の基本概念

### Service Principal

Service Principal は **アプリケーション用の Azure AD アイデンティティ**です。

通常のユーザーアカウントではなく、  
**アプリケーション専用の ID** を Azure AD に作成します。

これにより

ユーザーアカウントに依存しない  
スタッフ退職の影響を受けない  
RBAC による権限管理が可能  

というメリットがあります。

### Managed Identity

Managed Identity は Azure リソース専用の ID です。

Azure が以下を自動管理します。

資格情報  
トークン発行  
トークン更新  

つまり **パスワードや証明書を管理する必要がありません。**

## 4. アーキテクチャ設計

このシナリオでは次の構成になります。

Azure AD  
↓  
Service Principal  
↓  
Managed Identity  
↓  
Azure Databricks

この構成では

Azure 内アプリ → Managed Identity  
オンプレミス → Service Principal  

という形でアクセスできます。

## 5. この方式のメリット

### 資格情報管理が不要

Managed Identity はトークンを自動更新します。

つまり

パスワード  
証明書  
キー

を管理する必要がありません。

### 人に依存しない

ユーザーアカウントではなく  
アプリケーション ID を使います。

そのため

社員退職  
アカウント削除

の影響を受けません。

### セキュリティ向上

RBAC を使用して

最小権限  
アクセス監査  
ログ追跡  

が可能になります。

## 6. 他の選択肢が誤りな理由

### A OAuth 2.0 トークン

OAuth はユーザー委任認証が中心です。

また

アクセストークン  
リフレッシュトークン

の管理が必要になります。

頻繁な更新が必要になるため要件に合いません。

### C クライアント証明書

証明書管理には次の問題があります。

証明書更新  
証明書配布  
証明書失効

運用が複雑になります。

### D Personal Access Token (PAT)

PAT はユーザー単位のトークンです。

そのため

ユーザー退職  
トークン期限

の影響を受けます。

## 7. 最終回答

B. **サービスプリンシパル + マネージドアイデンティティ**

## 8. 試験ポイントまとめ

試験では次のキーワードが出たら注意です。

アプリケーション認証  
資格情報管理を減らす  
ユーザーに依存しない  
Azure リソースアクセス  

この場合の正解はほぼ次です。

**Managed Identity**