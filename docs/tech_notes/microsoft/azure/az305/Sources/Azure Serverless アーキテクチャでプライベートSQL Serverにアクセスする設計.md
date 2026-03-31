---
分類: cloud/azure
tags:
  - cloud/azure/functions
  - cloud/azure/event-grid
  - cloud/azure/virtual-network
  - cloud/architecture/serverless
  - exam/azure/az-305
---

# Azure Serverless アーキテクチャでプライベートSQL Serverにアクセスする設計

## 1. 背景（シナリオ）
サーバーレスアーキテクチャにおいて、Azure Event Grid のイベントに応答し、Azure 仮想マシン上でホストされているプライベートな SQL Server インスタンスに安全にアクセスしながら、カスタム C# コードを実行する必要があります。さらに、パフォーマンス最適化と低遅延も求められています。

## 2. 要件整理
- **サーバーレス環境**: 手動でサーバーを管理せず、自動スケーリング可能
- **イベント駆動**: Azure Event Grid のイベントに応答
- **カスタム C# コードの実行**
- **プライベート SQL Server への安全なアクセス**: VM 内のリソースに直接接続
- **パフォーマンスと低遅延**: コールドスタートを最小化し迅速なレスポンス

## 3. 技術の基本概念
- **Azure Functions (Premium Plan + VNET 統合)**:
  - サーバーレスコンピュートサービスで、自動スケーリングとイベント駆動をサポート
  - C# コードを直接実行可能
  - VNET 統合により、プライベート SQL Server への安全な接続を確保
  - Premium Plan は事前ウォームインスタンスを提供し、低遅延を実現

- **その他の選択肢との違い**:
  - **Logic Apps**: オーケストレーション向きで、カスタムコード実行には不向き
  - **Consumption Plan + ハイブリッド接続**: VNET 統合に比べ低効率で、コールドスタートによる遅延が発生
  - **ISE + Logic Apps**: 高セキュリティだが、複雑でコスト高、カスタムコード実行は主目的ではない

## 4. アーキテクチャまたは設計のポイント
- Azure Functions を Premium Plan でデプロイ
- VNET 統合を有効にし、VM 内 SQL Server へ安全接続
- Event Grid トリガーを設定し、イベント駆動で関数を実行
- パフォーマンス最適化のため、プレウォームインスタンスを利用

## 5. 設計判断（なぜこの構成になるか）
- サーバーレスで自動スケーリング可能
- C# コードを直接実行でき、処理柔軟性を提供
- VNET 統合によりプライベートリソースへの安全アクセスを確保
- Premium Plan で低遅延、高パフォーマンスを実現

## 6. 他の選択肢が誤りな理由
- **Logic Apps (Consumption Plan + Private Endpoint)**: カスタムコード実行が主目的ではない
- **Functions (Consumption Plan + Hybrid Connection)**: 遅延が大きく、Azure VM 内リソースアクセスは効率的でない
- **Logic Apps ISE + Service Endpoint**: 高コスト・複雑で、カスタムコード実行には最適でない

## 7. 最終回答
**A) VNET統合によるプレミアムプランにおけるAzure機能**

## 8. まとめ
Azure Functions の Premium Plan と VNET 統合を組み合わせることで、イベント駆動のサーバーレス処理、カスタム C# コードの実行、プライベート SQL Server への安全アクセス、低遅延応答を同時に実現できます。この構成がシナリオのすべての要件に最適です。