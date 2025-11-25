# 基本操作

## 状況確認

- `git status`
  作業ツリーとステージの状態を確認する。

## ステージングとコミット

- `git add <file>` : ファイルをステージに追加
- `git commit -m "message"` : コミットを作成
- `git commit --amend -m "new message"` : 直前のコミットを修正

---

## コミットを戻す（git reset）

Git には **3 つの領域** がある：

- コミット（Repository / HEAD）
- ステージ（Index）
- 作業ツリー（Working Directory）

`git reset` は HEAD を動かしつつ、オプションでステージや作業ツリーをどう扱うかが変わる。

### 1. `git reset --soft HEAD~1`

- HEAD を 1 つ前に戻す
- ステージと作業ツリーはそのまま
- → コミットだけ取り消し、すぐに再コミット可能

### 2. `git reset --mixed HEAD~1`（デフォルト）

- HEAD を 1 つ前に戻す
- ステージをリセット（空に）
- 作業ツリーはそのまま
- → ファイルは残るがステージから外れる

### 3. `git reset --hard HEAD~1`

- HEAD を 1 つ前に戻す
- ステージも作業ツリーも巻き戻す
- → 変更が消えるので要注意

### まとめ表

| コマンド  | HEAD | ステージ | 作業ツリー |
| --------- | ---- | -------- | ---------- |
| `--soft`  | 移動 | 保持     | 保持       |
| `--mixed` | 移動 | リセット | 保持       |
| `--hard`  | 移動 | リセット | リセット   |

---

## 作業の一時退避（git stash）

- `git stash` : 現在の変更を一時保存
- `git stash list` : スタッシュ一覧
- `git stash pop` : 直前の stash を復元して削除
- `git stash apply` : stash を復元（リストには残す）

---

## リモートとのやり取り

- `git push origin main`
  ローカルのコミットをリモートへ送信

- `git push --force`
  リモート履歴を強制的に上書き（注意が必要）

- `git fetch origin`
  リモートの最新情報を取得（作業ツリーには反映しない）

- `git pull origin main`
  リモートを取り込み、自分のブランチに反映

  - `git pull --rebase` でリベースも可能

---

## 特定のコミットを取り込む（git cherry-pick）

- `git cherry-pick <commit-hash>`
  指定したコミットを現在のブランチに適用

- 複数まとめて適用

  ```bash
  git cherry-pick <hash1> <hash2>
  ```

---

## `.git` ディレクトリの構造（補足）

- `.git/config` : リポジトリの設定
- `.git/HEAD` : 現在のブランチ
- `.git/refs/heads/` : 各ブランチの先頭コミット
- `.git/objects/` : コミット・ツリー・ファイルデータ

---

## まとめ

- 状況確認：`status`
- 戻す：`reset` / `commit --amend`
- 一時退避：`stash`
- リモート：`push` / `fetch` / `pull`
- 一部適用：`cherry-pick`

---
