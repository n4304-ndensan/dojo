[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Private Endpoint]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
# Azure Private Endpoint を使用した SQL Database の DNS 名前解決設計

（オンプレミスから Azure Private Endpoint へアクセスする DNS 構成）

---

# 1 背景（シナリオ）

企業では、セキュリティ要件の強化により Azure PaaS サービスを **パブリックエンドポイントではなく Private Endpoint 経由で利用するケース**が増えている。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
今回のシナリオでは、Azure SQL Database が **Private Endpoint を利用して VNET 内に配置**されている。

構成は以下の通り。

|リソース|説明|
|---|---|
|SQLDB1|Azure SQL Database|
|PE1|SQLDB1 に接続された Private Endpoint|
|VNET1|Private Endpoint が配置されている VNET|
|VM1|Azure 内のカスタム DNS サーバー|
|オンプレミス|社内ユーザー環境|

オンプレミスユーザーは SQLDB1 にアクセスする必要があるが、SQLDB1 は **Private Endpoint を利用しているためパブリック DNS では解決できない**。

そのため、オンプレミスユーザーが SQLDB1 にアクセスするためには **SQLDB1 の Private IP アドレスを正しく解決できる DNS 設計**が必要になる。

---

# 2 Private Endpoint の基本

Azure Private Endpoint は PaaS サービスを **VNET 内のプライベート IP アドレスとして公開する機能**である。

通常の Azure SQL Database の接続は以下のようになる。

```
sqlserver.database.windows.net
      │
      ▼
Public IP
```

しかし Private Endpoint を利用すると次のようになる。

```
sqlserver.database.windows.net
      │
      ▼
Private DNS Zone
      │
      ▼
Private IP（VNET 内）
```

つまり

```text
SQL Database への接続は Private IP で行われる
```

---

# 3 Private Endpoint を利用した SQL Database の DNS

Azure SQL の Private Endpoint を使用すると DNS レコードは次の Private DNS ゾーンに作成される。

```
privatelink.database.windows.net
```

DNS レコード例

```
sqlserver.database.windows.net
    CNAME
sqlserver.privatelink.database.windows.net
    A
10.1.0.5
```

この構成により SQLDB1 の名前解決は **Private IP に変換される**。

---

# 4 問題のポイント

今回のシナリオで重要なポイントは次の 3 点である。

### ① SQL Database は Private Endpoint

SQLDB1 は

```
PE1
```

を使用しているため、DNS は **Private DNS を利用する必要がある**。

---

### ② オンプレミスユーザーがアクセス

オンプレミスユーザーは Azure 内の DNS 情報を直接参照できない。

つまり

```
オンプレミス DNS
↓
Azure DNS
```

の **DNS フォワーディング**が必要になる。

---

### ③ Azure にカスタム DNS サーバーが存在

Azure VNET 内には

```
VM1
```

という DNS サーバーがある。

この DNS サーバーが **オンプレミスと Azure DNS の橋渡し**を行う。

---

# 5 Azure の DNS リゾルバ

Azure には内部 DNS リゾルバが存在する。

IP アドレス

```
168.63.129.16
```

これは Azure が提供する内部 DNS サービスであり、

- Private DNS Zone
    
- Azure 内部名前解決
    

を処理する。

---

# 6 推奨 DNS アーキテクチャ

今回の正しい DNS フローは次のようになる。

```
オンプレミスユーザー
        │
        ▼
オンプレミス DNS
        │
        ▼
条件付きフォワーダー
        │
        ▼
VM1（Azure DNS サーバー）
        │
        ▼
Azure DNS Resolver
168.63.129.16
        │
        ▼
Private DNS Zone
        │
        ▼
SQLDB1 Private IP
```

---

# 7 DNS 設定の詳細

必要な設定は次の通り。

### ① オンプレミス DNS

オンプレミス DNS に **条件付きフォワーダー**を設定する。

```
contoso.com → VM1
```

これにより

```
オンプレミス → Azure DNS
```

の問い合わせが可能になる。

---

### ② VNET1 DNS 設定

VNET1 の DNS は

```
VM1
```

を使用する。

つまり

```
Custom DNS Server
```

として VM1 の IP を設定する。

---

### ③ VM1 の DNS 設定

VM1 は Azure DNS リゾルバへ転送する。

```
168.63.129.16
```

これにより

```
Private DNS Zone
```

が解決される。

---

# 8 SQL Private Endpoint DNS 解決

名前解決の流れは次の通り。

```
sqlserver.database.windows.net
        │
        ▼
privatelink.database.windows.net
        │
        ▼
Private IP
```

オンプレミスユーザーもこの Private IP を取得できる。

---

# 9 他の選択肢が不適切な理由

### B

Azure Public DNS に転送

これは Private Endpoint を無視してしまう。

```
Public IP
```

が返される可能性がある。

---

### C

168.63.129.16 へ直接転送

オンプレミス DNS は

```
168.63.129.16
```

へ直接アクセスできない。

このアドレスは **Azure 内部専用 DNS** である。

---

### D

オンプレミス DNS ゾーン管理

オンプレミスで DNS ゾーンを管理すると

```
Private DNS Zone
```

の情報と同期できない。

---

# 10 正しい DNS 設計

今回の最適構成は以下。

|設定|内容|
|---|---|
|オンプレミス DNS|contoso.com を VM1 に転送|
|VNET1 DNS|VM1 を使用|
|VM1 DNS|168.63.129.16 へフォワード|

---

# 11 最終アーキテクチャ

```
On-premises User
        │
        ▼
On-prem DNS
        │
        ▼
Conditional Forwarder
        │
        ▼
VM1 (Azure DNS Server)
        │
        ▼
Azure DNS Resolver
168.63.129.16
        │
        ▼
Private DNS Zone
privatelink.database.windows.net
        │
        ▼
SQLDB1 Private IP
```

---

# 12 最終回答

正解

```
A
オンプレミス DNS から VM1 に転送
```

ただし実際の設計では

```
VNET1 DNS → VM1
```

に設定する必要がある。

---

# 13 まとめ

この問題の本質は **Private Endpoint の DNS 解決方法**である。

重要なポイント

|項目|内容|
|---|---|
|Private Endpoint|Private IP 接続|
|Private DNS Zone|privatelink.database.windows.net|
|DNS 中継|VM1|
|Azure DNS Resolver|168.63.129.16|

つまり正しい構成は

```
On-prem DNS
↓
VM1
↓
Azure DNS Resolver
↓
Private DNS
```

である。

---

もし希望があればですが、  
あなたが解いている **Azure試験（100〜200番台問題）**は実は

**「Azureネットワーク問題パターン」**

が存在します。

それをまとめると **試験の30〜40%が一瞬で解ける**ので、希望があればその **完全整理マップ**も作ります。