# tmux 連携と起動速度最適化

## この文書の責務

tmux によるターミナル多重化の基本操作と、zsh の起動速度をチューニングする手法を解説します。tmux と zsh は独立した技術ですが、「複数ペインで zsh を起動するたびに遅い」という問題が直結するため、1つの文書にまとめています。

## 前提

- [[03 zshrc 設定とプラグイン]] が完了していること
- `sudo apt install -y tmux` が実行済みであること

---

## Part A: tmux の基礎

### tmux とは何か

tmux（terminal multiplexer）は、1 つの端末ウィンドウの中に複数の仮想端末を作成するツールです。ウィンドウマネージャがデスクトップ上のウィンドウを管理するのと同じように、tmux は端末内の「ペイン」と「ウィンドウ」を管理します。

tmux の主な役割は 3 つです。

1. **多重化**: 1 つのターミナルで複数のシェルを同時に使えます。左にエディタ、右にビルド出力、下にログ監視といったレイアウトが可能です。
2. **デタッチ/アタッチ**: SSH 接続が切れてもセッションが生き続けます。再接続して `tmux attach` すれば作業途中の画面がそのまま戻ります。
3. **セッション管理**: プロジェクトごとにセッションを分けて、切り替えながら作業できます。

### 用語と階層構造

```text
tmux server
 └── session "dev"
      ├── window 0: "editor"     ← Neovim
      │    ├── pane 0 (上)
      │    └── pane 1 (下)
      └── window 1: "terminal"   ← シェル作業
           └── pane 0
```

| 用語 | 意味 |
| --- | --- |
| server | tmux のバックグラウンドプロセス。すべてのセッションを管理 |
| session | ウィンドウの集合。プロジェクト単位で作ると便利 |
| window | セッション内のタブのようなもの |
| pane | ウィンドウ内の分割領域。それぞれ独立したシェルが動く |

### prefix キー

tmux のキーバインドはすべて prefix キー（デフォルト: `Ctrl+b`）を先に押してから入力します。

```text
Ctrl+b → c    ← 新しいウィンドウを作成
```

### 基本操作

#### セッション操作

| キー / コマンド | 操作 |
| --- | --- |
| `tmux new -s dev` | "dev" という名前のセッションを作成 |
| `tmux ls` | セッション一覧 |
| `tmux attach -t dev` | "dev" セッションに接続 |
| `Ctrl+b d` | 現在のセッションからデタッチ |
| `Ctrl+b s` | セッション一覧をインタラクティブに選択 |
| `tmux kill-session -t dev` | "dev" セッションを終了 |

#### ウィンドウ操作

| キー | 操作 |
| --- | --- |
| `Ctrl+b c` | 新しいウィンドウを作成 |
| `Ctrl+b n` | 次のウィンドウに移動 |
| `Ctrl+b p` | 前のウィンドウに移動 |
| `Ctrl+b 0`〜`9` | 番号でウィンドウを選択 |
| `Ctrl+b ,` | ウィンドウ名を変更 |
| `Ctrl+b w` | ウィンドウ一覧をインタラクティブに選択 |

#### ペイン操作

| キー | 操作 |
| --- | --- |
| `Ctrl+b %` | 左右に分割 |
| `Ctrl+b "` | 上下に分割 |
| `Ctrl+b ←↑↓→` | ペイン間を移動 |
| `Ctrl+b z` | ペインを一時的に全画面化（トグル） |
| `Ctrl+b x` | ペインを閉じる（確認あり） |
| `Ctrl+b {` / `Ctrl+b }` | ペインの位置を入れ替え |

### 推奨 tmux.conf

`~/.tmux.conf` に以下を記述します。

```bash
# prefix を Ctrl+b → Ctrl+a に変更（Ctrl+a の方が押しやすい）
unbind C-b
set -g prefix C-a
bind C-a send-prefix

# ペイン分割を直感的なキーに
bind | split-window -h -c "#{pane_current_path}"
bind - split-window -v -c "#{pane_current_path}"
unbind '"'
unbind %

# ペイン移動を vim 風に
bind h select-pane -L
bind j select-pane -D
bind k select-pane -U
bind l select-pane -R

# マウス有効化
set -g mouse on

# 256 色対応
set -g default-terminal "tmux-256color"
set -ag terminal-overrides ",xterm-256color:RGB"

# ステータスバーを上に
set -g status-position top

# ウィンドウ番号を 1 から開始
set -g base-index 1
setw -g pane-base-index 1

# ウィンドウを閉じたときに番号を詰める
set -g renumber-windows on

# ESC キーの遅延を最小化（Neovim 用）
set -sg escape-time 10

# zsh をデフォルトシェルに
set -g default-shell /usr/bin/zsh
```

設定を反映するには:

```bash
tmux source-file ~/.tmux.conf
```

### tmux と Neovim の注意点

Neovim を tmux 内で使う場合、2 点の設定が重要です。

1. **`escape-time`**: デフォルト 500ms だと ESC キーの応答が遅くなります。上記設定の `set -sg escape-time 10` で解消します。
2. **True Color**: `terminal-overrides` で RGB を有効にしないと、Neovim のカラースキームが正しく表示されません。

---

## Part B: zsh 起動速度の最適化

### なぜ起動速度が重要か

tmux で新しいペインを開くたびに zsh が起動します。また `Ctrl+T` や `Ctrl+R` の fzf 連携もサブシェルを生成します。起動に 200ms かかる設定と 50ms かかる設定では、1 日の積み重ねで体感が大きく変わります。

