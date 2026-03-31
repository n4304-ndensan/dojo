# Scenario: Azure Key Vault

## シナリオ一覧

- 共通構成シークレットを集約する
- App Service から Key Vault の秘密を参照する
- VM から Key Vault の証明書を参照する

## 共通構成シークレットを集約する

シナリオ  
複数アプリケーションで共有する API キー、接続文字列、証明書を中央管理する。

構成  
Multiple Apps  
↓  
Azure Key Vault  
↓  
RBAC / Versioning / Rotation

ポイント  
- 秘密情報をソースコードや設定ファイルから外せる
- バージョン管理とローテーションを統一できる
- 監査ログの取得先を集約しやすい

関連リソース  
Azure Key Vault / Microsoft Entra ID / Managed Identity

出典  
- [[Sources/Azure Key Vault による機密構成データのセキュア管理.md]]
- [[Sources/Azure Key Vault による機密情報の安全な管理.md]]

## app-service-から-key-vault-の秘密を参照する

シナリオ  
App Service のアプリ設定から秘密を排除し、Key Vault 参照へ寄せる。

構成  
App Service  
↓  
Managed Identity  
↓  
Azure Key Vault

ポイント  
- アプリ設定の漏えいリスクを下げる
- ローテーションの反映が速い
- Private Endpoint を足すとネットワーク面も閉じられる

関連リソース  
Azure App Service / Managed Identity / Azure Key Vault

出典  
- [[Sources/Azure App Service で Key Vault の秘密を利用する設計.md]]
- [[Sources/Azure Key Vault への安全なアクセス設計（Managed Identity）.md]]

## vm-から-key-vault-の証明書を参照する

シナリオ  
VM 上のアプリケーションが証明書やシークレットを Key Vault から安全に取得する。

構成  
Azure VM  
↓  
Managed Identity  
↓  
Azure Key Vault

ポイント  
- VM 内に証明書ファイルを固定配置しなくてよい
- アプリ更新と証明書更新を分離できる
- 既存アプリの段階的移行に向く

関連リソース  
Azure Virtual Machines / Managed Identity / Azure Key Vault

出典  
- [[Sources/Azure VM 上の ASP.NET Core アプリから Key Vault を安全に利用する設計.md]]
- [[Sources/Azure Key Vault に安全にアクセスするためのマネージドID.md]]
