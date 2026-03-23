[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Network (VNet)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure API Management]]
# Azure API Management の外部モードと同一 VNet 内バックエンド接続の判断

## 1. 背景（シナリオ）

この問題は、Azure API Management（APIM）を仮想ネットワークに接続したときに、外部利用者からのアクセス経路と、APIM から内部バックエンドへの到達性がどうなるかを理解しているかを問うものである。設計の論点は三つある。第一に、APIM を **外部モード** でデプロイした場合、インターネット上のパートナーがその API を利用できるかどうか。第二に、APIM が同じ VNet 内の別サブネットに存在する VM1 にアクセスできるかどうか。第三に、それらを実現するために VPN ゲートウェイが必要かどうか、という点である。

問題文では、Web API をホストする APIM が VNet に参加しており、VM1 も同じ VNet 内に存在している。ただし、APIM と VM1 は別サブネットに配置されている。ここで重要なのは、**同じ VNet 内であればサブネットが異なっていても、適切な NSG やルートで遮断されていない限り、プライベート通信が可能**であること、そして **外部モードの APIM は外部クライアント向けの公開エンドポイントを持つ**という点である。Microsoft Learn では、外部モードはパブリック インターネットから API Management エンドポイントにアクセスできる構成として説明されている。さらに、VNet に接続された APIM は、そのネットワーク内のバックエンド サービスへアクセスできる。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

---

## 2. 要件整理

この問題を解くには、三つの要件をそれぞれ独立して考える必要がある。

まず、パートナーはインターネット経由で API を利用できる必要がある。これは APIM 側の **受信経路** の話であり、外部モードか内部モードかが直接関係する。外部モードであれば、APIM のゲートウェイはパブリックに公開されるため、パートナーはインターネット越しに API を呼び出せる。Microsoft Learn でも、内部モードは VNet 内だけからアクセス可能であり、対比として外部モードはパブリック インターネットからアクセス可能とされている。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

次に、APIM が VM1 にアクセスできる必要がある。これは APIM 側の **送信経路** の話であり、同じ VNet 内でバックエンドへ接続できるかどうかが問われている。APIM が VNet にインジェクトされており、VM1 も同じ VNet にある以上、サブネットが異なっていても、通常はプライベート IP 経由で到達できる。Microsoft Learn は、VNet に接続した APIM を使ってネットワーク内のバックエンド サービスへアクセスできると説明している。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

最後に、VPN ゲートウェイが必要かどうかである。VPN ゲートウェイは、一般に **オンプレミスと Azure 間**、あるいは **VNet 間** の接続で必要になる。同一 VNet 内のサブネット間通信には、通常 VPN ゲートウェイは不要である。したがって、このシナリオで VPN ゲートウェイを必須とする理由はない。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/virtual-network-concepts?utm_source=chatgpt.com "Azure API Management with an Azure virtual network"))

---

## 3. 関連 Azure サービスの説明

Azure API Management は、API を一元公開し、認証、レート制限、変換、監視などを提供する API ゲートウェイサービスである。ネットワーク面では、仮想ネットワークと組み合わせることで、内部バックエンドに安全に接続する構成を取れる。特に VNet インジェクションには **外部モード** と **内部モード** がある。

外部モードでは、APIM は仮想ネットワーク内に配置されつつ、API ゲートウェイはパブリックに到達可能な状態を維持する。つまり、クライアントから見ればインターネット経由で利用できるが、バックエンドに対しては仮想ネットワーク内からプライベートに接続できる。これは、「外部公開したいが、バックエンドは内部に閉じたい」という典型的なパターンに向いている。Microsoft Learn は、外部モードを「パブリック インターネットから API Management エンドポイントにアクセスでき、そのネットワーク内にバックエンド サービスが配置される構成」と説明している。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

一方、内部モードでは APIM のゲートウェイ自体が VNet 内からしかアクセスできない。そのため、インターネット上のパートナーが直接 API を呼び出すことはできない。この違いが、今回の第一問を判断する鍵である。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-internal-vnet?utm_source=chatgpt.com "Deploy Azure API Management instance to internal VNet"))

---

## 4. 技術的な仕組み

この構成を通信経路で整理すると、次のようになる。

