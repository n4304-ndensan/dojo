# CLI 操作環境の構築

## この文書の責務

fzf、zoxide、ripgrep、fd、eza の具体的な使い方、従来コマンドとの比較、カスタマイズ方法を解説します。[[03 zshrc 設定とプラグイン]] では `.zshrc` への組み込み方を扱いましたが、この文書では各ツール個別の操作と、ツール間の連携パターンを掘り下げます。

## 前提

- [[03 zshrc 設定とプラグイン]] が完了していること
- `apt install ripgrep fd-find fzf eza zoxide` が実行済みであること

## インストール（まだの場合）

```bash
sudo apt install -y ripgrep fd-find fzf eza zoxide
mkdir -p ~/.local/bin
ln -sf "$(which fdfind)" ~/.local/bin/fd
```

## 1. fd — find の代替

### 従来の find との比較

```bash
# find: ~/.git ディレクトリ内のファイルも返す、.gitignore を無視、遅い
find . -name "*.rs"

# fd: .gitignore を自動尊重、.git を除外、色付き出力、高速
fd -e rs
```

fd が速い理由は2つあります。

1. **並列探索**: fd はディレクトリツリーを複数スレッドで並列に走査します。find はシングルスレッドです。
2. **自動フィルタ**: `.gitignore`、`.fdignore`、`.ignore` に記載されたパターンを自動で除外するため、`node_modules` や `target/` などの巨大ディレクトリをスキップします。

### 基本操作

| コマンド | 操作 |
| --- | --- |
| `fd pattern` | ファイル名がパターンにマッチするものを検索 |
| `fd -e rs` | 拡張子が `.rs` のファイルを検索 |
| `fd -t d` | ディレクトリのみ検索 |
| `fd -t f` | ファイルのみ検索 |
| `fd -H pattern` | 隠しファイルも含めて検索 |
| `fd -I pattern` | `.gitignore` を無視して検索 |
| `fd pattern /path` | 指定ディレクトリ以下を検索 |

### 実用例

```bash
# Rust のソースファイルだけ一覧
fd -e rs

# テストファイルだけ一覧
fd test -e rs

# 特定ディレクトリ以下の設定ファイル
fd -e toml -e yaml -e json ~/.config

# 空ディレクトリを探す
fd -t d -e empty
```

## 2. ripgrep (rg) — grep の代替

### 従来の grep との比較

```bash
# grep: 再帰検索に -r が必要、遅い、バイナリも読む
grep -r "fn main" .

# rg: 再帰がデフォルト、.gitignore 尊重、バイナリ自動スキップ、高速
rg "fn main"
```

ripgrep が速い理由:

1. **正規表現エンジン**: Rust の `regex` クレートを使用。DFA ベースで、バックトラックが発生しないため、最悪ケースでも線形時間で動作します。
2. **メモリマップドファイル**: 大きなファイルをメモリマップで読み、コピーを減らしています。
3. **.gitignore 自動尊重**: fd と同様、無関係なファイルを自動スキップします。

### rg の基本操作

| コマンド | 操作 |
| --- | --- |
| `rg pattern` | カレントディレクトリ以下でテキスト検索 |
| `rg -i pattern` | 大文字小文字を区別しない |
| `rg -w pattern` | 単語境界でマッチ |
| `rg -l pattern` | マッチしたファイル名のみ表示 |
| `rg -c pattern` | ファイルごとのマッチ数を表示 |
| `rg -t rust pattern` | Rust ファイルのみ検索 |
| `rg -t py pattern` | Python ファイルのみ検索 |
| `rg --no-ignore pattern` | `.gitignore` を無視して検索 |
| `rg -A 3 -B 3 pattern` | 前後 3 行のコンテキストを表示 |

### rg の実用例

```bash
# TODO コメントを探す
rg "TODO|FIXME|HACK"

# 特定の関数定義を探す
rg "fn\s+process_"

# Rust ファイルだけでエラーハンドリングを探す
rg -t rust "unwrap\(\)"

# ファイル名だけ一覧（パイプに便利）
rg -l "async fn"
```

### rg と Neovim の連携

[[03 プラグイン設定（検索・ファイル・UX）]] で設定した fzf-lua の `live_grep` は、バックエンドとして ripgrep を使っています。Neovim 内の `<leader>fg` で呼ぶ検索は、ここで解説した `rg` と同じエンジンです。

## 3. fzf — ファジーファインダー

### fzf の動作原理

fzf は標準入力から受け取った行リストに対してファジーマッチ（曖昧一致）を適用し、選択されたものを標準出力に返します。これがすべてです。

```bash
# 基本パターン: 何かの出力 | fzf → 選択結果
echo -e "apple\nbanana\ncherry" | fzf
```

つまり fzf 自体はファイル検索ツールではなく、**汎用の選択インターフェース**です。何でもパイプで渡せます。

### zsh キーバインド（.zshrc で設定済み）

| キー | 操作 | 仕組み |
| --- | --- | --- |
| `Ctrl+R` | 履歴検索 | `history` の出力を fzf に渡す |
| `Ctrl+T` | ファイル検索 | `fd` の出力を fzf に渡し、選択結果をコマンドラインに挿入 |
| `Alt+C` | ディレクトリ移動 | `fd -t d` の出力を fzf に渡し、選択結果に `cd` |

### fzf をパイプで活用する

fzf の真価は他のコマンドとの組み合わせにあります。

