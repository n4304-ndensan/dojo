# zshrc 設定とプラグイン

## この文書の責務

`.zshrc` の完成版を構築します。プラグインの設定、補完システム、キーバインド、エイリアスを扱います。各設定項目について「何をしているか」だけでなく「なぜそうするか」を説明します。

## 前提

- [[02 zsh 基盤セットアップ]] が完了していること
- zsh がデフォルトシェルになっていること
- zinit と starship がインストール済みであること

## 完成版 .zshrc

以下が完成形です。各セクションの解説はこの後に続きます。

```bash
# ============================================================
# PATH
# ============================================================
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"

# ============================================================
# zinit
# ============================================================
source "${HOME}/.local/share/zinit/zinit.git/zinit.zsh"

# ============================================================
# プラグイン（Turbo Mode で遅延ロード）
# ============================================================
zinit ice wait lucid
zinit light zsh-users/zsh-autosuggestions

zinit ice wait lucid
zinit light zsh-users/zsh-completions

zinit light zsh-users/zsh-history-substring-search

# ============================================================
# 補完システム
# ============================================================
autoload -Uz compinit
compinit

zstyle ':completion:*' matcher-list 'm:{a-z}={A-Z}'
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
zstyle ':completion:*' menu select
zstyle ':completion:*:descriptions' format '%F{green}-- %d --%f'

# ============================================================
# ヒストリ
# ============================================================
HISTFILE=~/.zsh_history
HISTSIZE=50000
SAVEHIST=50000
setopt HIST_IGNORE_DUPS
setopt HIST_IGNORE_SPACE
setopt SHARE_HISTORY
setopt APPEND_HISTORY
setopt INC_APPEND_HISTORY

# ============================================================
# シェルオプション
# ============================================================
setopt AUTO_CD
setopt AUTO_PUSHD
setopt PUSHD_IGNORE_DUPS
setopt CORRECT

# ============================================================
# キーバインド
# ============================================================
bindkey -e
bindkey '^P' history-substring-search-up
bindkey '^N' history-substring-search-down

# ============================================================
# エイリアス
# ============================================================
alias v="nvim"
alias ll="eza -la --icons"
alias ls="eza --icons"
alias lt="eza -la --icons --tree --level=2"
alias gs="git status"
alias gc="git commit"
alias gp="git push"
alias gl="git log --oneline --graph -20"

# ============================================================
# fzf
# ============================================================
[ -f /usr/share/doc/fzf/examples/key-bindings.zsh ] && \
  source /usr/share/doc/fzf/examples/key-bindings.zsh
[ -f /usr/share/doc/fzf/examples/completion.zsh ] && \
  source /usr/share/doc/fzf/examples/completion.zsh

export FZF_DEFAULT_OPTS='--height 40% --layout=reverse --border'
export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git'
export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
export FZF_ALT_C_COMMAND='fd --type d --hidden --follow --exclude .git'

# ============================================================
# zoxide
# ============================================================
command -v zoxide >/dev/null 2>&1 && eval "$(zoxide init zsh)"

# ============================================================
# zsh-syntax-highlighting（custom widget 定義の後）
# ============================================================
zinit light zsh-users/zsh-syntax-highlighting

# ============================================================
# starship（最後に配置）
# ============================================================
eval "$(starship init zsh)"
```

## セクション別の解説

### PATH

```bash
export PATH="$HOME/.local/bin:$HOME/.cargo/bin:$PATH"
```

`~/.local/bin` には starship、uv でインストールしたツール（basedpyright, ruff）、fd のシンボリックリンクが入ります。`~/.cargo/bin` には rustup / cargo でインストールしたツール（rust-analyzer, rustfmt, cargo）が入ります。

PATH の設定は `.zshrc` の **先頭** に書きます。後続のプラグインやツールが PATH 上のバイナリに依存するためです。

### プラグイン

4つのプラグインを使います。すべて zinit の Turbo Mode（`wait lucid`）で遅延ロードします。

#### zsh-autosuggestions

```bash
zinit ice wait lucid
zinit light zsh-users/zsh-autosuggestions
```

入力中に、過去のコマンド履歴から候補をグレーのテキストで表示します。→ キーで候補を採用できます。

なぜ有用か: コマンドの再入力が大幅に減ります。特に長いコマンドや、ディレクトリパスを含むコマンドで効果が大きいです。fish が標準で持っている機能を zsh で再現するプラグインです。

#### zsh-syntax-highlighting

```bash
zinit light zsh-users/zsh-syntax-highlighting
```

入力中のコマンドをリアルタイムでハイライトします。存在するコマンドは緑、存在しないコマンドは赤で表示されます。

なぜ有用か: Enter を押す前にコマンドの正否がわかります。タイプミスに即座に気づけるため、「command not found」のエラーが減ります。

