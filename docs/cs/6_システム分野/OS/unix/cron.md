#Linux #システム管理 #タスクスケジューリング #cron #crontab #運用

# cron

---

## 概要（Overview）

**cron** は、UNIX/Linuxシステムにおける定期タスク自動実行のためのデーモン（常駐プロセス）である。
システムメンテナンス・バックアップ・ログローテーション・監視など、
あらゆる自動化タスクの基盤として機能する。

---

## 1. cronデーモン（`crond`）

### 定義

* 実行プロセス名: **`crond`**
* 役割: cron設定ファイルを読み込み、指定されたスケジュールに従いジョブを実行する。

### 主要ファイル

| ファイル                                  | 説明         |
| ------------------------------------- | ---------- |
| `/usr/sbin/crond`                     | デーモン本体     |
| `/var/run/crond.pid`                  | プロセスIDファイル |
| `/var/log/cron` または `/var/log/syslog` | 実行ログ出力先    |

### サービス操作（systemd環境）

```bash
sudo systemctl start crond      # デーモン起動
sudo systemctl stop crond       # 停止
sudo systemctl restart crond    # 再起動
sudo systemctl enable crond     # 自動起動を有効化
sudo systemctl disable crond    # 自動起動を無効化
sudo systemctl status crond     # 状態確認
```

※ Debian/Ubuntuではサービス名が `cron` の場合もある。

---

## 2. 設定ファイル構造（Configuration Layout）

| パス                       | 説明                        | 権限     |
| ------------------------ | ------------------------- | ------ |
| `/etc/crontab`           | システム全体設定                  | rootのみ |
| `/etc/cron.d/`           | パッケージ単位のジョブ設定             | rootのみ |
| `/var/spool/cron/<user>` | 各ユーザの設定 (`crontab -e`で管理) | 該当ユーザ  |
| `/etc/cron.daily/`       | 毎日実行スクリプト格納               | rootのみ |
| `/etc/cron.weekly/`      | 毎週実行スクリプト格納               | rootのみ |
| `/etc/cron.monthly/`     | 毎月実行スクリプト格納               | rootのみ |

---

## 3. cron書式（Syntax）

### 一般形式

```
# ┌──────── 分 (0-59)
# │ ┌────── 時 (0-23)
# │ │ ┌──── 日 (1-31)
# │ │ │ ┌── 月 (1-12)
# │ │ │ │ ┌ 曜日 (0-7) 0および7は日曜
# │ │ │ │ │
# * * * * * 実行コマンド
```

### 拡張指定

| 記法        | 意味    | 例                  |
| --------- | ----- | ------------------ |
| `*`       | 任意の値  | 毎時・毎日など            |
| `*/N`     | N間隔   | `*/5` → 5分ごと       |
| `A,B`     | 複数指定  | `1,15` → 1日と15日    |
| `A-B`     | 範囲    | `9-17` → 9時から17時まで |
| `@reboot` | 起動時1回 | サーバ起動直後に実行         |

---

## 4. crontabコマンド（ユーザ設定）

| コマンド                | 説明                  |
| ------------------- | ------------------- |
| `crontab -e`        | 現在のユーザのcronジョブを編集   |
| `crontab -l`        | 現在のジョブ一覧を表示         |
| `crontab -r`        | 現在のジョブを削除           |
| `crontab -u <user>` | rootが他ユーザのジョブを操作    |
| `crontab -i`        | 削除前に確認するインタラクティブモード |

### crontabファイル例

```bash
# m h dom mon dow command
0 2 * * * /usr/local/bin/backup.sh
*/10 * * * * /usr/local/bin/check_health
```

---

## 5. 環境変数と実行環境（Environment）

cronは**最小限の環境**で実行されるため、環境変数を明示することが推奨される。

```bash
SHELL=/bin/bash
PATH=/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin
MAILTO=root
HOME=/
```

**注意:**
`$PATH` が短いため、絶対パスでのコマンド指定が望ましい。

---

## 6. cron.allow / cron.deny（アクセス制御）

| ファイル              | 目的                      |
| ----------------- | ----------------------- |
| `/etc/cron.allow` | 許可されたユーザを列挙（存在すればこれが優先） |
| `/etc/cron.deny`  | 実行を禁止するユーザを列挙           |

例:

```bash
# /etc/cron.allow
root
admin
deploy
```

---

## 7. ジョブの削除・停止・再登録

### 個別ジョブの削除

```bash
crontab -l | grep -v "backup.sh" | crontab -
```

### 全削除

```bash
crontab -r
```

### 停止（systemd管理下）

```bash
sudo systemctl stop crond
```

### 再登録

```bash
(crontab -l; echo "0 3 * * * /usr/local/bin/newtask.sh") | crontab -
```

---

## 8. ログ・デバッグ（Monitoring & Debugging）

* **ログ出力場所**

  * `/var/log/cron`（CentOS/RHEL系）
  * `/var/log/syslog`（Debian/Ubuntu系）

* **確認**

  ```bash
  grep CRON /var/log/syslog
  journalctl -u cron
  ```

* **メール通知**

  * 実行結果は`MAILTO`環境変数に設定されたアドレスへ送信される。
  * 無指定時はローカルユーザ宛に送信。

---

## 9. ベストプラクティス（Best Practices）

* コマンドは絶対パス指定で記述する。
* 標準出力・標準エラー出力をログファイルへリダイレクトする。

  ```bash
  0 1 * * * /usr/bin/backup.sh >> /var/log/backup.log 2>&1
  ```
* 環境変数を明示的に指定する。
* 長期実行ジョブは`systemd.timer`へ移行するのが望ましい。
* `@reboot`ジョブには待機処理（例：`sleep 30`）を挿入する。

---

## 10. Cloze知識整理（Anki対応）

* cronデーモンの実体は {**crond**} である。
* システム全体の設定ファイルは {**/etc/crontab**} である。
* ユーザごとの設定は {**/var/spool/cron/<user>**} に格納される。
* ジョブ書式は {**分・時・日・月・曜日・コマンド**} の6要素で構成される。
* cronの実行環境変数は {**限定的であるため絶対パス指定が推奨**} される。
* cronのアクセス制御は {**/etc/cron.allow**} と {**/etc/cron.deny**} により行われる。
* cronサービスの起動コマンドは {**systemctl start crond**} である。
* 実行結果のログは {**/var/log/cron**} または {**/var/log/syslog**} に出力される。

---

## 11. systemd.timer との比較（Modern Alternative）

| 項目     | cron     | systemd.timer                     |
| ------ | -------- | --------------------------------- |
| 実行単位   | 行単位ジョブ   | サービス単位                            |
| ログ     | syslog経由 | journalctlで一元管理                   |
| 依存関係管理 | 不可       | 可能（`After=`, `Requires=`）         |
| 柔軟性    | 固定的      | 高い（`OnCalendar=`, `OnBootSec=`など） |

---

## 12. 参考文献（References）

* `man 5 crontab`
* `man 8 cron`
* Debian Wiki: [Cron](https://wiki.debian.org/cron)
* RedHat Documentation: *Configuring Cron Jobs*
* ArchWiki: [Cron](https://wiki.archlinux.org/title/Cron)

---