```bash
# プロセスを選んで kill
ps aux | fzf | awk '{print $2}' | xargs kill

# Git ブランチを選んでチェックアウト
git branch -a | fzf | xargs git checkout

# Docker コンテナを選んで接続
docker ps | fzf | awk '{print $1}' | xargs -I {} docker exec -it {} bash

# ripgrep の結果からファイルを選んで nvim で開く
rg -l "pattern" | fzf | xargs nvim
```

### fzf の検索構文

fzf のファジーマッチにはいくつかの修飾子があります。

| 入力 | 意味 |
| --- | --- |
| `word` | `word` を含む行にファジーマッチ |
| `'exact` | `exact` を正確に含む行（前置 `'`） |
| `^prefix` | `prefix` で始まる行 |
| `suffix$` | `suffix` で終わる行 |
| `!exclude` | `exclude` を含まない行 |
| `a b` | `a` と `b` の両方を含む行（スペース区切りで AND） |
| `a \| b` | `a` または `b` を含む行（`\|` で OR） |

例: `^src .rs$ !test` は「`src` で始まり、`.rs` で終わり、`test` を含まない」行にマッチします。

## 4. zoxide — cd の代替

### zoxide の仕組み

zoxide は訪問したディレクトリの履歴をデータベース（`~/.local/share/zoxide/db.zo`）に記録し、頻度（frecency = frequency × recency）に基づいてスコアリングします。

```bash
# 従来の cd
cd ~/projects/web-app/backend/src

# zoxide
z backend
```

`z backend` で `~/projects/web-app/backend/src` に移動できるのは、過去にそのディレクトリを訪問した回数と新しさが高いためです。

### zoxide の基本操作

| コマンド | 操作 |
| --- | --- |
| `z keyword` | キーワードにマッチする最高スコアのディレクトリに移動 |
| `z keyword1 keyword2` | 複数キーワードの AND マッチ |
| `zi` | fzf を使ってインタラクティブに選択 |
| `z -` | 直前のディレクトリに戻る |

### zoxide と cd の使い分け

| 場面 | 使うコマンド |
| --- | --- |
| 初めて訪れるディレクトリ | `cd`（zoxide にまだ学習データがない） |
| よく行くディレクトリ | `z`（キーワードだけで到達） |
| パスの一部しか覚えていない | `zi`（fzf でインタラクティブに選ぶ） |
| 1つ上に移動 | `cd ..`（`z ..` は使えない） |

### なぜ cd がほぼ不要になるか

開発作業では同じディレクトリ群を何度も行き来します。zoxide はこのパターンを学習するため、使い込むほどキーワードの精度が上がります。1〜2週間使えば、プロジェクトのどこにでも 2〜3 文字で移動できるようになります。

## 5. eza — ls の代替

### 従来の ls との比較

```bash
# ls: 色がない（あっても限定的）、Git 状態がわからない、ツリー表示なし
ls -la

# eza: 色付き、アイコン付き、Git 状態表示、ツリー表示
eza -la --icons --git
```

### .zshrc で設定済みのエイリアス

```bash
alias ll="eza -la --icons"     # 詳細一覧
alias ls="eza --icons"         # 簡易一覧（ls を上書き）
alias lt="eza -la --icons --tree --level=2"  # ツリー表示（深さ 2）
```

### eza の便利なオプション

| オプション | 効果 |
| --- | --- |
| `--icons` | ファイルタイプに応じたアイコンを表示 |
| `--git` | Git の変更状態（M, N, I 等）を表示 |
| `--tree` | ツリー形式で表示 |
| `--level=N` | ツリーの深さを制限 |
| `--sort=modified` | 更新日時でソート |
| `--group-directories-first` | ディレクトリを先に表示 |

> **なぜ exa ではなく eza か**: `exa` はオリジナルの `ls` 代替ですが、2023 年にメンテナンスが停止しました。`eza` はそのコミュニティ fork で、活発にメンテナンスされています。

## ツール間の連携パターン

これらのツールは単独でも強力ですが、組み合わせるとさらに効果的です。

### fd + fzf: ファイルを探して開く

```bash
# Rust ファイルを fzf で選んで nvim で開く
fd -e rs | fzf | xargs nvim
```

### rg + fzf: テキストを探してファイルを開く

```bash
# パターンを含むファイルを fzf で選んで nvim で開く
rg -l "async fn" | fzf | xargs nvim
```

### fzf + zoxide: ディレクトリをインタラクティブに選ぶ

```bash
zi
```

`zi` は内部で fzf を使い、zoxide のデータベースからディレクトリをインタラクティブに選択します。

### eza + fd: プロジェクト構造の確認

```bash
# 特定ディレクトリのツリーを表示
eza --tree --level=3 --icons $(fd -t d "src" | head -1)
```

## 従来コマンドとの対応表

| 従来 | 代替 | 主な利点 |
| --- | --- | --- |
| `find` | `fd` | 速度、`.gitignore` 尊重、シンプルな構文 |
| `grep -r` | `rg` | 速度、`.gitignore` 尊重、バイナリ自動スキップ |
| `history \| grep` | `Ctrl+R`（fzf） | ファジーマッチ、インタラクティブ |
| `cd path/to/dir` | `z keyword` | キーワードだけで移動、学習型 |
| `ls -la` | `eza -la --icons` | 色、アイコン、Git 状態、ツリー |

従来コマンドは依然として使えます。代替ツールはユーザーの操作に特化しているだけで、シェルスクリプト内や CI では従来コマンドを使い続けても問題ありません。

## 次に読む文書

[[05 tmux 連携と起動速度最適化]] に進み、tmux によるターミナル多重化と zsh の起動速度チューニングを学びます。