重要なのは**配置位置**です。`zsh-syntax-highlighting` は ZLE widget をフックして動作するため、`compinit`、`zsh-history-substring-search`、fzf のキーバインドなど、**他の widget 定義が終わった後**に読み込む必要があります。特に WSL でよく使われる Ubuntu 22.04 の zsh 5.8.1 では、この順序を守らないと `unhandled ZLE widget 'history-substring-search-up'` の警告が出ます。

#### zsh-completions

```bash
zinit ice wait lucid
zinit light zsh-users/zsh-completions
```

数百のコマンドに対する追加の補完定義を提供します。`docker`, `cargo`, `systemctl` 等、zsh 標準では不足する補完を補います。

#### zsh-history-substring-search

```bash
zinit light zsh-users/zsh-history-substring-search
```

入力中のテキストを部分一致で履歴検索します。`git` と入力して Ctrl+P を押すと、`git` を含む過去のコマンドが順番に表示されます。

zsh 標準の履歴検索（Ctrl+R）は前方一致です。部分一致の方が実用的な場面が多く、このプラグインで補います。

このプラグインは `history-substring-search-up` / `history-substring-search-down` という widget を定義します。したがって、これらに対する `bindkey` は **このプラグインの読み込み後** に行う必要があります。Turbo Mode で遅延ロードしたまま先に `bindkey` すると、widget 未定義のままキー割り当てだけが先行し、WSL の zsh 5.8 系では syntax-highlighting 側の警告につながります。

### 補完システム

```bash
autoload -Uz compinit
compinit
```

zsh の補完システム `compsys` を初期化します。`autoload -Uz` は「必要になるまで関数を読み込まない」宣言で、`compinit` がその関数を実際に実行します。

```bash
zstyle ':completion:*' matcher-list 'm:{a-z}={A-Z}'
```

小文字で入力しても大文字のファイル名にマッチさせます。`readme` と入力して Tab を押すと `README.md` が補完されます。

```bash
zstyle ':completion:*' list-colors "${(s.:.)LS_COLORS}"
```

補完候補に `LS_COLORS` に基づいた色を付けます。ディレクトリは青、実行ファイルは緑など、視覚的に区別しやすくなります。

```bash
zstyle ':completion:*' menu select
```

補完候補が複数ある場合にメニュー形式で表示し、矢印キーで選択できるようにします。

```bash
zstyle ':completion:*:descriptions' format '%F{green}-- %d --%f'
```

補完候補のグループにヘッダーを表示します。例えば `git` の補完で「commands」「aliases」「branches」がグループ分けされます。

### ヒストリ

```bash
HISTFILE=~/.zsh_history
HISTSIZE=50000
SAVEHIST=50000
```

履歴ファイルのパス、メモリ上の保持件数、ファイルへの保存件数を設定します。50,000 件あれば数ヶ月分のコマンドが保持されます。

```bash
setopt HIST_IGNORE_DUPS     # 直前と同じコマンドは記録しない
setopt HIST_IGNORE_SPACE    # スペースで始まるコマンドは記録しない
setopt SHARE_HISTORY        # 複数の zsh セッション間で履歴を共有する
setopt APPEND_HISTORY       # 履歴ファイルを上書きではなく追記する
setopt INC_APPEND_HISTORY   # コマンド実行後すぐに履歴ファイルに追記する
```

`HIST_IGNORE_SPACE` は意図的に履歴に残したくないコマンド（トークンを含む一時的なコマンドなど）で使えます。先頭にスペースを付ければ記録されません。

`SHARE_HISTORY` と `INC_APPEND_HISTORY` の組み合わせにより、tmux で複数ペインを開いている場合でも、あるペインで実行したコマンドが他のペインの履歴に即座に反映されます。

### シェルオプション

```bash
setopt AUTO_CD            # ディレクトリ名だけ入力すると cd する
setopt AUTO_PUSHD         # cd するたびにディレクトリスタックに積む
setopt PUSHD_IGNORE_DUPS  # スタックの重複を除外する
setopt CORRECT            # コマンドのタイプミスを修正提案する
```

`AUTO_CD` は `~/projects` と入力するだけで `cd ~/projects` と同じ動作をします。`AUTO_PUSHD` と組み合わせると、`cd -` で直前のディレクトリに戻れるだけでなく、`cd -2`, `cd -3` と番号付きで過去のディレクトリに戻れます。

### キーバインド

```bash
bindkey -e
```

Emacs キーバインドを使います。bash のデフォルトと同じです。Ctrl+A（行頭）、Ctrl+E（行末）、Ctrl+W（単語削除）などが使えます。Vim キーバインド（`bindkey -v`）もありますが、シェルの対話操作では Emacs バインドの方が直感的です。

```bash
bindkey '^P' history-substring-search-up
bindkey '^N' history-substring-search-down
```

Ctrl+P / Ctrl+N で部分一致の履歴検索を上下移動します。zsh-history-substring-search プラグインの機能です。

ここでの前提は、`zsh-history-substring-search` がすでに読み込まれていることです。今回の構成ではこの条件を満たすため、プラグイン自体は遅延ロードせず、その後に `bindkey` を実行しています。

