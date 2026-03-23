# Azure 仮想マシンをインターネットから安全に管理する設計

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Bastion]]
（Azure Bastion + Conditional Access + Azure MFA）

---

# 1 背景

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
企業では Azure 上の仮想マシン（VM）を運用しており、インターネット経由でこれらの VM を管理する必要がある。  
一般的に VM 管理には次のプロトコルが使用される。

|OS|管理プロトコル|
|---|---|
|Windows VM|RDP (Remote Desktop Protocol)|
|Linux VM|SSH|

通常これらは以下のポートを使用する。

|プロトコル|ポート|
|---|---|
|RDP|TCP 3389|
|SSH|TCP 22|

しかしこれらのポートをインターネットに直接公開すると、次のようなセキュリティリスクが発生する。

- ブルートフォース攻撃
    
- パスワードスプレー攻撃
    
- ボットによるスキャン
    
- ゼロデイ攻撃
    

そのため Azure では **VM 管理ポートを直接公開しない設計**が推奨されている。

---

# 2 要件整理

問題文から読み取れる重要な要件は次の通りである。

### ① インターネットから VM を管理する

管理者は外部ネットワークから VM にアクセスする必要がある。

---

### ② Azure MFA を使用する

管理アクセスには **多要素認証（MFA）**を強制する必要がある。

---

### ③ TLS 443 を使用する

通信は

```text
HTTPS (TLS 443)
```

で行う必要がある。

---

### ④ RDP / SSH をサポートする

Windows VM と Linux VM の両方を管理できる必要がある。

---

### ⑤ セキュアな管理アクセス

VM 管理ポートを直接公開しない設計が望ましい。

---

# 3 Azure Bastion

この要件を満たす Azure サービスが **Azure Bastion**である。

Azure Bastion は **Azure が提供する管理ジャンプホストサービス**である。

主な特徴

|機能|説明|
|---|---|
|RDP / SSH サポート|Windows / Linux VM 両方対応|
|TLS 443 通信|ブラウザ経由で接続|
|Public IP 不要|VM に公開 IP を付けない|
|Azure Portal から接続|ブラウザベース|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
|VNet 内に配置|安全なネットワーク|

---

# 4 Bastion のアーキテクチャ

通常の VM 管理

```text
Admin
  │
  ▼
Internet
  │
  ▼
VM Public IP
  │
  ▼
RDP / SSH
```

この構成では VM が直接インターネットに公開される。

---

Azure Bastion を利用した場合

```text
Admin
  │
  ▼
HTTPS (443)
  │
  ▼
Azure Portal
  │
  ▼
Azure Bastion
  │
  ▼
Virtual Network
  │
  ▼
VM (Private IP)
```

この構成では VM は **インターネットから直接アクセスできない**。

---

# 5 TLS 443 の利用

Azure Bastion は管理セッションを

```text
HTTPS (TLS 443)
```

経由で提供する。

つまり管理者は

```text
Browser
```

から VM に接続できる。

この仕組みにより

- RDP 3389
    
- SSH 22
    

をインターネットに公開する必要がなくなる。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
# 6 Azure AD 条件付きアクセス（Conditional Access）

次に必要なのは **Azure MFA の強制**である。

Azure ではこれを **Conditional Access（条件付きアクセス）**で実装する。

Conditional Access は

```text
If
  条件
Then
  アクセス制御
```

というポリシーで動作する。

例

```text
User Login
  │
  ▼
Conditional Access
  │
  ▼
MFA Required
```

---

# 7 Azure Windows VM Sign-in

Azure Bastion を使用する場合、次の CA ポリシーを適用できる。

```text
Azure Windows VM Sign-in
```

このポリシーでは

- Azure AD 認証
    
- MFA
    
- デバイス準拠
    
- IP 制限
    

などを設定できる。

---

# 8 MFA を含む認証フロー

実際の接続フローは次のようになる。

```text
Admin
  │
  ▼
Azure Portal
  │
  ▼
Azure AD Authentication
  │
  ▼
Conditional Access
(MFA)
  │
  ▼
Azure Bastion
  │
  ▼
RDP / SSH
  │
  ▼
VM
```

この構成により

- 強力な認証
    
- セキュアな接続
    
- ポート非公開
    

が実現される。

---

# 9 他の選択肢が不適切な理由

## A

Just-in-Time VM Access

JIT は

```text
ポート開放の時間制御
```

を行う機能である。

しかし

- TLS 443 接続
    
- ブラウザ RDP
    
- Bastion ゲートウェイ
    

は提供しない。

---

## C

Azure WAF / Front Door

WAF は

```text
HTTP / HTTPS Webアプリ保護
```

のためのサービスである。

RDP / SSH 管理には使用できない。

---

# 10 推奨アーキテクチャ

最適構成

```text
Administrator
      │
      ▼
Internet
      │
      ▼
Azure AD Authentication
      │
      ▼
Conditional Access (MFA)
      │
      ▼
Azure Bastion
      │
      ▼
Virtual Network
      │
      ▼
VM (Private IP)
```

---

# 11 セキュリティメリット

この設計には多くのメリットがある。

### VM をインターネットに公開しない

RDP / SSH ポートを閉じた状態で運用できる。

---

### MFA 強制

すべての管理者アクセスに MFA を要求できる。

---

### 攻撃面の縮小

公開エンドポイントは

```text
HTTPS 443
```

のみになる。

---

### 中央管理

Azure AD と Bastion により

- 認証
    
- セッション管理
    

を一元化できる。

---

# 12 最終回答

正解

**B**

```text
Access Service : Azure Bastion
Conditional Access : Azure Windows VM Sign-in
```

---

# 13 まとめ

今回の設計のポイント

|要件|解決方法|
|---|---|
|インターネット管理|Azure Bastion|
|MFA 強制|Conditional Access|
|TLS 443|Bastion HTTPS|
|RDP / SSH|Bastion Gateway|

つまり最適な構成は

```text
Azure Bastion
+
Conditional Access
+
Azure MFA
```

である。

これは **Azure VM を安全に管理するための Microsoft 推奨アーキテクチャ**である。