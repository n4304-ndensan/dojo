---
分類: Cloud
tags:
  - cloud/azure
  - cloud/azure/data-factory
  - cloud/azure/blob-storage
  - cloud/architecture/data-migration
  - cloud/architecture/high-availability
  - data/integration
  - data/etl
  - exam/azure
---

# Azure Data Factory を使用したオンプレミスから Blob Storage へのデータ移行

## 1. 背景（シナリオ）

ある組織では、オンプレミスのファイルサーバーに保存されている **約5TBの非構造化データ**を Azure Blob Storage に移行する計画があります。  
この移行は単なるコピーではなく、業務に影響を与えないよう **最小限のダウンタイム**で実施する必要があります。

さらに、移行中のデータ破損や欠損を防ぐため、**データ整合性の維持**も重要な要件です。

組織はデータ統合およびデータ移動のオーケストレーションツールとして **Azure Data Factory (ADF)** を使用することを決定しました。

このシナリオでは、オンプレミス環境と Azure クラウドの間で安全かつ信頼性の高いデータ転送を実現する構成を設計する必要があります。

## 2. 要件整理

問題文から読み取れる重要な要件を整理すると次の通りです。

まず、データはオンプレミスのファイルサーバーに存在します。  
そのため、Azure Data Factory がオンプレミス環境へ接続できる必要があります。

次に、データ量は **5TB と大容量**です。  
このため、長時間の転送中でもサービスが停止しないよう、冗長性と信頼性が必要です。

さらに、移行中の障害によってデータ転送が停止しないよう **高可用性構成**が求められます。

このシナリオの要件は次の通りです。

オンプレミスデータソースへの接続  
大容量データ転送  
最小限のダウンタイム  
高可用性  
データ整合性の維持  

## 3. 技術の基本概念

Azure Data Factory は、クラウドベースの **データ統合サービス**です。  
データのコピー、変換、移動をパイプラインとして定義し、自動化することができます。

Data Factory では、データ移動を実行するためのコンポーネントとして **Integration Runtime (IR)** が使用されます。

Integration Runtime は、データコピーやデータ処理を実行するコンピューティング環境です。

IR には主に次の3種類があります。

まず **Azure Integration Runtime** はクラウド上で動作する標準ランタイムです。  
主にクラウドサービス間のデータ移動に使用されます。

次に **Self-hosted Integration Runtime** はオンプレミス環境にインストールされるランタイムです。  
オンプレミスデータソースと Azure サービスの接続に使用されます。

最後に **Azure SSIS Integration Runtime** は SSIS パッケージの実行用です。

今回のシナリオでは、オンプレミス環境から Azure へのデータ転送が必要なため **Self-hosted Integration Runtime** が必要になります。

## 4. アーキテクチャまたは設計のポイント

このシナリオでは、オンプレミスサーバーに **Self-hosted Integration Runtime (SHIR)** をインストールします。

このコンポーネントは Azure Data Factory とオンプレミス環境の間で安全な通信を確立し、データコピー処理を実行します。

さらに、大容量データ転送中の障害に備えるため、SHIR を **高可用性構成（HA）**で配置します。

高可用性構成では、複数の SHIR ノードを同じ IR クラスターに参加させます。

この構成により、1台のサーバーが停止しても別のノードが処理を継続できます。

## 5. 設計判断（なぜこの構成になるか）

Self-hosted Integration Runtime は、オンプレミスデータソースへ接続できる唯一の Data Factory ランタイムです。

さらに、高可用性構成を採用することで次のメリットがあります。

まず、データ転送中のサーバー障害による停止を防ぐことができます。  
次に、長時間の大容量データ転送でも処理を継続できます。  
さらに、データコピー処理の信頼性が向上します。

このように、SHIR の高可用性構成は **大容量データ移行のベストプラクティス**です。

## 6. 他の選択肢が誤りな理由

### A Azure Data Factory Integration Runtime をインストール

Integration Runtime をインストールするだけでは十分ではありません。  
高可用性構成を設定しない場合、ランタイム障害時にデータ転送が停止する可能性があります。

### C Azure Data Box

Azure Data Box はオフラインデータ転送のための物理デバイスです。  
ネットワーク帯域が極端に制限されている場合には有効ですが、Azure Data Factory を利用する移行には適していません。

### D Azure File Sync

Azure File Sync はオンプレミスのファイルサーバーと **Azure Files** を同期するサービスです。  
今回の移行先は **Azure Blob Storage** であるため、目的に適していません。

## 7. 最終回答

B. **高可用性構成で Self-hosted Integration Runtime を設定する**

## 8. まとめ

Azure Data Factory を使用してオンプレミスデータを Azure に移行する場合、Self-hosted Integration Runtime が必要になります。

さらに、大容量データ移行では高可用性構成を設定することで、障害発生時でもデータ転送を継続できます。

試験では **「オンプレミス → Azure Data Factory」「データ移行」「Integration Runtime」** が出た場合、Self-hosted Integration Runtime が正解になるケースが多いです。