### エイリアス

```bash
alias v="nvim"
alias ll="eza -la --icons"
alias ls="eza --icons"
alias lt="eza -la --icons --tree --level=2"
```

エイリアスは最小限に留めます。多すぎると何が本当のコマンドで何がエイリアスかわからなくなります。

方針:

- **頻繁に使うもの**だけをエイリアスにする
- **元のコマンド名を上書き**して良いのは、完全上位互換のもの（`eza` は `ls` の上位互換）
- **1文字エイリアス**は `v`（nvim）程度に留める。多用すると衝突する

### fzf 連携

```bash
[ -f /usr/share/doc/fzf/examples/key-bindings.zsh ] && \
  source /usr/share/doc/fzf/examples/key-bindings.zsh
[ -f /usr/share/doc/fzf/examples/completion.zsh ] && \
  source /usr/share/doc/fzf/examples/completion.zsh
```

Ubuntu の `apt install fzf` では、キーバインドと補完スクリプトが `/usr/share/doc/fzf/examples/` に配置されます。これを source することで以下のキーバインドが有効になります。

| キー | 操作 |
| --- | --- |
| `Ctrl+R` | コマンド履歴をファジー検索（fzf 版。デフォルトの `Ctrl+R` を置き換える） |
| `Ctrl+T` | カレントディレクトリ以下のファイルをファジー検索して入力に挿入 |
| `Alt+C` | カレントディレクトリ以下のディレクトリをファジー検索して `cd` |

```bash
export FZF_DEFAULT_OPTS='--height 40% --layout=reverse --border'
```

fzf の表示をターミナルの下部 40% に表示し、結果を上から並べ、枠線を表示します。全画面を占有しないのでコンテキストを失いません。

```bash
export FZF_DEFAULT_COMMAND='fd --type f --hidden --follow --exclude .git'
export FZF_CTRL_T_COMMAND="$FZF_DEFAULT_COMMAND"
```

fzf のファイル検索バックエンドを `find` から `fd` に置き換えます。fd は `.gitignore` を自動的に尊重し、`find` より桁違いに速いです。`--hidden` でドットファイルも含め、`--exclude .git` で `.git` ディレクトリを除外します。

```bash
export FZF_ALT_C_COMMAND='fd --type d --hidden --follow --exclude .git'
```

`Alt+C` のディレクトリ検索でも同様に fd を使います。

### zoxide 連携

```bash
command -v zoxide >/dev/null 2>&1 && eval "$(zoxide init zsh)"
```

zoxide がインストールされている場合だけシェル統合を初期化します。これにより `z` コマンドが使えるようになります。WSL の初期構築中など、まだ `zoxide` を入れていない段階でもシェル起動時に `command not found` を出さないため、この形にしておく方が安全です。詳細は [[04 CLI 操作環境の構築]] で解説します。

### zsh-syntax-highlighting の配置位置

```bash
zinit light zsh-users/zsh-syntax-highlighting
```

`zsh-syntax-highlighting` は `.zshrc` のかなり後ろに置きます。少なくとも以下より後ろです。

- `compinit`
- `zsh-history-substring-search`
- `bindkey '^P' ...` / `bindkey '^N' ...`
- fzf の key-bindings 読み込み

これは性能上の都合ではなく、**正しくすべての widget を認識させるための要件**です。

### starship の配置位置

```bash
eval "$(starship init zsh)"
```

starship の初期化は `.zshrc` の **末尾** に置きます。starship はプロンプトを書き換えるため、他の設定が先に完了している必要があります。

## 設定の読み込み順序が重要な理由

`.zshrc` の中で設定の順序を間違えると、意図した動作にならないことがあります。

```text
1. PATH           ← ツールが見つかる前提を確保
2. zinit 初期化   ← プラグインマネージャーを使えるようにする
3. 軽量プラグイン  ← autosuggestions / completions
4. history-substring-search ← widget を先に定義する
5. 補完           ← compsys を初期化
6. ヒストリ       ← 履歴関連オプション
7. シェルオプション
8. キーバインド   ← history-substring-search の widget に bind する
9. エイリアス
10. fzf / zoxide  ← 外部ツール連携と widget 定義
11. syntax-highlighting ← 最後に widget 群を認識させる
12. starship      ← プロンプト（最後）
```

## 動作確認

`.zshrc` を保存した後、zsh を再起動します。

```bash
exec zsh
```

確認手順:

1. プロンプトが starship で表示されること
2. `git` と入力してグレーのサジェスチョンが表示されること（autosuggestions）
3. 存在しないコマンド名が赤くハイライトされること（syntax-highlighting）
4. `Ctrl+R` で fzf の履歴検索が開くこと
5. Tab キーでメニュー形式の補完が表示されること

## 次に読む文書

[[04 CLI 操作環境の構築]] に進み、fzf・zoxide・ripgrep・fd・eza の具体的な使い方とカスタマイズを学びます。