**目標**: `time zsh -i -c exit` が **50ms 以下**であること。

### 起動時間の計測方法

```bash
# 方法 1: time コマンド
time zsh -i -c exit

# 方法 2: 10 回平均を取る
for i in $(seq 1 10); do
  (time zsh -i -c exit) 2>&1
done | grep real | awk '{print $2}'

# 方法 3: zsh のプロファイラ
# .zshrc の先頭に追記
zmodload zsh/zprof
# .zshrc の末尾に追記
zprof
```

方法 3 を使うと、どのプラグインや処理に何 ms かかっているかが分かります。

### zinit Turbo Mode の仕組みと効果

[[03 zshrc 設定とプラグイン]] で設定した `wait` オプションが Turbo Mode です。

```bash
# 通常読み込み: プロンプト表示前に読み込み → 起動が遅くなる
zinit light zsh-users/zsh-autosuggestions

# Turbo Mode: プロンプト表示後に非同期で読み込み → 起動に影響しない
zinit wait lucid light-mode for zsh-users/zsh-autosuggestions
```

Turbo Mode はプラグインの読み込みをプロンプト表示後（次のイベントループ）に遅延させます。ユーザーがプロンプトを見てコマンドを打ち始めるまでの数百 ms の間にバックグラウンドで読み込まれるため、体感上は「最初から読み込まれている」ように見えます。

### wait の数値による制御

```bash
zinit wait"0" lucid for ...   # 即座に遅延読み込み（デフォルト）
zinit wait"1" lucid for ...   # 1 秒後に読み込み
zinit wait"2" lucid for ...   # 2 秒後に読み込み
```

よく使うプラグインほど小さい数値にします。

| プラグイン | 推奨 wait | 理由 |
| --- | --- | --- |
| zsh-autosuggestions | 0 | 最初のキー入力時に必要 |
| zsh-syntax-highlighting | 即時ロード | custom widget 定義の後ろに置く必要があるため |
| zsh-completions | 0 | Tab 補完に必要 |
| zsh-history-substring-search | 即時ロード | Ctrl+P/N で使う widget を先に定義する必要があるため |

WSL でよく使われる Ubuntu 22.04 の zsh 5.8.1 では、`zsh-syntax-highlighting` を先に遅延ロードし、`history-substring-search` の widget を後から定義すると `unhandled ZLE widget` 警告が出ることがあります。この組み合わせでは、起動速度よりも**読み込み順の正しさ**を優先してください。

### compinit の最適化

`compinit` は zsh の補完システムを初期化する処理で、起動時間の大部分を占めることがあります。

```bash
# 最適化前: 毎回セキュリティチェック付きで初期化
autoload -Uz compinit && compinit

# 最適化後: 1 日 1 回だけセキュリティチェック
autoload -Uz compinit
if [[ -n ~/.zcompdump(#qN.mh+24) ]]; then
  compinit
else
  compinit -C   # キャッシュからロード（高速）
fi
```

`compinit -C` はダンプファイル（`~/.zcompdump`）をそのまま読み込むため高速です。`(#qN.mh+24)` は「24 時間以上古い場合」を意味する zsh glob qualifier です。

### 遅いプラグインの特定と対処

`zprof` で遅いプラグインを見つけたら:

1. **Turbo Mode にする**: `wait lucid` を付ける
2. **代替を探す**: oh-my-zsh プラグインなどは大きくて遅いことがある
3. **不要なら削除する**: 使っていないプラグインは遅延読み込みしても無駄

ただし例外があります。`zsh-syntax-highlighting` と `zsh-history-substring-search` のように ZLE widget の定義順序に依存するものは、無理に Turbo Mode にせず、正しい順序で即時ロードした方が安定します。

starship の警告が `Scanning current directory timed out` である場合は、プラグイン順序ではなく `scan_timeout` の問題です。特に WSL で `/mnt/c` 配下や `\\wsl$` 越しのディレクトリを開くと既定値 30ms を超えやすいため、`~/.config/starship.toml` に `scan_timeout = 2000` を設定して探索時間を伸ばします。

### 起動速度チェックリスト

- [ ] `time zsh -i -c exit` が 50ms 以下
- [ ] `zprof` で 10ms 以上かかる処理がないか確認
- [ ] 遅延可能なプラグインだけに `wait lucid` を付けている
- [ ] `compinit -C` でキャッシュを活用している
- [ ] 不要なプラグインを読み込んでいない
- [ ] `nvm` や `pyenv` など重い初期化を遅延させている

> **NVM の注意**: nvm（Node.js バージョンマネージャ）は初期化に 200〜500ms かかるのが一般的です。この文書の構成では uv（Python）と rustup（Rust）を使っており、どちらも起動時の初期化は PATH の追加だけなので影響は最小限です。

---

## 典型的な作業フロー

```bash
# 1. プロジェクト用セッションを作成
tmux new -s myproject

# 2. Neovim でコーディング
nvim .

# 3. Ctrl+a | で右にペインを分割 → ビルド実行
cargo build

# 4. Ctrl+a - で下にペインを分割 → テスト実行
cargo test

# 5. 作業を中断してデタッチ
# Ctrl+a d

# 6. 後日再開
tmux attach -t myproject
```

## 次に読む文書

zsh シリーズはこの文書で完結です。Neovim の設定に進む場合は [[00 Neovim ドキュメントガイド]] を参照してください。