```text
Partner (Internet)
      │
      │ HTTPS
      ▼
API Management (External mode, public endpoint)
      │
      │ Private VNet routing
      ▼
VM1 / Backend API (same VNet, different subnet)
```

受信方向では、APIM は外部モードなので、パートナーはインターネットから APIM の公開エンドポイントにアクセスできる。これは「外部モード = 公開ゲートウェイ」という性質による。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

送信方向では、APIM は同一 VNet 内のバックエンドにプライベートに接続できる。サブネットが異なっていても、Azure VNet の基本動作として、サブネット間は同一 VNet の一部であり、明示的に NSG や UDR で遮断していなければ到達可能である。Microsoft Learn も、VNet に接続した APIM が VNet 内のバックエンドへアクセスするシナリオを前提に説明している。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

また、VPN ゲートウェイはこの経路には登場しない。なぜなら通信は「インターネット → APIM の公開エンドポイント」と「APIM → 同一 VNet 内バックエンド」に分かれており、いずれも VNet-to-VNet や On-prem-to-Azure のトンネル接続を必要としていないからである。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/virtual-network-concepts?utm_source=chatgpt.com "Azure API Management with an Azure virtual network"))

---

## 5. 設計判断（なぜこの構成か）

この問題の本質は、「公開入口」と「内部バックエンド接続」を分けて考えられるかどうかにある。APIM の外部モードは、まさにその二層を両立するための構成である。外部のパートナーにはパブリックな API エンドポイントを提供しながら、バックエンドは VNet 内で非公開に保ち、APIM だけがそのバックエンドへ到達する。これは API ゲートウェイの典型的な設計であり、外部公開と内部保護を両立したい場合に自然な選択になる。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

もし APIM が内部モードであれば、パートナーはインターネットから直接 API を利用できない。そのため、問題文が「パートナーはインターネット経由で API を利用できますか」と聞いている時点で、外部モードであることが非常に重要になる。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-internal-vnet?utm_source=chatgpt.com "Deploy Azure API Management instance to internal VNet"))

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
また、APIM と VM1 が同じ VNet に存在する以上、APIM から VM1 への通信のために VPN ゲートウェイを挟む設計は冗長である。VPN ゲートウェイは異なるネットワークを橋渡しするサービスであり、同一 VNet 内通信には本来不要である。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/advisor/advisor-reference-reliability-recommendations?utm_source=chatgpt.com "Reliability recommendations - Azure Advisor"))

---

## 6. 他の選択肢が誤りな理由

選択肢 B は、「パートナーは利用できる」が「APIM は VM1 にアクセスできない」「VPN ゲートウェイは必要」としている。しかし、同じ VNet 内の別サブネットであれば、通常 APIM は VM1 に到達できるため、この前提が誤っている。また、同一 VNet 内通信に VPN ゲートウェイは不要である。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

選択肢 C は、「パートナーは利用できない」としている点が外部モードの理解に反している。外部モードはパブリック インターネットから APIM エンドポイントへアクセス可能な構成であるため、この部分が誤りである。さらに、やはり VPN ゲートウェイ必須という判断も誤りである。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

選択肢 D は、三つとも否定しているが、外部モードの公開性と、同一 VNet 内通信の到達性の両方に反しているため不正解である。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

---

## 7. 最終回答

**A. はい、はい、いいえ**

つまり、パートナーはインターネット経由で API を利用でき、APIM は VM1 にアクセスでき、VPN ゲートウェイは不要である。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))

---

## 8. まとめ

この問題で覚えるべきことは二つある。第一に、**APIM の外部モードは公開入口を持つ**ため、インターネット上のパートナーが API を利用できるという点である。第二に、**同じ VNet 内にあるバックエンドには APIM からプライベートに接続できる**ため、別サブネットであっても通常は VM1 に到達できるという点である。したがって、この構成に VPN ゲートウェイは必要ない。Microsoft Learn でも、外部モードはパブリックアクセスを提供しつつ、VNet 内バックエンドへのアクセスを可能にする構成として説明されている。([Microsoft Learn](https://learn.microsoft.com/en-us/azure/api-management/api-management-using-with-vnet?utm_source=chatgpt.com "Deploy Azure API Management Instance to External Virtual ..."))