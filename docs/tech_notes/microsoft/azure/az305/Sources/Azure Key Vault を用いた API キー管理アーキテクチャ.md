[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
# Azure Key Vault を用いた API キー管理アーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
（Secret + Managed Identity）

---

# 1 背景

企業は Ubuntu 仮想マシン上で動作するアプリケーションを開発しており、そのアプリケーションは **サードパーティのメールサービス API** を利用する必要がある。メールサービスを利用するには **API キー**による認証が必要である。

API キーは機密情報であり、アプリケーションコードや設定ファイルに直接保存すると以下のリスクがある。

- ソースコード漏洩によるキー流出
    
- Git リポジトリへの誤コミット
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
- VM 侵害によるキー盗難
    

そのため Azure では **Azure Key Vault** を利用して機密情報を安全に管理することが推奨されている。

今回の要件は次の通りである。

- API キーを安全に保存する
    
- Ubuntu VM 上のアプリからアクセスする
    
- **管理作業を最小化**する
    

---

# 2 Azure Key Vault とは

Azure Key Vault は Azure が提供する **機密情報管理サービス**である。

Key Vault では以下のデータを保存できる。

|種類|用途|
|---|---|
|Secrets|APIキー、パスワード|
|Keys|暗号鍵|
|Certificates|TLS証明書|

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
これらを Azure AD による認証で安全に管理できる。

---

# 3 APIキーを保存する方法

API キーは通常 **ランダムな文字列トークン**である。

例

```text
4c82dfab89d3124e8cfa9123
```

このようなデータは Azure Key Vault では **Secret（シークレット）**として保存するのが適切である。

理由

- 単なる機密文字列
    
- 暗号鍵ではない
    
- 証明書でもない
    

そのため

```text
Key Vault → Secret
```

として保存する。

---

# 4 Managed Identity

次に重要なのは **VM から Key Vault へ安全にアクセスする方法**である。

通常の認証方法

|方法|問題|
|---|---|
|APIキー|キー管理が必要|
|サービスプリンシパル|シークレット管理必要|
|証明書|証明書管理必要|

これらはすべて **認証情報管理が必要**になる。

Azure ではこれを解決するために **Managed Identity** が提供されている。

---

# 5 Managed Identity の仕組み

Managed Identity は Azure リソース専用の **自動管理される ID**である。

特徴

- Azure AD による認証
    
- パスワード不要
    
- 自動ローテーション
    
- 秘密情報管理不要
    

構成は次のようになる。

```text
Ubuntu VM
     │
Managed Identity
     │
     ▼
Azure AD
     │
     ▼
Key Vault
```

アプリケーションは **認証情報を持たずに Key Vault にアクセスできる**。

---

# 6 システム割り当て ID

Managed Identity には2種類ある。

|種類|説明|
|---|---|
|System-assigned|VMに自動作成|
|User-assigned|共有ID|

通常は **System-assigned Managed Identity** を使用する。

---

# 7 Key Vault アクセス設定

VM の Managed Identity に Key Vault のアクセス権を付与する。

設定方法

1. Key Vault を作成
    
2. Secret を保存
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Role-Based Access Control (RBAC)]]
3. Access Policy または RBAC を設定
    
4. VM の Managed Identity に権限付与
    

例

```text
Permission: Get Secret
```

---

# 8 実行時のアクセスフロー

アプリケーションの実行フローは次の通り。

```text
Application
     │
     ▼
Managed Identity Token
     │
     ▼
Azure AD
     │
     ▼
Key Vault
     │
     ▼
API Key Secret
```

アプリケーションは Key Vault から API キーを取得してメールサービスを呼び出す。

---

# 9 実装例

Python 例

```python
from azure.identity import DefaultAzureCredential
from azure.keyvault.secrets import SecretClient

credential = DefaultAzureCredential()

client = SecretClient(
    vault_url="https://myvault.vault.azure.net/",
    credential=credential
)

secret = client.get_secret("email-service-api-key")

api_key = secret.value
```

このコードでは **資格情報をコードに保存していない**。

---

# 10 他の選択肢が不適切な理由

### Key として保存

Key Vault の Key は暗号処理用の鍵であり API トークン用途ではない。

---

### Certificate として保存

証明書は TLS や署名用途であり API キーには適さない。

---

### Service Principal

サービスプリンシパルを使用すると

- Client secret 管理
    
- Secret rotation
    

などの管理作業が増える。

---

# 11 推奨アーキテクチャ

最適構成

```text
Ubuntu VM
     │
Managed Identity
     │
     ▼
Azure Key Vault
     │
     ▼
Secret (API Key)
```

---

# 12 最終回答

正解

**D**

```text
Secret として保存
Managed Identity でアクセス
```

---

# 13 まとめ

今回の要件

|要件|解決策|
|---|---|
|APIキー保存|Key Vault Secret|
|認証情報管理不要|Managed Identity|
|セキュアアクセス|Azure AD|

そのため最適構成は

```text
Azure Key Vault Secret
+
Managed Identity
```

である。

この設計は Azure における **機密情報管理のベストプラクティス**として広く使用されている。