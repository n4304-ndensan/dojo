# Azure Key Vault 技術ドキュメント

（Secrets / Keys / Certificates / ARM統合）

---

# 1 概要

**Azure Key Vault** は、クラウドアプリケーションで使用する **機密情報を安全に保存・管理するサービス**である。

管理対象のデータ

- シークレット（パスワード・接続文字列）
    
- 暗号キー
    
- 証明書
    

Azure Key Vaultを使用することで

- 秘密情報をコードに埋め込まない
    
- セキュアな鍵管理
    
- 中央管理
    

が実現できる。

基本構成

```text
Application
     │
     ▼
Azure Key Vault
     │
     ├ Secrets
     ├ Keys
     └ Certificates
```

---

# 2 背景

アプリケーションでは多くの機密情報を扱う。

例

- DB接続文字列
    
- APIキー
    
- OAuth秘密鍵
    
- TLS証明書
    

従来の方法

```text
appsettings.json
web.config
source code
```

問題

- Gitに保存される
    
- 開発者が閲覧できる
    
- セキュリティリスク
    

そのため

```text
Secrets → Key Vault
```

という設計が推奨される。

---

# 3 Key Vault の基本構造

Key Vaultの構成

```text
Azure Subscription
        │
        ▼
Resource Group
        │
        ▼
Key Vault
        │
        ├ Secrets
        ├ Keys
        └ Certificates
```

Vaultは

**機密情報のコンテナ**

として機能する。

---

# 4 Key Vault の主要機能

## 4.1 Secrets

Secretsは

**機密データの保存**

である。

例

```text
DBPassword
APIKey
ConnectionString
```

構造

```text
Secret Name
Secret Value
Version
```

例

```text
DatabasePassword
*************
```

---

## 4.2 Keys

Keysは

**暗号鍵の管理**

である。

用途

- データ暗号化
    
- 署名
    
- トークン
    

Key Vaultは

- RSA
    
- ECC
    

などをサポートする。

構成

```text
Application
    │
    ▼
Key Vault Key
    │
    ▼
Encryption / Decryption
```

---

## 4.3 Certificates

証明書管理

```text
TLS Certificates
```

用途

- HTTPS
    
- API認証
    

Key Vaultでは

- 証明書発行
    
- 自動更新
    

が可能。

---

# 5 Key Vault アクセス制御

Key Vaultには2種類のアクセス制御方式がある。

|方式|説明|
|---|---|
|Access Policy|従来方式|
|Azure RBAC|新方式|

---

## 5.1 Access Policy

Key Vault内の

- Secrets
    
- Keys
    
- Certificates
    

へのアクセス権を設定する。

例

```text
User
App
Service Principal
```

アクセス権

```text
Get
List
Set
Delete
```

構成

```text
Key Vault
     │
     ▼
Access Policy
     │
     ▼
User / Application
```

---

## 5.2 Azure RBAC

Azure RBACを使用することで

```text
Role Assignment
```

でKey Vaultを管理できる。

例

|Role|権限|
|---|---|
|Key Vault Reader|読み取り|
|Key Vault Secrets User|Secretsアクセス|
|Key Vault Administrator|管理|

---

# 6 ARMテンプレートとの統合

Azure Resource Managerテンプレートでは

**Key VaultのSecretsを参照できる。**

構成

```text
ARM Template
      │
      ▼
Key Vault
      │
      ▼
Secret
```

例

```json
reference(
  keyVaultResourceId,
  'secretName'
)
```

この場合

**Key Vault Access Policy**

を設定しないとアクセスできない。

---

# 7 Managed Identityとの統合

Azureでは

**Managed Identity**

を使ってKey Vaultへアクセスできる。

構成

```text
Application
     │
     ▼
Managed Identity
     │
     ▼
Key Vault
```

メリット

- パスワード不要
    
- 自動認証
    

---

# 8 Key Vault ネットワークセキュリティ

Key Vaultはネットワーク制御も可能。

方法

|機能|用途|
|---|---|
|Firewall|IP制御|
|Virtual Network|VNet接続|
|Private Endpoint|Privateアクセス|

構成

```text
Application
     │
     ▼
Private Endpoint
     │
     ▼
Key Vault
```

---

# 9 Key Vault セキュリティ機能

## Soft Delete

削除された秘密を復元可能。

例

```text
Deleted Secret
   │
   ▼
Recover
```

---

## Purge Protection

完全削除防止。

---

## HSM

Key Vaultには

**Hardware Security Module**

が利用できる。

SKU

|タイプ|用途|
|---|---|
|Standard|一般用途|
|Premium|HSM対応|

---

# 10 アーキテクチャ例

## アプリケーションシークレット管理

```text
Application
     │
     ▼
Managed Identity
     │
     ▼
Azure Key Vault
     │
     ▼
Database Password
```

---

## ARMデプロイメント

```text
ARM Template
     │
     ▼
Key Vault Secret
     │
     ▼
VM Deployment
```

---

# 11 Azure試験でよく出るポイント

試験では次のポイントが頻出。

|質問|答え|
|---|---|
|秘密情報管理|Key Vault|
|ARMテンプレートで秘密取得|Access Policy|
|証明書管理|Key Vault|
|暗号鍵管理|Key Vault|

---

# 12 まとめ

Azure Key Vaultは

**機密情報管理サービス**

である。

管理対象

|種類|用途|
|---|---|
|Secrets|パスワード|
|Keys|暗号鍵|
|Certificates|証明書|

アクセス制御

|方式|説明|
|---|---|
|Access Policy|Key Vault内部権限|
|RBAC|Azureロール管理|

ARMテンプレートでSecretsを利用するには

**Key Vault Access Policy**

を有効化する必要がある。