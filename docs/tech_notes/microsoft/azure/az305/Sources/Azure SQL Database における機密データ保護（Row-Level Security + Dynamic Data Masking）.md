---
分類: Databases
tags:
  - cloud/azure
  - cloud/azure/sql-database
  - cloud/security
  - cloud/security/pii
  - cloud/security/data-masking
  - cloud/security/row-level-security
  - exam/azure
---

# Azure SQL Database における機密データ保護（Row-Level Security + Dynamic Data Masking）

## 1. 背景（シナリオ）

組織は PII（個人識別情報）を含む機密データを Azure SQL Database に保存しています。  
このデータベースではユーザーがクエリを実行する必要がありますが、すべてのユーザーが機密情報を閲覧できるわけではありません。

そのため次の要件を満たすセキュリティ設計が必要です。

- ユーザーはデータベースをクエリできる  
- 一部ユーザーのみPIIを閲覧できる  
- 未承認ユーザーは機密情報を見られない  
- 厳格なアクセス制御が必要  

このようなケースでは「データの可視性」を制御する仕組みが重要になります。

---

## 2. 要件整理

この問題の重要ポイントは次の2つです。

まず、ユーザーごとにアクセスできるデータを制御する必要があります。  
次に、ユーザーはクエリ可能である必要がありますが、機密データは見えないようにする必要があります。

つまり

クエリは可能  
機密データは非表示  

という仕組みが必要になります。

---

## 3. Row-Level Security（RLS）

Row-Level Security は、ユーザーごとにアクセスできる「行」を制御する機能です。

同じテーブルでもユーザーによって返されるデータが変わります。

例えば社員テーブルがあるとします。

| Name | Department |
|-----|-----|
| Alice | HR |
| Bob | Sales |

HRユーザーがクエリを実行すると、HRのデータのみ表示されます。  
Salesユーザーの場合はSalesのデータのみ表示されます。

このようにRLSはユーザー属性に基づいて行レベルのアクセス制御を実現します。

---

## 4. Dynamic Data Masking（DDM）

Dynamic Data Masking は、機密データをマスク表示する機能です。

データ自体は変更されませんが、権限のないユーザーがクエリするとマスクされた値が表示されます。

例えばメールアドレスの場合

実際の値

john@example.com

マスク表示

j***@example.com

クレジットカード番号の場合

実際の値

1234-5678-9999-1111

表示

XXXX-XXXX-XXXX-1111

つまりデータは存在していても、権限のないユーザーには実際の値が見えません。

---

## 5. この問題の最適解

この問題では次の2つの機能が必要です。

まず、ユーザーごとにアクセスできるデータを制御する必要があります。  
これは Row-Level Security を使用します。

次に、ユーザーがクエリしても機密データを見えなくする必要があります。  
これは Dynamic Data Masking を使用します。

この2つを組み合わせることで

ユーザーはクエリできる  
しかし機密データは閲覧できない  

というセキュリティ要件を満たすことができます。

---

## 6. 他の選択肢が誤りな理由

### B  
Always Encrypted + Azure AD 認証

Always Encrypted は強力な暗号化機能ですが、クライアント側で復号が必要になり、クエリ機能に制限が生じる場合があります。  
この問題の「ユーザーがクエリ可能」という要件には最適ではありません。

### C  
TDE + Backup Encryption

TDEは保存時暗号化を提供する機能です。  
しかしユーザーの閲覧制御は行えません。

### D  
SQL Audit + Threat Detection

これらは監査と脅威検出の機能です。  
データアクセス制御の機能ではありません。

---

## 7. 正解

A  
Row-Level Security と Dynamic Data Masking

---

## 8. まとめ

Azure SQL Database で機密データを保護する場合、以下の組み合わせがよく使用されます。

Row-Level Security  
Dynamic Data Masking

この組み合わせにより

- 行レベルアクセス制御  
- 機密データのマスク表示  

を同時に実現できます。