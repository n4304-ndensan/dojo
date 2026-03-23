[[docs/tech_notes/microsoft/azure/az305/Azure用語集.md#Azure Virtual Machines]]
# Azure VM 上の SQL Server におけるディスクキャッシュ設定設計

（P40 Managed Disk を使用したデータディスクとログディスクの最適化）

---

# 1 背景（シナリオ）

企業では Azure 仮想マシン（Azure VM）上で SQL Server を実行し、データベースワークロードを処理するケースが多い。

Azure VM 上で SQL Server を実行する場合、ディスクのパフォーマンス設計は非常に重要になる。特に次の要素が性能とデータ整合性に大きく影響する。

- データファイルディスク
    
- トランザクションログディスク
    
- ディスクキャッシュ設定
    

今回のシナリオでは次の構成の SQL Server が Azure VM 上にデプロイされている。

|ディスク|用途|
|---|---|
|P40 Disk 1|SQL Server データファイル|
|P40 Disk 2|SQL Server トランザクションログ|

P40 は Azure Premium SSD ディスクの一種であり、高い IOPS とスループットを提供する。

今回の要件は次の通りである。

- SQL Server の **パフォーマンスを最適化する**
    
- **データ整合性を維持する**
    

そのため、適切な **ホストキャッシュ設定（Host Cache）**を選択する必要がある。

---

# 2 Azure Managed Disk のキャッシュ

Azure Managed Disk には **Host Caching** という機能がある。

Host Cache は、Azure VM ホスト側のメモリにディスクデータをキャッシュすることで、ディスクアクセスのレイテンシを削減する仕組みである。

Azure では以下のキャッシュモードが利用できる。

|キャッシュモード|説明|
|---|---|
|None|キャッシュなし|
|ReadOnly|読み取りのみキャッシュ|
|ReadWrite|読み書き両方キャッシュ|

キャッシュは **VM ホストのローカル SSD** に保存される。

---

# 3 SQL Server ディスク構成

SQL Server のストレージは通常、次の 2 種類のファイルに分かれている。

|ファイル|用途|
|---|---|
|Data File (.mdf / .ndf)|データ保存|
|Log File (.ldf)|トランザクションログ|

この 2 つのファイルは **I/O 特性が大きく異なる**。

---

# 4 SQL Server データファイルの I/O 特性

データファイルの特徴

- ランダム I/O
    
- 読み取りが多い
    
- キャッシュの恩恵を受けやすい
    

そのため、Azure SQL Server のベストプラクティスでは

```text
Data Disk = ReadOnly Cache
```

が推奨される。

理由

- 読み取り性能向上
    
- ランダムアクセス高速化
    

---

# 5 SQL Server ログファイルの I/O 特性

ログファイルはデータファイルとは性質が異なる。

特徴

- シーケンシャル書き込み
    
- 書き込み中心
    
- トランザクション整合性が重要
    

SQL Server は **Write-Ahead Logging（WAL）** を使用する。

つまり

```text
ログ書き込み
↓
データ書き込み
```

の順序が保証される必要がある。

---

# 6 ログディスクにキャッシュを使う問題

ログディスクにキャッシュを使用すると問題が発生する可能性がある。

理由

- キャッシュは **揮発性**
    
- VM ホスト障害でデータ消失の可能性
    
- WAL の整合性が破壊される可能性
    

そのため Microsoft の推奨設定は

```text
Log Disk = None
```

である。

---

# 7 SQL Server on Azure の推奨設定

Microsoft が推奨する SQL Server ディスクキャッシュ設定は次の通り。

|ディスク|キャッシュ|
|---|---|
|データディスク|ReadOnly|
|ログディスク|None|

この設定により

- 読み取り性能の向上
    
- トランザクション整合性の維持
    

が両立できる。

---

# 8 ディスク構成の例

今回の構成

|ディスク|種類|用途|
|---|---|---|
|Disk1|P40|Data File|
|Disk2|P40|Log File|

最適キャッシュ設定

|ディスク|キャッシュ|
|---|---|
|Disk1|ReadOnly|
|Disk2|None|

---

# 9 アーキテクチャ

```text
Azure VM
 │
 ├ P40 Disk
 │   SQL Data
 │   Cache = ReadOnly
 │
 └ P40 Disk
     SQL Log
     Cache = None
```

---

# 10 他の選択肢が不正解の理由

## B

Data: ReadWrite

データディスクに ReadWrite キャッシュを使うと

- キャッシュ整合性リスク
    
- パフォーマンス低下
    

が発生する可能性がある。

---

## C

Log: ReadOnly

ログディスクにキャッシュを使用することは

```text
トランザクション整合性
```

を損なう可能性がある。

---

## D

Data: ReadWrite  
Log: ReadWrite

ログディスクに Write キャッシュは危険であり

```text
データ破損リスク
```

がある。

---

# 11 最終回答

正解

```text
A

データディスク: ReadOnly
ログディスク: None
```

---

# 12 まとめ

SQL Server on Azure VM のディスク設計では

**データディスクとログディスクの I/O 特性を理解することが重要**である。

|ディスク|推奨キャッシュ|
|---|---|
|Data Disk|ReadOnly|
|Log Disk|None|

この構成により

- SQL Server 読み取り性能の最適化
    
- トランザクションログ整合性確保
    

を同時に実現できる。

これは **Azure VM 上の SQL Server の公式ベストプラクティス**である。