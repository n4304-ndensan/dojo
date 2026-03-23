[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
# Azure App Service で Key Vault の秘密を利用する設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
（Key Vault Reference + Managed Identity）

---

# 1 背景

クラウド環境でアプリケーションを運用する場合、次のような **機密情報（Secrets）**を安全に管理する必要がある。

代表的な例

- データベース接続文字列
    
- API キー
    
- OAuth シークレット
    
- パスワード
    

これらの機密情報を **コードや設定ファイルに直接保存することは推奨されない**。理由は次の通りである。

- ソースコード漏洩のリスク
    
- Git リポジトリへの誤コミット
    
- CI/CD ログへの露出
    

Azure では、この問題を解決するために **Azure Key Vault** を使用する。

Key Vault は機密情報を安全に保存し、アプリケーションが必要なときに安全に取得できるサービスである。

---

# 2 シナリオ

今回のシナリオでは次の構成がある。

- Web アプリケーションが Azure 上で実行されている
    
- Web アプリは **System Assigned Managed Identity** を使用する
    
- Key Vault に保存された **Secret** を読み取る必要がある
    

さらに重要な要件がある。

---

# 3 要件整理

問題文から重要な要件を整理する。

### ① Managed Identity を使用

アプリケーションは

```text
System Assigned Managed Identity
```

を利用して Key Vault にアクセスする。

つまり

```text
認証情報をコードに保存しない
```

設計である。

---

### ② 最小限のコード変更

既存のアプリケーションコードを **ほとんど変更せず**に Key Vault を利用する必要がある。

---

### ③ 最小権限

Key Vault に対する権限は

```text
必要最低限
```

にする必要がある。

---

### ④ アプリ設定として Secret を使用

Key Vault の秘密は **アプリケーション設定（App Settings）として参照される必要がある**。

---

# 4 Key Vault とアプリ統合方法

Azure App Service と Key Vault を統合する方法はいくつかある。

|方法|特徴|
|---|---|
|Key Vault SDK|コードから直接取得|
|Key Vault Reference|設定から自動取得|
|Environment Variables|手動設定|

---

# 5 Key Vault SDK

SDK を使用する方法では、アプリケーションコードから Key Vault にアクセスする。

例

```csharp
var client = new SecretClient(
    new Uri("https://myvault.vault.azure.net"),
    new DefaultAzureCredential());

KeyVaultSecret secret = client.GetSecret("DbPassword");
```

問題点

- コード変更が必要
    
- SDK 依存が増える
    
- メンテナンスが増える
    

そのため今回の要件

```text
最小限のコード変更
```

には適さない。

---

# 6 Key Vault Reference

Azure App Service には **Key Vault Reference** という機能がある。

この機能を使用すると、アプリ設定から Key Vault の秘密を直接参照できる。

例

```text
@Microsoft.KeyVault(SecretUri=https://myvault.vault.azure.net/secrets/DbPassword/)
```

この設定を **App Settings** に記述するだけで、プラットフォームが自動的に Secret を取得する。

---

# 7 Key Vault Reference の仕組み

アプリケーションが設定を読み込むと、Azure App Service が自動的に Key Vault にアクセスする。

フロー

```text
App Service
     │
     ▼
Managed Identity
     │
     ▼
Azure AD
     │
     ▼
Key Vault
     │
     ▼
Secret
```

アプリケーション側では

```text
通常の環境変数
```

として値を読み取れる。

---

# 8 必要な Key Vault 権限

Key Vault の権限は **最小権限の原則**に従う必要がある。

今回必要な権限は

```text
Secrets.Get
```

のみである。

理由

- Secret を取得するだけ
    
- Secret の一覧取得は不要
    

---

### List 権限

```text
Secrets.List
```

は不要である。

この権限があると

```text
Key Vault 内のすべての Secret 名を取得
```

できてしまうため、権限が過剰になる。

---

# 9 推奨アーキテクチャ

今回の構成は次のようになる。

```text
Web App
   │
   ▼
App Settings
(Key Vault Reference)
   │
   ▼
Managed Identity
   │
   ▼
Azure AD
   │
   ▼
Azure Key Vault
   │
   ▼
Secret
```

---

# 10 実装例

### App Service 設定

App Settings

```text
DbPassword = @Microsoft.KeyVault(SecretUri=https://myvault.vault.azure.net/secrets/db-password/)
```

---

### Key Vault 権限

Managed Identity に次の権限を付与

```text
Secrets.Get
```

---

# 11 他の選択肢が不適切な理由

### A

Key Vault SDK + Secrets.Get

コード変更が必要。

---

### B

appsettings.json

App Service の Key Vault Reference ではない。

---

### D

Web.config + List + Get

権限が過剰。

---

# 12 最終回答

正解

**C**

```text
Integration Method
Application Settings Key Vault Reference

Permission
Secrets.Get
```

---

# 13 まとめ

今回の設計ポイント

|要件|解決方法|
|---|---|
|コード変更最小|Key Vault Reference|
|認証|Managed Identity|
|権限|Secrets.Get|
|設定|App Settings|

つまり最適構成は

```text
Key Vault Reference
+
Managed Identity
+
Secrets.Get
```

である。

これは **Azure App Service と Key Vault を統合する際の Microsoft 推奨アーキテクチャ**である。