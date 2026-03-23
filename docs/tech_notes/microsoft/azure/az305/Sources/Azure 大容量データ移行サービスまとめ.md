# Azure 大容量データ移行サービスまとめ（物理データ転送）

オンプレミス環境から Azure に大量のデータを移行する場合、ネットワーク帯域が不足していたり回線の信頼性が低いケースがあります。このような場合は、インターネット経由の転送ではなく **物理デバイスを使用してデータを Azure に配送する方法**が利用されます。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Box]]
Azure では主に **Azure Data Box 系サービス**と **Azure Import/Export サービス**が提供されています。これらは大容量データを安全に Azure Storage に移行するための仕組みです。

---

## Azure Data Box Disk（Azure Data Box Disk）

Azure Data Box Disk は、Microsoft が提供する **暗号化された SSD ディスクを使用してデータを転送するサービス**です。Microsoft から専用ディスクが送付され、そのディスクにデータをコピーして返送することで Azure Storage にアップロードされます。

この方法では、インターネット回線を利用せずにデータを物理配送できるため、WAN 回線が遅い環境でも効率的にデータを移行できます。

主な特徴は次の通りです。

- Microsoft が提供する暗号化 SSD を使用
- 数十 TB 規模のデータ移行に適している
- 複数ディスクを使用することでデータ量を拡張可能
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
- Azure Blob Storage などに直接取り込み可能

このサービスは **数十 TB 程度のデータ移行に適した方法**としてよく利用されます。

---

## Azure Import/Export Service（Azure インポート / エクスポート サービス）

Azure Import/Export Service は、ユーザーが用意したディスクを Azure データセンターに送付することでデータを Azure Storage に取り込むサービスです。

ユーザーが自分のハードディスクにデータを書き込み、それを Microsoft データセンターへ配送することで Azure Storage にインポートされます。また、Azure Storage のデータをディスクに書き出して返送する **エクスポート機能**も利用できます。

主な特徴は次の通りです。

- ユーザーが用意したディスクを使用
- 大容量データの物理転送が可能
- Azure Storage へのインポート・エクスポートの両方に対応
- オンプレミス環境からの大量データ移行に利用される

このサービスは **ネットワーク転送が困難な場合の物理データ移行手段**として利用されます。

---

## Azure Data Box（Azure Data Box）

Azure Data Box は、Microsoft が提供する **専用のデータ転送アプライアンス（物理デバイス）**です。オンプレミス環境からデータをコピーして Microsoft に返送することで Azure Storage に取り込まれます。

Data Box Disk よりも大容量のデータ転送に対応しています。

主な特徴は次の通りです。

- 専用ハードウェアデバイス
- 数百 TB 規模のデータ転送
- 高速ローカルネットワークコピー
- Azure Blob Storage や Data Lake へ取り込み可能

---

## Azure Data Box Heavy（Azure Data Box Heavy）

Azure Data Box Heavy は、Azure Data Box の大容量版であり、**ペタバイト規模のデータ移行に対応する物理デバイス**です。

非常に大量のデータを短期間で Azure に移行する必要がある場合に使用されます。

主な特徴は次の通りです。

- 約 1PB 規模のデータ転送
- 大規模データセンター移行向け
- 高速データコピー
- 大規模分析基盤移行などに利用

---

# ネットワークベースのデータ転送（参考）

物理配送ではなくネットワーク経由でデータ転送する場合は次のツールが利用されます。

## AzCopy

AzCopy は Azure Storage へのデータコピーを行うコマンドラインツールです。Blob Storage や Data Lake Storage への高速データ転送をサポートしています。

ただし、大容量データの場合は **WAN 回線の速度に依存するため、回線が遅い環境では適していません。**

---

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Factory]]
## Azure Data Factory

Azure Data Factory はデータ統合および ETL（Extract, Transform, Load）サービスです。オンプレミスデータやクラウドデータを統合して処理できます。

セルフホステッド統合ランタイムを使用するとオンプレミスからデータ転送が可能ですが、これもネットワーク帯域に依存します。

---

# データ移行サービス選択の目安

データ量と移行方法の目安は次のように整理できます。

小～中規模データ（数十 TB）  
→ Azure Data Box Disk

中～大規模データ（数百 TB）  
→ Azure Data Box

超大規模データ（PB級）  
→ Azure Data Box Heavy

物理ディスクを自分で用意  
→ Azure Import/Export Service

ネットワーク転送  
→ AzCopy / Azure Data Factory

---

# 試験対策のポイント

試験では次のキーワードが出た場合、**物理データ転送サービスを選択する問題**であることが多いです。

- WAN が遅い
- インターネット帯域が不足
- 大容量データ（数十 TB 以上）
- 物理配送

このような場合は **Azure Data Box 系サービス または Import/Export** を選択します。