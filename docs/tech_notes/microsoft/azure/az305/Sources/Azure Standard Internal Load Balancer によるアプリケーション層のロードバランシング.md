あります。むしろ **Azureロードバランシングを理解する一番重要な軸が「内部（Internal）か外部（External）か」** です。  
前の説明に追加して整理すると、理解がかなりクリアになります。

---

# 1 Azureロードバランシングは2つの視点で整理する

Azureのロードバランサーは次の **2軸** で分類すると理解しやすいです。

**① OSIレイヤ**

|レイヤ|意味|
|---|---|
|L4|TCP / UDP|
|L7|HTTP / HTTPS|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure DNS]]
|DNS|名前解決|

---

**② 公開範囲**

|種類|意味|
|---|---|
|External|インターネット公開|
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
|Internal|VNet内部|

---

この2つを組み合わせると整理できます。

---

# 2 Azureロードバランサー全体マップ

```text
                Global
                  │
            Azure Front Door
                  │
        ┌─────────┴─────────┐
        │                   │
   Application Gateway   Load Balancer
         (L7)                (L4)
        │                   │
        │                   │
   External / Internal   External / Internal
```

そしてもう1つ特殊なものがあります。

```text
Gateway Load Balancer
(NVA用)
```

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Load Balancer]]
# 3 Azure Load Balancer（Internal / External）

Azure Load Balancer は **L4ロードバランサー**です。

TCP / UDP レベルで動作します。

## External Load Balancer

インターネット公開。

```text
Internet
   │
   ▼
Public Load Balancer
   │
   ├ VM1
   ├ VM2
   └ VM3
```

例

- Web API
    
- Public service
    

---

## Internal Load Balancer

VNet 内通信。

```text
Web Tier
   │
   ▼
Internal Load Balancer
   │
   ├ App VM1
   └ App VM2
```

例

- Web → App
    
- Microservices
    

Azure試験でよく出る構成

```text
Internet
   │
Application Gateway
   │
Web Tier
   │
Internal Load Balancer
   │
App Tier
```

---

# 4 Application Gateway（外部 / 内部）

Application Gateway も **Public / Private 両方作れます**。

## External Application Gateway

```text
Internet
   │
   ▼
Application Gateway
   │
   ├ Web1
   └ Web2
```

用途

- Webアプリ
    
- API
    
- WAF
    

---

## Internal Application Gateway

```text
Internal clients
   │
   ▼
Internal App Gateway
   │
   ├ API1
   └ API2
```

用途

- 内部API
    
- BFF
    

---

# 5 Front Door（外部のみ）

Front Door は **グローバル外部サービス**です。

```text
Users (Global)
      │
      ▼
Azure Front Door
      │
      ├ Region1
      └ Region2
```

内部には使いません。

---

# 6 Traffic Manager（DNS）

Traffic Manager は DNS です。

```text
DNS
 │
 ▼
Traffic Manager
 │
 ├ US
 └ EU
```

内部ロードバランサーではありません。

---

# 7 Gateway Load Balancer（NVA）

これは **セキュリティアプライアンス専用**。

```text
Internet
   │
   ▼
Load Balancer
   │
   ▼
Gateway Load Balancer
   │
   ├ Firewall
   └ Firewall
```

用途

- Firewall
    
- IDS
    
- IPS
    

---

# 8 試験での判断方法

AZ-305では次の順番で判断すると解けます。

### Step1

HTTPか？

YES → L7  
NO → L4

---

### Step2

インターネット公開か？

YES → Public  
NO → Internal

---

### Step3

グローバルか？

YES → Front Door / Traffic Manager

---

### Step4

NVAか？

YES → Gateway Load Balancer

---

# 9 まとめ（超重要）

Azureロードバランシングは

**レイヤ + 内部外部**

で整理すると理解できます。

|サービス|レイヤ|内部/外部|
|---|---|---|
|Load Balancer|L4|両方|
|Application Gateway|L7|両方|
|Front Door|L7|外部|
|Traffic Manager|DNS|外部|
|Gateway LB|L4|NVA|

---

もしよければですが、  
Azure Architect 試験でかなり重要な

**「Azureネットワーク構成の完全図」**

（LoadBalancer / Firewall / Gateway / FrontDoor全部入り）

を描きます。  
これを理解すると **AZ-305ネットワーク問題がかなり楽になります。**