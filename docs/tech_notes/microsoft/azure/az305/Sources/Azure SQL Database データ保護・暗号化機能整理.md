[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
# Azure SQL Database データ保護・暗号化機能整理

Azure SQL Database では、機密データを保護するために複数のセキュリティ機能が提供されています。  
これらの機能は **保護する対象や目的が異なる**ため、要件に応じて適切な機能を選択する必要があります。

特に次のようなデータがある場合に利用されます。

- クレジットカード番号
- 個人情報（PII）
- 医療データ
- 財務データ

Azure SQL のセキュリティは主に次の観点で分類できます。

- 保存データの保護
- クエリ結果の保護
- アクセス制御
- アプリケーション側暗号化

主な機能は以下です。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Always Encrypted]]
- Always Encrypted  
- Dynamic Data Masking  
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Transparent Data Encryption (TDE)]]
- Transparent Data Encryption (TDE)  
- Row-Level Security  
- Column-Level Encryption  

---

# Always Encrypted（常に暗号化）

Always Encrypted は、**データベース管理者や開発者が機密データの平文を見ることができないようにする暗号化機能**です。

この仕組みでは、データは **クライアントアプリケーション側で暗号化されてからデータベースに送信されます**。  
そのため、データベースには常に暗号化されたデータのみが保存されます。

データの復号はアプリケーション側で行われるため、暗号鍵を持たないユーザーは平文データを見ることができません。

この方式により、次のような構成になります。

```

アプリケーション  
↓ 暗号化  
Azure SQL Database  
↓ 暗号化データ保存

```

開発者が SQL クエリを実行しても、取得できるのは暗号化された値のみになります。

```

SELECT CreditCardNumber FROM Customers

```

結果

```

0xA8F903A19D....

```

この機能は次のような要件に適しています。

- DBA や開発者から機密データを保護したい
- クレジットカード番号などの機密情報を保護したい
- アプリケーションは完全なデータを利用する必要がある

---

# Dynamic Data Masking（動的データマスキング）

Dynamic Data Masking は、**クエリ結果の表示をマスクする機能**です。

データベースに保存されているデータ自体は変更されません。  
ユーザーの権限に応じて、結果の表示だけをマスクします。

例

```

4111-1111-1111-1111

```

マスク表示

```

XXXX-XXXX-XXXX-1111

```

この機能は次の用途に適しています。

- 開発環境でのデータ閲覧
- カスタマーサポートなどの閲覧制御
- アプリケーションログの表示制御

ただし、十分な権限を持つユーザーはマスクを解除できるため、  
**完全なセキュリティ対策ではありません。**

---

# Transparent Data Encryption（TDE）

Transparent Data Encryption は、**データベースファイルを保存時に暗号化する機能**です。

この機能は主に次のデータを保護します。

- データベースファイル
- バックアップファイル
- トランザクションログ

構造は次の通りです。

```

Azure SQL Database  
↓  
ディスク保存  
↓  
暗号化

```

TDE は **保存時暗号化（Encryption at Rest）**を提供します。

ただし、データベースにアクセスできるユーザーが SQL クエリを実行すると、  
平文データを見ることができます。

そのため、次の目的には適していません。

- 開発者からデータを隠す
- クエリ結果の制御

---

# Row-Level Security（行レベルセキュリティ）

Row-Level Security は、**ユーザーごとにアクセスできる行を制御する機能**です。

ユーザーの属性やログイン情報に応じて、表示されるデータを制御できます。

例

営業データテーブル

```

CustomerID | SalesRep

```

アクセス制御

```

営業A → 自分の顧客のみ  
営業B → 自分の顧客のみ

```

この機能は次の用途で使用されます。

- マルチテナントアプリケーション
- 部門別データ管理
- セキュリティポリシー制御

ただし Row-Level Security は **列の値を隠す機能ではありません。**

---

# Column-Level Encryption（列レベル暗号化）

Column-Level Encryption は、**特定のカラムを暗号化する機能**です。

この方法では、アプリケーション側で暗号化処理を実装する必要があります。

```

アプリケーション  
↓  
暗号化  
↓  
SQL Database

```

この方法は柔軟性がありますが、次の問題があります。

- 鍵管理が複雑
- 実装が難しい
- アプリケーション変更が必要

そのため、Azure SQL Database では  
**Always Encrypted の利用が推奨されるケースが多いです。**

---

# Azure SQL セキュリティ機能の役割比較

|機能|主な目的|
|---|---|
Always Encrypted | DBA や開発者から機密データを保護 |
Dynamic Data Masking | クエリ結果の表示をマスク |
Transparent Data Encryption | 保存データの暗号化 |
Row-Level Security | 行レベルのアクセス制御 |
Column-Level Encryption | 特定カラムの暗号化 |

---

# 試験での判断ポイント

Azure 試験では、次の判断が頻繁に出題されます。

機密データを **DB管理者や開発者からも保護する必要がある**

→ Always Encrypted

クエリ結果を **一部マスクして表示したい**

→ Dynamic Data Masking

データベースファイルやバックアップを **保存時に暗号化したい**

→ Transparent Data Encryption

ユーザーごとに **見える行を制御したい**

→ Row-Level Security

---

# まとめ

Azure SQL Database には、複数のデータ保護機能があります。  
それぞれの機能は **保護する対象と目的が異なる**ため、要件に応じて適切に使い分ける必要があります。

特に機密情報を扱うシステムでは、  
Always Encrypted を使用することで **アプリケーションのみが平文データを扱える構成**を実現できます。

このアプローチにより、開発者やデータベース管理者による不正なデータ閲覧を防ぎながら、アプリケーションの正常な動作を維持することが可能になります。
