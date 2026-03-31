# Scenario: Microsoft Entra ID

## シナリオ一覧

- 社内 Web アプリの SSO
- 外部ユーザーの B2B 招待とアクセスレビュー
- 委任アクセスで Graph API を呼び出す
- ハイブリッド ID の認証方式選定

## 社内webアプリのsso

シナリオ  
社内 Web アプリの認証を Entra ID に統一し、SSO と条件付きアクセスをまとめて適用する。

構成  
User  
↓  
Microsoft Entra ID  
↓  
App Service / Web App

ポイント  
- 認証をアプリ実装から分離できる
- MFA と Conditional Access を入口で適用できる
- グループやロールで認可設計を続けやすい

関連リソース  
Microsoft Entra ID / App Service / Conditional Access

出典  
- [[Sources/Microsoft Entra ID を利用した Azure Web アプリケーション認証アーキテクチャ.md]]
- [[Sources/Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ.md]]

## 外部ユーザーのb2b招待とアクセスレビュー

シナリオ  
パートナーや外部開発者をゲストとして招待し、一定期間ごとにアクセス棚卸しを行う。

構成  
External User  
↓  
Microsoft Entra ID B2B  
↓  
Group / App / Access Reviews

ポイント  
- ゲスト ID を自社テナントで集中管理できる
- Access Reviews で不要権限の残留を防げる
- Entitlement Management と組み合わせると期限付き付与にしやすい

関連リソース  
Microsoft Entra ID / Access Reviews / B2B

出典  
- [[Sources/Azure Entra ID における外部ユーザーアクセス管理（B2B とアクセスレビュー）.md]]
- [[Sources/外部ユーザーアクセスの定期監査（Azure AD Access Reviews）.md]]

## 委任アクセスでgraph-apiを呼び出す

シナリオ  
サインイン済みユーザーの権限で Graph API にアクセスし、カレンダーやプロフィール情報を取得する。

構成  
User  
↓  
App Registration  
↓  
Microsoft Graph

ポイント  
- Delegated Permissions と Application Permissions を切り分ける
- 最小権限のスコープ設計が重要
- トークン取得と API 保護を Entra ID に集約できる

関連リソース  
Microsoft Entra ID / App Registration / Microsoft Graph

出典  
- [[Sources/Azure AD を利用した ASP.NET Core アプリのユーザー委任アクセス設計.md]]
- [[Sources/SaaS アプリにおける Azure AD と OAuth 2.0 を利用した認証・認可設計.md]]

## ハイブリッド-id-の認証方式を選定する

シナリオ  
オンプレミス AD を残したまま Azure 側へ認証を拡張し、PHS/PTA/SSO を要件で選び分ける。

構成  
On-prem AD  
↓  
Entra Connect  
↓  
Microsoft Entra ID

ポイント  
- 可用性重視なら PHS が候補になりやすい
- 既存認証経路を残したいなら PTA を比較する
- 同期範囲は OU / Group 単位で絞る

関連リソース  
Microsoft Entra ID / Entra Connect / Hybrid Identity

出典  
- [[Sources/Azure AD Connect によるハイブリッドID同期設計.md]]
- [[Sources/Microsoft Entra ID ハイブリッド認証方式ドキュメント.md]]
