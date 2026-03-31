[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Data Lake Storage Gen2]]
## オンプレミス NAS から Azure Data Lake Storage Gen2 へのデータ移行設計

### ― AzCopy を利用した大規模ファイル転送アーキテクチャ ―

---

# 1 背景とシナリオ

企業がデータ分析基盤やクラウドネイティブアプリケーションを導入する際、オンプレミスに保存されている大量のデータをクラウドストレージへ移行する必要があるケースが多い。今回のシナリオでは、オンプレミス環境に **Network Attached Storage（NAS）** が存在し、その中に約 **1TB の JSON ファイル**を含むファイル共有が保存されている。

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Blob Storage]]
同時に Azure 環境には **storage1** という名前のストレージアカウントが存在しており、このストレージアカウントでは **Hierarchical Namespace（階層型名前空間）** が有効化されている。階層型名前空間が有効なストレージアカウントは、通常の Blob Storage ではなく **Azure Data Lake Storage Gen2（ADLS Gen2）** として動作する。

今回の要件は、オンプレミスの NAS デバイスからこの storage1 へ JSON ファイルを効率的に転送する方法を設計することである。データ量は 1TB と比較的大きいため、単純なコピーではなく **Azure に最適化されたデータ転送ツール**を利用する必要がある。

この要件を満たす最適なツールが **AzCopy** である。

---

# 2 Azure Data Lake Storage Gen2 の概要

Azure Data Lake Storage Gen2 は、大規模データ分析ワークロードを目的として設計されたストレージサービスである。Blob Storage をベースとしており、**階層型ファイルシステム（Hierarchical Namespace）** をサポートしている。

通常の Blob Storage はオブジェクトストレージとして動作するため、ディレクトリ構造は論理的な概念に過ぎない。しかし、Data Lake Storage Gen2 ではディレクトリ構造がネイティブにサポートされるため、ビッグデータ処理に適したファイル操作が可能になる。

典型的な構造は次のようになる。

```text
Data Lake Storage Gen2
│
├ raw-data
│   ├ logs
│   └ json-files
│
├ processed-data
│
└ analytics
```

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Synapse Analytics]]
[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Databricks]]
この構造は Hadoop や Spark のような分散データ処理システムと相性が良く、Azure Synapse Analytics や Azure Databricks などの分析基盤で広く利用されている。

---

# 3 データ移行の要件

今回の移行シナリオには次の特徴がある。

- データは **オンプレミス NAS に保存されている**
    
- データ量は **約1TB**
    
- ファイル形式は **JSON**
    
- 転送先は **Azure Data Lake Storage Gen2**
    
- 大量ファイルを **高速かつ効率的に転送する必要がある**
    

このような条件では、Azure が提供する **専用のデータ転送ツール**を使用することが推奨される。

---

# 4 AzCopy の概要

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Storage Account]]
AzCopy は Microsoft が提供する **Azure Storage 向けのコマンドラインデータ転送ツール**である。大量のデータを Azure Storage にアップロードまたはダウンロードする際に最適化されている。

AzCopy の主な特徴は次の通りである。

- 高速並列転送
    
- 大量ファイル処理
    
- 再試行機能
    
- 中断後の再開
    
- Data Lake Storage Gen2 対応
    
- コマンドライン自動化
    

AzCopy は次のようなデータ転送をサポートする。

```text
Local → Azure Storage
Azure Storage → Local
Azure Storage → Azure Storage
```

今回のケースでは

```text
NAS → Azure Data Lake Storage Gen2
```

という構成になる。

---

# 5 データ転送のアーキテクチャ

オンプレミス NAS から Azure Data Lake Storage へのデータ転送は次の構成になる。

```text
On-premises NAS
      │
      │ 1 TB JSON files
      │
      ▼
Migration Host (AzCopy)
      │
      ▼
Azure Data Lake Storage Gen2
(storage1)
```

この構成では、NAS にアクセスできるサーバーまたはクライアントマシンで AzCopy を実行し、Azure ストレージへ直接アップロードする。

---

# 6 AzCopy によるデータ転送の仕組み

AzCopy は内部的に複数の並列スレッドを使用し、ファイル転送を高速化する。さらに、転送失敗時には自動的に再試行を行うため、大量データの移行でも安定した転送が可能である。

データ転送の基本フローは次の通り。

```text
NAS File Share
      │
      ▼
AzCopy CLI
      │
      ▼
Azure Data Lake Storage
```

例えば次のようなコマンドでアップロードできる。

```
azcopy copy "/mnt/nas/jsonfiles" "https://storage1.dfs.core.windows.net/container" --recursive
```

このコマンドにより、NAS 内の JSON ファイルが Data Lake Storage に一括アップロードされる。

---

# 7 他の選択肢が不適な理由

### Azure File Sync

[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Files]]
Azure File Sync は **オンプレミスの Windows ファイルサーバーと Azure Files を同期するためのサービス**である。目的はファイルサーバーのキャッシュや同期であり、Azure Data Lake Storage Gen2 へのデータ移行には適していない。

さらに、Azure File Sync は **Hierarchical Namespace をサポートしていない**ため、今回のストレージ構成とは互換性がない。

---

### Robocopy

Robocopy は Windows 環境でファイルコピーを行うためのツールであり、ローカルまたはネットワーク共有間のファイル転送には適している。しかし Azure Storage への直接アップロードをネイティブにサポートしていないため、このシナリオでは使用できない。

---

### Azure Storage Mover

Azure Storage Mover はオンプレミスのファイル共有を Azure に移行するためのサービスであるが、現在は主に **Azure Files や Blob Storage 向けに最適化されている**。また、機能成熟度の観点でも AzCopy の方が広く利用されており、Data Lake Storage Gen2 への大規模データ転送では AzCopy が最適な選択となる。

---

# 8 完成アーキテクチャ

最終的なデータ移行構成は次のようになる。

```text
On-premises NAS
      │
      ▼
Migration Server
(AzCopy CLI)
      │
      ▼
Azure Storage Account
(storage1)
      │
      ▼
Azure Data Lake Storage Gen2
```

この構成では、NAS に保存されている JSON ファイルを効率的に Azure Data Lake Storage に移行できる。

---

# 9 AzCopy のメリット

AzCopy を利用することで次の利点が得られる。

- 高速な並列データ転送
    
- 大量データ移行に最適化
    
- Data Lake Storage Gen2 をネイティブサポート
    
- コマンドラインによる自動化
    
- 再試行や再開機能による信頼性
    

そのため、Azure 環境で大規模ファイル移行を行う際の **標準的なツール**として利用されている。

---

# 10 まとめ

今回のシナリオでは

- オンプレミス NAS に 1TB の JSON ファイルが存在
    
- 転送先は Azure Data Lake Storage Gen2
    
- 大量データを効率的に移行する必要がある
    

この要件を満たす最適なソリューションは

**AzCopy**

である。

最終構成は次の通り。

```text
NAS
 │
 ▼
AzCopy
 │
 ▼
Azure Data Lake Storage Gen2
```

AzCopy を利用することで、大量の JSON ファイルを Azure Data Lake Storage に高速かつ効率的に移行できる。