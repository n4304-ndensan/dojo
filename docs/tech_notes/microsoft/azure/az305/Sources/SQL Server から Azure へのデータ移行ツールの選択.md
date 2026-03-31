# SQL Server から Azure へのデータ移行ツールの選択

## 1. 背景（シナリオ）

企業がクラウド移行を進める際、オンプレミスのデータベースを Azure に移行するケースは非常に多い。特に既存の SQL Server 環境をクラウドに移行する場合、データベースのサイズ、移行対象、移行先のサービスなどに応じて適切なツールを選択する必要がある。Azure にはさまざまなデータ移行ツールが存在しており、それぞれ用途や対象となるデータストアが異なるため、適切なツール選択が重要になる。

今回のシナリオでは、オンプレミス環境に SQL Server 2012 のデータベースが存在しており、それを Azure に移行する必要がある。また別の要件として、SQL Server 2014 にある **単一のテーブル**を Azure に移行する必要がある。ここで重要なのは、移行対象が「データベース全体」と「単一のテーブル」で分かれている点である。データベース全体を移行する場合と、特定のテーブルのみを移行する場合では、使用するツールや方法が異なることがある。

この問題は、Azure のデータ移行ツールの役割を理解し、どのツールがどの移行シナリオに適しているかを判断できるかを問う内容である。

---

## 2. 要件整理

問題の条件を整理すると、移行対象は二つ存在している。一つ目は SQL Server 2012 のデータベース全体であり、これを Azure に移行する必要がある。二つ目は SQL Server 2014 にある単一のテーブルであり、こちらも Azure に移行する必要があるが、対象はデータベース全体ではなく特定のテーブルだけである。

このように移行粒度が異なる場合、それぞれの目的に合ったツールを選択する必要がある。データベース全体の移行では、スキーマの互換性チェックや移行支援機能を持つツールが適している。一方で単一テーブルの移行では、特定のデータセットだけを取り込めるツールが必要になる。

---

## 3. Data Migration Assistant（DMA）

SQL Server を Azure に移行する際の代表的なツールの一つが **Data Migration Assistant（DMA）** である。DMA は Microsoft が提供する移行ツールであり、オンプレミスの SQL Server データベースを Azure の SQL サービスへ移行するための評価と移行をサポートする。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure SQL Database]]
DMA の大きな特徴は、移行前に互換性評価を行える点である。SQL Server のバージョンによっては Azure SQL Database でサポートされない機能が存在するため、DMA は移行前にスキーマを分析し、互換性の問題や修正が必要な箇所を提示する。さらに、スキーマとデータの両方を移行する機能も備えているため、データベース全体を Azure に移行する場合には非常に有効なツールである。

今回の問題では SQL Server 2012 のデータベース全体を Azure に移行する必要があるため、このシナリオには DMA が適している。

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Cosmos DB]]
## 4. Azure Cosmos DB データ移行ツール

Azure Cosmos DB は、分散型の NoSQL データベースであり、リレーショナルデータベースとは異なるデータモデルを採用している。そのため、SQL Server のデータを Cosmos DB に移行する場合には、テーブルデータをドキュメントモデルへ変換する処理が必要になる。

この用途のために提供されているのが **Azure Cosmos DB データ移行ツール** である。このツールは、SQL Server や JSON ファイル、CSV ファイルなどのさまざまなソースからデータを取り込み、Cosmos DB のコレクションへインポートすることができる。

このツールの特徴は、移行対象を柔軟に選択できることである。例えば SQL Server の特定のテーブルだけを選択して Cosmos DB に取り込むことができる。そのため、データベース全体ではなく **単一テーブルのみを移行するシナリオ** に適している。

今回の問題では SQL Server 2014 から単一テーブルを Azure に移行する必要があるため、この用途には Cosmos DB データ移行ツールが適している。

---

## 5. 技術的な仕組み

今回の移行シナリオを構成図として表すと、次のようになる。

```id="migration-arch"
On-premises SQL Server 2012
          │
          │ Data Migration Assistant
          ▼
      Azure SQL

On-premises SQL Server 2014
          │
          │ Cosmos DB Data Migration Tool
          ▼
      Azure Cosmos DB
```

SQL Server 2012 のデータベース全体は DMA を使用して Azure SQL へ移行される。一方で SQL Server 2014 の特定テーブルは Cosmos DB データ移行ツールを使用して Cosmos DB に取り込まれる。

このように、移行対象の種類や粒度によってツールを使い分けることが Azure のデータ移行設計では重要になる。

---

## 6. 他の選択肢が誤りな理由

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
AzCopy は Azure Storage との間でデータをコピーするためのツールであり、Blob Storage や File Storage のデータ転送には適しているが、SQL Server のデータベースを直接移行する用途には使用されない。そのため、SQL Server 2012 のデータベース移行に AzCopy を使用する選択肢は適切ではない。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
データ管理ゲートウェイ（Data Management Gateway）は、かつて Azure Data Factory でオンプレミスデータソースへ接続するために使用されていたコンポーネントであり、現在では統合ランタイム（Integration Runtime）に置き換えられている。このコンポーネントはデータアクセスのためのゲートウェイであり、データベースの移行ツールとして使用するものではない。

また、Cosmos DB データ移行ツールと AzCopy を組み合わせる選択肢も適切ではない。Cosmos DB データ移行ツールは Cosmos DB へのデータ取り込みに特化しており、AzCopy はストレージ転送ツールであるため、SQL Server の移行シナリオとしては一貫性がない。

---

## 7. 最終回答

**A  
SQL Server 2012：Data Migration Assistant  
SQL Server 2014 の単一テーブル：Azure Cosmos DB データ移行ツール**

---

## 8. まとめ

Azure へのデータ移行では、移行対象の種類や粒度によって使用するツールが異なる。SQL Server のデータベース全体を Azure に移行する場合には、互換性評価と移行機能を備えた Data Migration Assistant が適している。一方で SQL Server の特定のテーブルを Azure Cosmos DB に移行する場合には、Cosmos DB データ移行ツールが適している。

Azure の移行ツールには多くの種類があり、それぞれ目的が異なるため、ツールの役割を理解して適切に選択することが重要である。特に Azure 試験では、DMA、AzCopy、Cosmos DB データ移行ツール、Azure Data Factory などの役割の違いを理解しておくことが重要になる。