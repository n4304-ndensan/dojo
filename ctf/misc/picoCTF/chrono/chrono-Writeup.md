#競技セキュリティ #picoCTF #Linux #cron

# chrono — Writeup

**Author:** Mubarak Mikail
**Category:** General Skills / Linux
**Difficulty:** Easy
**Tags:** cron, crontab, privilege, file-inspection

---

## 概要（Problem Overview）

Linuxサーバ上で「タスクを一定間隔で自動実行する」方法を調べる問題である。
与えられたサーバにSSHで接続し、`cron`関連の設定を探索することでフラグを取得する。

---

## 制約分析（Constraint Analysis）

* 接続情報:

  ```
  Server: saturn.picoctf.net
  Port: 59232
  Username: picoplayer
  Password: pYkku7iMsS
  ```
* `sudo`権限なし（Permission denied多発）
* `/challenge`ディレクトリはアクセス不可
* `/etc`ディレクトリには読み取り可能な設定ファイル多数あり

---

## 解法設計（Solution Design）

### ステップ1 — SSH接続

問題文の接続情報を用いてサーバにログインする。

```bash
ssh picoplayer@saturn.picoctf.net -p 59232
```

### ステップ2 — 権限確認と探索

トップディレクトリを確認。

```bash
ls /
```

`/challenge` はアクセスできないため、他の箇所を調査する。

### ステップ3 — cron関連の探索

`cron`や`crontab`コマンドは権限不足で動作しないが、設定ファイルは閲覧可能。

```bash
cd /etc
ls | grep cron
```

確認すると、次のようなディレクトリ構造を発見。

```
cron.d
cron.daily
cron.hourly
cron.weekly
cron.monthly
crontab
```

### ステップ4 — /etc/crontab の内容確認

```bash
cat /etc/crontab
```

内容を読むとコメント内にフラグが記載されている。

```
# picoCTF{Sch3DUL7NG_T45K3_L1NUX_7754e199}
```

---

## 結果（Flag）

```
picoCTF{Sch3DUL7NG_T45K3_L1NUX_7754e199}
```

---

## 要約表（Summary Table）

| 項目     | 内容                                       |
| ------ | ---------------------------------------- |
| 分類     | Linux / Task Scheduling                  |
| 主要コマンド | `cron`, `crontab`, `cat /etc/crontab`    |
| キー概念   | タスクスケジューラ, 権限, システム設定ファイル                |
| 難易度    | Easy                                     |
| フラグ    | picoCTF{Sch3DUL7NG_T45K3_L1NUX_7754e199} |

---

## 学習ポイント（Key Takeaways）

* Linuxでは定期タスクの自動化に **cron** を使用する。
* システム全体の設定は `/etc/crontab` に保存される。
* ユーザ固有のジョブは `crontab -e` により編集可能である。
* `cron`ディレクトリ群（`cron.daily`, `cron.weekly`など）はスクリプトベースのジョブ配置場所である。

---

この問題は、**Linuxのタスクスケジューラ構造を理解しているか**を確認する入門的CTF課題である。

