[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Kubernetes Service (AKS)]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# AKS から Azure SQL Database に安全に接続する認証方式

## 1. 背景（シナリオ）

クラウドネイティブアプリケーションでは、コンテナ化されたワークロードがデータベースなどの外部サービスへアクセスする必要がある。今回のシナリオでは、Azure Kubernetes Service（AKS）クラスターで動作するアプリケーションが **Azure SQL Database** に接続する必要がある。ただし重要な制約として、**コード内にユーザー名やパスワードなどの認証情報を保存してはいけない**という要件がある。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Microsoft Entra ID]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Managed Identity]]
この要件は、クラウドセキュリティのベストプラクティスに基づくものである。コンテナイメージやアプリケーションコードに認証情報を直接埋め込むと、コードリポジトリやイメージレジストリから情報が漏洩するリスクがある。そのため Azure では、認証情報を管理する代わりに **Azure AD を利用したトークンベース認証** や **Managed Identity（管理された ID）** を利用する設計が推奨されている。

この問題は、AKS 上のアプリケーションが Azure SQL Database に接続する際に、**認証情報をコードに保存せずに安全にアクセスする方法**を理解しているかどうかを問うものである。

---

## 2. 要件整理

問題の要件を整理すると、次の三つのポイントが重要になる。まず、アプリケーションは AKS 上のコンテナとして実行されているため、Kubernetes ポッド単位で Azure リソースへアクセスする必要がある。次に、Azure SQL Database に対して安全に認証する必要がある。そして最も重要な点として、コードや設定ファイルにユーザー名やパスワードといった認証情報を保存してはいけない。

この要件を満たすためには、アプリケーションが実行時に Azure AD からトークンを取得し、そのトークンを使って Azure SQL Database に接続する仕組みが必要になる。つまり、静的な資格情報ではなく **動的なトークンベース認証**を利用することが求められている。

---

## 3. Azure AD Managed Identity の仕組み

Azure には **Managed Identity（管理された ID）** という仕組みがあり、Azure リソースに対して自動的に Azure AD の ID を割り当てることができる。この ID を利用することで、アプリケーションはパスワードやシークレットを管理することなく、Azure AD からトークンを取得して他の Azure サービスにアクセスできる。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Workload Identity]]
AKS の場合、この仕組みは **Pod Managed Identity**（現在は Azure AD Workload Identity へ進化）として利用できる。これにより、Kubernetes のポッドに Azure AD の ID を関連付けることができ、そのポッドで実行されるアプリケーションは Azure AD からアクセストークンを取得して Azure SQL Database に認証できる。

この方式では、アプリケーションは Azure AD から短期間有効なトークンを取得し、そのトークンを使ってデータベースに接続する。パスワードを保存する必要がなく、トークンは一定時間で失効するためセキュリティリスクも低減される。

---

## 4. 技術的な構成

AKS から Azure SQL Database に接続する際の構成を整理すると、次のような流れになる。

```text
AKS Pod
   │
   │ Managed Identity
   ▼
Azure AD
   │
   │ Access Token
   ▼
Azure SQL Database
```

ポッドには Azure AD の管理された ID が関連付けられている。アプリケーションはこの ID を利用して Azure AD からアクセストークンを取得する。そのトークンを利用して Azure SQL Database に対して認証を行う。

この方式では、接続文字列にパスワードを含める必要がなく、Azure AD による認証とアクセス制御を利用することができる。

---

## 5. 設計判断（なぜこの方式か）

この問題の設計判断のポイントは、**認証情報をコードに保存しないこと**である。SQL Server 認証のようにユーザー名とパスワードを使う方式では、これらの資格情報をどこかに保存する必要があり、要件を満たすことができない。

一方で Azure AD の Managed Identity を使用すると、アプリケーションは実行時に Azure AD からトークンを取得するだけでよく、パスワードやシークレットを保持する必要がない。また、アクセス制御は Azure AD と Azure SQL Database 側で集中管理できるため、セキュリティ管理や監査の面でも優れている。

そのため、AKS から Azure SQL Database へ安全にアクセスするための最も適切な方法は **Azure AD Pod Managed Identity** を利用することである。

---

## 6. 他の選択肢が誤りな理由

SQL Server 認証は、ユーザー名とパスワードを使用する従来型の認証方式である。この方式では接続文字列や設定ファイルに資格情報を保存する必要があるため、コードに認証情報を保存しないという要件に違反する。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
共有アクセス署名（SAS）は Azure Storage のリソースに対するアクセス制御に使用される仕組みであり、Azure SQL Database の認証には使用されない。そのため、このシナリオでは適用できない。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Key Vault]]
Azure Key Vault にシークレットを保存する方法は、コードに直接資格情報を書き込むよりは安全であるが、実行時に Key Vault からシークレットを取得する処理が必要になる。つまり、最終的にはアプリケーションがパスワードを扱う必要があり、完全に資格情報を排除することはできない。Managed Identity を使用する方法の方が、より安全でシンプルな設計になる。

---

## 7. 最終回答

**B. Azure AD ポッド管理型 ID**

---

## 8. まとめ

AKS から Azure SQL Database に安全にアクセスする場合、パスワードを管理する方式ではなく Azure AD のトークンベース認証を利用することが推奨される。Managed Identity を使用すると、アプリケーションは Azure AD から短期間のアクセストークンを取得してデータベースに接続できるため、コードや設定ファイルに資格情報を保存する必要がない。

この仕組みは Azure のゼロトラストセキュリティモデルとも一致しており、AKS から Azure リソースへ安全にアクセスする際の標準的な方法となっている。Azure の設計問題では、**「認証情報をコードに保存しない」要件がある場合は Managed Identity を選択する**というパターンを覚えておくと判断しやすい。