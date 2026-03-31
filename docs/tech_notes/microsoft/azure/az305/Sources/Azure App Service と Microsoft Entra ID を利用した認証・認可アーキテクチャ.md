[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure App Service]]
## Azure App Service と Microsoft Entra ID を利用した認証・認可アーキテクチャ

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Conditional Access]]
### ― App Service Authentication（Easy Auth）と Conditional Access によるセキュアな Web アプリ設計 ―

---

# 1 背景と問題のコンテキスト

クラウド環境で Web アプリケーションを公開する場合、ユーザー認証とアクセス制御はアプリケーションのセキュリティ設計において最も重要な要素の一つである。特に企業向けアプリケーションでは、ユーザーが組織の ID 管理システムで認証され、所属グループやロールに応じてアクセス権限が制御される仕組みが必要になる。

今回のシナリオでは、Azure App Service 上に Web アプリケーションを展開しており、ユーザー認証には **Microsoft Entra ID（旧 Azure Active Directory）** を利用する必要がある。また、単純なログインだけでなく、次のような高度なセキュリティ要件も満たす必要がある。

- ユーザーは Microsoft Entra ID を通じて認証されること
    
- アクセス許可は **グループメンバーシップ** に基づいて判断されること
    
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Multi-Factor Authentication (MFA)]]
- 信頼できないネットワークからアクセスする場合は **多要素認証（MFA）** を要求すること
    
- 条件付きアクセスポリシー（Conditional Access）を適用できること
    

これらの要件を満たすために推奨される設定は **App Service Authentication（Easy Auth）と Microsoft Entra ID の統合**である。

---

# 2 Azure App Service Authentication（Easy Auth）

Azure App Service には **App Service Authentication** という機能があり、これは「Easy Auth」と呼ばれることもある。この機能は、アプリケーションコードを変更することなく、外部の ID プロバイダーを使用した認証を簡単に有効化できる仕組みである。

Easy Auth を使用すると、App Service のフロントエンドで認証処理が行われ、アプリケーションは認証済みユーザーの情報を HTTP ヘッダーやトークンとして受け取ることができる。

基本的な構造は次の通りである。

```text
User
  │
  ▼
Azure App Service Authentication
  │
  ▼
Microsoft Entra ID
  │
  ▼
Web Application
```

この仕組みにより、開発者はログイン機能やトークン検証ロジックを自分で実装する必要がなくなる。

---

# 3 Microsoft Entra ID によるユーザー認証

Microsoft Entra ID は Azure のクラウドベースの ID 管理サービスであり、ユーザー認証、シングルサインオン、条件付きアクセスなどを提供する。

App Service と Entra ID を統合すると、ユーザーは次のようなフローで認証される。

```text
User Browser
     │
     ▼
Azure App Service
     │
     ▼
Redirect to Entra ID
     │
     ▼
User Login
     │
     ▼
Token issued (JWT)
     │
     ▼
App Service receives token
```

ユーザーがログインすると、Entra ID は **JWT（JSON Web Token）** を発行する。このトークンにはユーザー情報やグループ情報が含まれている。

---

# 4 グループベースの認可

今回の要件では、ユーザーがどの Azure AD グループに所属しているかに基づいてアクセスを制御する必要がある。

Microsoft Entra ID は JWT トークン内に **グループクレーム（group claims）** を含めることができる。

トークンの例

```json
{
  "name": "user@example.com",
  "groups": [
    "Admins",
    "Developers"
  ]
}
```

アプリケーションはこの情報を利用して、次のような認可ロジックを実装できる。

```text
If user in Admins group → Full access
If user in Developers group → Limited access
```

これにより、アプリケーション側でユーザー管理を行う必要がなくなる。

---

# 5 Conditional Access（条件付きアクセス）

Microsoft Entra ID の **Conditional Access** は、ユーザーのアクセス条件に応じて追加のセキュリティ要件を適用する機能である。

条件付きアクセスでは、次のようなポリシーを設定できる。

- 信頼できないネットワークからのアクセス時に MFA を要求
    
- 特定のデバイスのみアクセス許可
    
- 特定の地域からのアクセスを制限
    
- リスクの高いログインをブロック
    

今回の要件では

**信頼できないネットワークからアクセスする場合 MFA を要求**

する必要があるため、Entra ID の Conditional Access を利用する。

認証フローは次のようになる。

```text
User
  │
  ▼
Login request
  │
  ▼
Entra ID Conditional Access
  │
  ├ Trusted network → allow
  └ Untrusted network → require MFA
```

この処理はアプリケーションではなく **Entra ID 側で実行される**。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
# 6 Azure API Management と JWT 検証

API ベースのアプリケーションでは、API Gateway として **Azure API Management（APIM）** を利用することがある。APIM では JWT トークンの検証ポリシーを使用して、API アクセスを制御できる。

構造は次のようになる。

```text
Client
  │
  ▼
Azure API Management
  │
  ▼
JWT Validation
  │
  ▼
Backend API
```

APIM はトークンの有効性を検証し、認証されていないリクエストをブロックすることができる。

ただし、今回の問題の要件は **Web アプリケーションの認証**であり、APIM は必須ではない。API を公開する場合には追加のセキュリティ層として有用である。

---

# 7 他の選択肢が不適な理由

### カスタム認証の実装

アプリケーションコードに独自の認証ロジックを実装することも可能である。しかし、この方法では次の問題が発生する。

- トークン検証ロジックの実装が必要
    
- セッション管理が複雑
    
- セキュリティリスクが増加
    
- Conditional Access を適用しにくい
    

Microsoft Entra ID を使用する場合、これらの機能はすでに提供されているため、カスタム認証を実装するメリットは少ない。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Front Door]]
### Azure Front Door + WAF

Azure Front Door はグローバルロードバランサーおよびセキュリティゲートウェイとして使用されるサービスであり、WAF（Web Application Firewall）によってアプリケーションを攻撃から保護することができる。

しかし、Front Door は次の機能を提供しない。

- ユーザー認証
    
- Azure AD グループベースの認可
    
- MFA enforcement
    

そのため、Front Door はセキュリティ層として有用ではあるが、今回の認証要件を満たすものではない。

---

# 8 推奨アーキテクチャ

最終的なアーキテクチャは次のようになる。

```text
User
  │
  ▼
Azure App Service Authentication
  │
  ▼
Microsoft Entra ID
  │
  ├ User authentication
  ├ Group membership
  └ Conditional Access (MFA)
  │
  ▼
Web Application
```

必要に応じて API Management を追加すると次のようになる。

```text
User
  │
  ▼
Azure App Service
  │
  ▼
Azure API Management
  │
  ▼
Backend APIs
```

---

# 9 まとめ

今回の要件は次の通りである。

- Azure App Service Web アプリケーション
    
- Microsoft Entra ID によるユーザー認証
    
- グループベースのアクセス制御
    
- Conditional Access（MFA）対応
    

これらを満たす最適な設定は

**App Service Authentication を Microsoft Entra ID と統合すること**

である。

この構成により

- Entra ID による安全な認証
    
- グループベース認可
    
- Conditional Access による MFA
    
- アプリケーションコードの簡素化
    

を同時に実現できる。