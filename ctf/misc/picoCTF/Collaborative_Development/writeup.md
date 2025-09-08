# Collaborative Development

## 問題情報

- **Author:** Jeffery John
- **Description:**
  “My team has been working very hard on new features for our flag printing program! I wonder how they'll work together?”
  `challenge.zip`

---

## 前提

- `challenge.zip` を解凍すると **Git 管理されたディレクトリ**（隠しフォルダ `.git`）が含まれている。
- 主要ファイルは `flag.py`。**複数ブランチに分散した変更**があり、それらを正しく統合するとフラグが得られる。

---

## 方針（要点）

1. **ブランチ構成を把握**して、`flag.py` の差分を確認する。
2. 安全な作業用ブランチで **順にマージ**し、競合は「両方の変更を残す（union）」方針で解決。
3. 統合後の `flag.py` を実行してフラグを出力。

---

## 手順（再現可能なコマンド）

### 1) 展開と初期調査

```bash
unzip challenge.zip
cd challenge
git status
git branch -a
git log --oneline --graph --decorate --all
```

`flag.py` が各ブランチでどう違うかを確認：

```bash
git show <branch1>:flag.py > /tmp/flag_branch1.py
git show <branch2>:flag.py > /tmp/flag_branch2.py
diff -u /tmp/flag_branch1.py /tmp/flag_branch2.py
```

### 2) 作業用ブランチを作成しマージ

```bash
git checkout -b solve
git merge <branch1>
git merge <branch2>
```

### 3) 実行してフラグ取得

```bash
python3 flag.py
# -> FLAG が出力される
```

---

## 関連技術

- Git のブランチ統合（`git merge` / 競合解決）の基本。
- 変更が分散したコードの**機能統合**と**依存関係の整理**。
