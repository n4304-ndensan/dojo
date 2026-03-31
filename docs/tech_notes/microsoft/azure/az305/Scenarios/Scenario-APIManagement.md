# Scenario: Azure API Management

## シナリオ一覧

- Entra ID トークンを APIM で検証する
- Logic Apps を APIM 越しに外部公開する

## entra-id-トークンを-apim-で検証する

シナリオ  
内部 API の認証を各アプリ実装ではなく APIM 側へ共通化する。

構成  
Client  
↓  
Azure API Management  
↓  
Backend API

ポイント  
- JWT 検証やレート制限をポリシー化できる
- API ごとの差分を減らせる

関連リソース  
Azure API Management / Microsoft Entra ID

出典  
- [[Sources/Azure AD と API Management を用いた内部 API セキュリティ設計.md]]

## logic-apps-を-apim-越しに外部公開する

シナリオ  
Logic Apps を外部向け API として公開し、認証と入口管理を共通化する。

構成  
Partner  
↓  
Azure API Management  
↓  
Azure Logic Apps

ポイント  
- ワークフローと API 入口を分離できる
- VNet / 外部モード設計が重要

関連リソース  
Azure API Management / Azure Logic Apps

出典  
- [[Sources/Azure API Management を利用した Logic Apps の外部公開.md]]
- [[Sources/Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断.md]]
